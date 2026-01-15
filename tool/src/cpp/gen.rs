use super::header::Header;
use super::Cpp2Formatter;
use crate::c::Header as C2Header;
use crate::c::ItemGenContext as CItemGenContext;
use crate::ErrorStore;
use askama::Template;
use diplomat_core::hir::CallbackInstantiationFunctionality;
use diplomat_core::hir::OpaqueId;
use diplomat_core::hir::Slice;
use diplomat_core::hir::{
    self, MaybeOwn, Mutability, OpaqueOwner, ReturnType, SelfType, StructPathLike, SuccessType,
    SymbolId, TyPosition, Type, TypeDef,
};
use std::borrow::Cow;

use crate::c::CAPI_NAMESPACE;
use crate::filters;

/// A type name with a corresponding variable name, such as a struct field or a function parameter.
pub struct NamedType<'a> {
    var_name: Cow<'a, str>,
    type_name: Cow<'a, str>,
}

/// We generate a pair of methods for writeables, one which returns a std::string
/// and one which operates on a WriteTrait
struct MethodWriteableInfo<'a> {
    /// The method name. Usually `{}_write()`, but could potentially
    /// be made customizeable
    method_name: Cow<'a, str>,
    /// The return type for the method without the std::string
    return_ty: Cow<'a, str>,
    c_to_cpp_return_expression: Option<Cow<'a, str>>,
}

/// Everything needed for rendering a method.
pub struct MethodInfo<'a> {
    /// HIR of the method being rendered
    method: &'a hir::Method,
    /// The C++ return type
    return_ty: Cow<'a, str>,
    /// The C++ method name
    method_name: Cow<'a, str>,
    /// The C method name
    abi_name: String,
    /// Qualifiers for the function that come before the declaration (like "static")
    pre_qualifiers: Vec<Cow<'a, str>>,
    /// Qualifiers for the function that come after the declaration (like "const")
    post_qualifiers: Vec<Cow<'a, str>>,
    /// Type declarations for the C++ parameters
    param_decls: Vec<NamedType<'a>>,
    /// Parameter validations, such as string checks
    param_validations: Vec<String>,
    /// Conversion code from C++ to C, used to fill out cpp_to_c_params before a call. Used for converting clones of structs to references.
    param_pre_conversions: Vec<String>,
    /// C++ conversion code for each parameter of the C function
    cpp_to_c_params: Vec<Cow<'a, str>>,
    /// Conversion code of params from C to C++, grabbing the results of cpp_to_c_params and converting them into something C++ friendly. Used for converting references to clones of structs.
    param_post_conversions: Vec<String>,
    /// If the function has a return value, the C++ code for the conversion. Assumes that
    /// the C function return value is saved to a variable named `result` or that the
    /// DiplomatWrite, if present, is saved to a variable named `output`.
    c_to_cpp_return_expression: Option<Cow<'a, str>>,

    /// If the method returns a writeable, the info for that
    writeable_info: Option<MethodWriteableInfo<'a>>,
    docs: String,
    deprecated: Option<&'a str>,
}

/// An expression with a corresponding variable name, such as a struct field or a function parameter.
struct NamedExpression<'a> {
    var_name: Cow<'a, str>,
    expression: Cow<'a, str>,
}

#[derive(Template)]
#[template(path = "cpp/free_functions.h.jinja", escape = "none")]
/// Header for the implementation of a block of functions.
pub(crate) struct FuncImplTemplate<'a> {
    pub namespace: Option<String>,
    pub methods: Vec<MethodInfo<'a>>,
    pub c_header: C2Header,
    pub fmt: &'a Cpp2Formatter<'a>,
}

/// Context for generating a particular type's header
pub(crate) struct ItemGenContext<'ccx, 'tcx, 'header> {
    pub formatter: &'ccx Cpp2Formatter<'tcx>,
    #[allow(dead_code)] // Currently unused but could be in the future
    pub config: &'ccx super::CppConfig,
    pub errors: &'ccx ErrorStore<'tcx, String>,
    pub c: CItemGenContext<'ccx, 'tcx, 'header>,
    pub impl_header: &'header mut Header<'ccx>,
    pub decl_header: &'header mut Header<'ccx>,
    /// Are we currently generating struct fields?
    pub generating_struct_fields: bool,
}

impl<'ccx, 'tcx: 'ccx> ItemGenContext<'ccx, 'tcx, '_> {
    /// Adds an enum definition to the current decl and impl headers.
    ///
    /// The enum is defined in C++ using a `class` with a single private field that is the
    /// C enum type. This enables us to add methods to the enum and generally make the enum
    /// behave more like an upgraded C++ type. We don't use `enum class` because methods
    /// cannot be added to it.
    pub fn gen_enum_def(&mut self, id: hir::EnumId) {
        let ty = self.c.tcx.resolve_enum(id);
        let type_name = self.formatter.fmt_type_name(id.into());
        let type_name_unnamespaced = self.formatter.fmt_type_name_unnamespaced(id.into());
        let ctype = self.formatter.fmt_c_type_name(id.into());
        let c_header = self.c.gen_enum_def(id);
        let c_impl_header = self.c.gen_impl(id.into());

        let _guard = self
            .errors
            .set_context_ty(self.c.tcx.fmt_symbol_name_diagnostics(id.into()));

        let methods = ty
            .methods
            .iter()
            .flat_map(|method| self.gen_method_info(id.into(), method))
            .collect::<Vec<_>>();

        let mut found_default: Option<&hir::EnumVariant> = None;
        let mut found_zero = None;

        // Not all enums have a zero-variant; zero-initializing those is a mistake and will
        // lead to aborts in the conversion code. To allow default-initialization, we generate *some*
        // default ctor. It is, in order: the explicit default variant, OR the variant with 0 discriminant,
        // OR the first variant.
        for v in ty.variants.iter() {
            if v.attrs.default {
                if let Some(existing) = found_default {
                    self.errors.push_error(format!(
                        "Found multiple default variants for enum: {} and {}",
                        existing.name, v.name
                    ))
                }
                found_default = Some(v)
            }
            if v.discriminant == 0 {
                found_zero = Some(v)
            }
        }

        let default_variant = found_default
            .or(found_zero)
            .unwrap_or(ty.variants.first().unwrap());

        let default_variant = self.formatter.fmt_enum_variant(default_variant);
        #[derive(Template)]
        #[template(path = "cpp/enum_decl.h.jinja", escape = "none")]
        struct DeclTemplate<'a> {
            ty: &'a hir::EnumDef,
            fmt: &'a Cpp2Formatter<'a>,
            type_name: &'a str,
            ctype: &'a str,
            methods: &'a [MethodInfo<'a>],
            namespace: Option<&'a str>,
            type_name_unnamespaced: &'a str,
            c_header: C2Header,
            docs: &'a str,
            deprecated: Option<&'a str>,
            default_variant: Cow<'a, str>,
        }

        DeclTemplate {
            ty,
            fmt: self.formatter,
            type_name: &type_name,
            ctype: &ctype,
            methods: methods.as_slice(),
            namespace: ty.attrs.namespace.as_deref(),
            type_name_unnamespaced: &type_name_unnamespaced,
            c_header,
            docs: &self.formatter.fmt_docs(&ty.docs, &ty.attrs),
            deprecated: ty.attrs.deprecated.as_deref(),
            default_variant,
        }
        .render_into(self.decl_header)
        .unwrap();

        #[derive(Template)]
        #[template(path = "cpp/enum_impl.h.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            ty: &'a hir::EnumDef,
            fmt: &'a Cpp2Formatter<'a>,
            type_name: &'a str,
            ctype: &'a str,
            methods: &'a [MethodInfo<'a>],
            namespace: Option<&'a str>,
            c_header: C2Header,
        }

        ImplTemplate {
            ty,
            fmt: self.formatter,
            type_name: &type_name,
            ctype: &ctype,
            methods: methods.as_slice(),
            namespace: ty.attrs.namespace.as_deref(),
            c_header: c_impl_header,
        }
        .render_into(self.impl_header)
        .unwrap();
    }

    pub fn gen_opaque_def(&mut self, id: OpaqueId, def_block: String) {
        let type_name = self.formatter.fmt_type_name(id.into());
        let type_name_unnamespaced = self.formatter.fmt_type_name_unnamespaced(id.into());
        let ctype = self.formatter.fmt_c_type_name(id.into());
        let ty = self.c.tcx.resolve_opaque(id);
        let dtor_name = self
            .formatter
            .namespace_c_name(id.into(), ty.dtor_abi_name.as_str());

        let c_header = self.c.gen_opaque_def(id);
        let c_impl_header = self.c.gen_impl(id.into());

        let methods = ty
            .methods
            .iter()
            .flat_map(|method| self.gen_method_info(id.into(), method))
            .collect::<Vec<_>>();

        #[derive(Template)]
        #[template(path = "cpp/opaque_decl.h.jinja", escape = "none")]
        struct DeclTemplate<'a> {
            // ty: &'a hir::OpaqueDef,
            fmt: &'a Cpp2Formatter<'a>,
            type_name: &'a str,
            ctype: &'a str,
            methods: &'a [MethodInfo<'a>],
            namespace: Option<&'a str>,
            type_name_unnamespaced: &'a str,
            c_header: C2Header,
            docs: &'a str,
            deprecated: Option<&'a str>,
            def_block: String,
        }

        DeclTemplate {
            // ty,
            fmt: self.formatter,
            type_name: &type_name,
            ctype: &ctype,
            methods: methods.as_slice(),
            namespace: ty.attrs.namespace.as_deref(),
            type_name_unnamespaced: &type_name_unnamespaced,
            c_header,
            docs: &self.formatter.fmt_docs(&ty.docs, &ty.attrs),
            deprecated: ty.attrs.deprecated.as_deref(),
            def_block,
        }
        .render_into(self.decl_header)
        .unwrap();

        #[derive(Template)]
        #[template(path = "cpp/opaque_impl.h.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            // ty: &'a hir::OpaqueDef,
            fmt: &'a Cpp2Formatter<'a>,
            type_name: &'a str,
            ctype: &'a str,
            dtor_name: String,
            methods: &'a [MethodInfo<'a>],
            namespace: Option<&'a str>,
            c_header: C2Header,
        }

        ImplTemplate {
            // ty,
            fmt: self.formatter,
            type_name: &type_name,
            ctype: &ctype,
            dtor_name,
            methods: methods.as_slice(),
            namespace: ty.attrs.namespace.as_deref(),
            c_header: c_impl_header,
        }
        .render_into(self.impl_header)
        .unwrap();
    }

    pub fn gen_struct_def<P: TyPosition + 'ccx>(&mut self, id: P::StructId) {
        let def = P::resolve_struct(self.c.tcx, id);
        let type_name = self.formatter.fmt_type_name(id.into());
        let type_name_unnamespaced = self.formatter.fmt_type_name_unnamespaced(id.into());
        let ctype = self.formatter.fmt_c_type_name(id.into());

        let namespace = def.attrs.namespace.clone();

        let c_header = self.c.gen_struct_def::<P>(id);
        let c_impl_header = self.c.gen_impl(id.into());

        self.generating_struct_fields = true;
        let field_decls = def
            .fields
            .iter()
            .map(|field| self.gen_ty_decl(&field.ty, field.name.as_str()))
            .collect::<Vec<_>>();
        self.generating_struct_fields = false;

        let cpp_to_c_fields = def
            .fields
            .iter()
            .map(|field| self.gen_cpp_to_c_for_field("", field, namespace.clone()))
            .collect::<Vec<_>>();

        let c_to_cpp_fields = def
            .fields
            .iter()
            .map(|field| self.gen_c_to_cpp_for_field("c_struct.", field))
            .collect::<Vec<_>>();

        let methods = def
            .methods
            .iter()
            .flat_map(|method| self.gen_method_info(SymbolId::TypeId(id.into()), method))
            .collect::<Vec<_>>();

        #[derive(Template)]
        #[template(path = "cpp/struct_decl.h.jinja", escape = "none")]
        struct DeclTemplate<'a> {
            // ty: &'a hir::OpaqueDef,
            fmt: &'a Cpp2Formatter<'a>,
            type_name: &'a str,
            ctype: &'a str,
            fields: &'a [NamedType<'a>],
            methods: &'a [MethodInfo<'a>],
            namespace: Option<&'a str>,
            type_name_unnamespaced: &'a str,
            c_header: C2Header,
            is_sliceable: bool,
            docs: &'a str,
            deprecated: Option<&'a str>,
        }

        DeclTemplate {
            // ty,
            fmt: self.formatter,
            type_name: &type_name,
            ctype: &ctype,
            fields: field_decls.as_slice(),
            methods: methods.as_slice(),
            namespace: namespace.as_deref(),
            type_name_unnamespaced: &type_name_unnamespaced,
            c_header,
            is_sliceable: def.attrs.abi_compatible,
            docs: &self.formatter.fmt_docs(&def.docs, &def.attrs),
            deprecated: def.attrs.deprecated.as_deref(),
        }
        .render_into(self.decl_header)
        .unwrap();

        #[derive(Template)]
        #[template(path = "cpp/struct_impl.h.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            // ty: &'a hir::OpaqueDef,
            fmt: &'a Cpp2Formatter<'a>,
            type_name: &'a str,
            ctype: &'a str,
            cpp_to_c_fields: &'a [NamedExpression<'a>],
            c_to_cpp_fields: &'a [NamedExpression<'a>],
            methods: &'a [MethodInfo<'a>],
            namespace: Option<&'a str>,
            c_header: C2Header,
        }

        ImplTemplate {
            // ty,
            fmt: self.formatter,
            type_name: &type_name,
            ctype: &ctype,
            cpp_to_c_fields: cpp_to_c_fields.as_slice(),
            c_to_cpp_fields: c_to_cpp_fields.as_slice(),
            methods: methods.as_slice(),
            namespace: def.attrs.namespace.as_deref(),
            c_header: c_impl_header,
        }
        .render_into(self.impl_header)
        .unwrap();
    }

    pub fn gen_method_info(
        &mut self,
        id: SymbolId,
        method: &'ccx hir::Method,
    ) -> Option<MethodInfo<'ccx>> {
        if method.attrs.disable {
            return None;
        }
        let lib_name_ns_prefix = &self.formatter.lib_name_ns_prefix;
        let _guard = self
            .errors
            .set_context_method(method.name.to_string().into());
        let method_name = self.formatter.fmt_method_name(method);
        let abi_name = self
            .formatter
            .namespace_c_name(id, method.abi_name.as_str());
        let mut param_decls = Vec::new();
        let mut cpp_to_c_params = Vec::new();

        let mut param_pre_conversions = Vec::new();
        let mut param_post_conversions = Vec::new();

        if let Some(param_self) = method.param_self.as_ref() {
            // Convert the self parameter as normal:
            let conversion = self.gen_cpp_to_c_self(&param_self.ty);
            // If we happen to be a reference to a struct (and we can't just do a reinterpret_cast on the pointer),
            // Then we need to add some pre- and post- function call conversions to:
            // 1. Create `thisDiplomatRefClone` as the converted FFI friendly struct.
            // 2. Pass in the reference to `thisDiplomatRefClone`
            // 3. Assign `*this` to the value of `thisDiplomatRefClone`
            let conversion = if let hir::ParamSelf {
                ty: SelfType::Struct(ref s),
                ..
            } = param_self
            {
                let attrs = &s.resolve(self.c.tcx).attrs;
                if !s.owner.is_owned() && !attrs.abi_compatible {
                    param_pre_conversions
                        .push(format!("auto thisDiplomatRefClone = {conversion};"));

                    if s.owner.mutability().is_mutable() {
                        param_post_conversions.push(format!(
                            "*this = {}::FromFFI(thisDiplomatRefClone);",
                            self.formatter.fmt_symbol_name(s.id().into())
                        ));
                    }
                    "&thisDiplomatRefClone".to_string().into()
                } else {
                    conversion
                }
            } else {
                conversion
            };

            cpp_to_c_params.push(conversion);
        }

        let mut param_validations = Vec::new();
        let mut returns_utf8_err = false;

        let namespace = match id {
            SymbolId::FunctionId(f) => self.c.tcx.resolve_function(f).attrs.namespace.clone(),
            SymbolId::TypeId(ty) => self.c.tcx.resolve_type(ty).attrs().namespace.clone(),
            _ => panic!("Unsupported SymbolId: {id:?}"),
        };

        for param in method.params.iter() {
            let decls = self.gen_ty_decl(&param.ty, param.name.as_str());
            let param_name = decls.var_name.clone();
            param_decls.push(decls);
            if matches!(
                param.ty,
                Type::Slice(hir::Slice::Str(_, hir::StringEncoding::Utf8))
            ) {
                param_validations.push(format!(
                    "if (!{lib_name_ns_prefix}diplomat::capi::diplomat_is_str({param_name}.data(), {param_name}.size())) {{\n  return {lib_name_ns_prefix}diplomat::Err<{lib_name_ns_prefix}diplomat::Utf8Error>();\n}}",
                ));
                returns_utf8_err = true;
            }

            let conversion = self.gen_cpp_to_c_for_type(
                &param.ty,
                param_name,
                Some(method.abi_name.to_string()),
                namespace.clone(),
            );
            // If we happen to be a reference to a struct (and we can't just do a reinterpret_cast on the pointer),
            // Then we need to add some pre- and post- function call conversions to:
            // 1. Create `varNameDiplomatRefClone` as the converted FFI friendly struct.
            // 2. Pass in the reference to `varNameDiplomatRefClone`
            // 3. Assign `varName` to the value of `varNameDiplomatRefClone`
            let conversion = if let hir::Param {
                ty: hir::Type::Struct(ref s),
                ..
            } = param
            {
                let attrs = &s.resolve(self.c.tcx).attrs;
                if !s.owner.is_owned() && !attrs.abi_compatible {
                    param_pre_conversions.push(format!(
                        "auto {}DiplomatRefClone = {};",
                        param.name, conversion
                    ));

                    if s.owner.mutability().is_mutable() {
                        param_post_conversions.push(format!(
                            "{} = {}::FromFFI({}DiplomatRefClone);",
                            param.name,
                            self.formatter.fmt_type_name(s.id()),
                            param.name
                        ));
                    }
                    format!("&{}DiplomatRefClone", param.name).into()
                } else {
                    conversion
                }
            } else {
                conversion
            };

            cpp_to_c_params.push(conversion);
        }

        /// The UTF8 errors are added in by the C++ backend when converting from C++
        /// types. We wrap them in another layer of diplomat::result.
        fn wrap_return_ty_and_expr_for_utf8(
            return_ty: &mut Cow<str>,
            c_to_cpp_return_expression: &mut Option<Cow<str>>,
            fmt: &Cpp2Formatter,
        ) {
            let lib_name_ns_prefix = &fmt.lib_name_ns_prefix;
            if let Some(return_expr) = c_to_cpp_return_expression {
                *c_to_cpp_return_expression = Some(
                    format!("{lib_name_ns_prefix}diplomat::Ok<{return_ty}>({return_expr})").into(),
                );
                *return_ty = format!(
                        "{lib_name_ns_prefix}diplomat::result<{return_ty}, {lib_name_ns_prefix}diplomat::Utf8Error>"
                    )
                    .into();
            } else {
                *c_to_cpp_return_expression =
                    Some(format!("{lib_name_ns_prefix}diplomat::Ok<std::monostate>()").into());
                *return_ty = format!(
                    "{lib_name_ns_prefix}diplomat::result<std::monostate, {lib_name_ns_prefix}diplomat::Utf8Error>"
                )
                .into();
            }
        }

        let mut return_ty = self.gen_cpp_return_type_name(&method.output, false);

        let mut c_to_cpp_return_expression =
            self.gen_c_to_cpp_for_return_type(&method.output, "result".into(), false);

        if returns_utf8_err {
            wrap_return_ty_and_expr_for_utf8(
                &mut return_ty,
                &mut c_to_cpp_return_expression,
                self.formatter,
            )
        };

        // If the return expression is a std::move, unwrap that, because the linter doesn't like it
        c_to_cpp_return_expression = c_to_cpp_return_expression.map(|expr| {
            if expr.starts_with("std::move") {
                expr["std::move(".len()..(expr.len() - 1)].to_owned().into()
            } else {
                expr
            }
        });

        // Writeable methods generate a `std::string foo()` and a
        // `template<typename W> void foo_write(W& writeable)` where `W` is a `WriteTrait`
        // implementor. The generic method needs its own return type and conversion code.
        let mut writeable_info = None;
        if method.output.is_write() {
            cpp_to_c_params.push("&write".into());
            let mut return_ty = self.gen_cpp_return_type_name(&method.output, true);

            let mut c_to_cpp_return_expression =
                self.gen_c_to_cpp_for_return_type(&method.output, "result".into(), true);
            if returns_utf8_err {
                wrap_return_ty_and_expr_for_utf8(
                    &mut return_ty,
                    &mut c_to_cpp_return_expression,
                    self.formatter,
                )
            };
            writeable_info = Some(MethodWriteableInfo {
                method_name: format!("{method_name}_write").into(),
                return_ty,
                c_to_cpp_return_expression,
            });
        }

        let pre_qualifiers =
            if method.param_self.is_none() && !matches!(id, SymbolId::FunctionId(..)) {
                vec!["static".into()]
            } else {
                vec![]
            };

        let post_qualifiers = match &method.param_self {
            Some(param_self)
                if param_self.ty.is_immutably_borrowed() || param_self.ty.is_consuming() =>
            {
                vec!["const".into()]
            }
            Some(_) => vec![],
            None => vec![],
        };

        Some(MethodInfo::<'ccx> {
            method,
            return_ty,
            method_name,
            abi_name,
            pre_qualifiers,
            post_qualifiers,
            param_decls,
            param_pre_conversions,
            param_validations,
            param_post_conversions,
            cpp_to_c_params,
            c_to_cpp_return_expression,
            writeable_info,
            docs: self.formatter.fmt_docs(&method.docs, &method.attrs),
            deprecated: method.attrs.deprecated.as_deref(),
        })
    }

    /// Generates C++ code for referencing a particular type with a given name.
    pub(super) fn gen_ty_decl<'a, P: TyPosition>(
        &mut self,
        ty: &Type<P>,
        var_name: &'a str,
    ) -> NamedType<'a>
    where
        'ccx: 'a,
    {
        let var_name = self.formatter.fmt_param_name(var_name);
        let type_name = self.gen_type_name(ty);

        NamedType {
            var_name,
            type_name,
        }
    }

    /// Generates C++ code for referencing a particular type.
    ///
    /// This function adds the necessary type imports to the decl and impl files.
    pub(crate) fn gen_type_name<P: TyPosition>(&mut self, ty: &Type<P>) -> Cow<'ccx, str> {
        let lib_name_ns_prefix = &self.formatter.lib_name_ns_prefix;
        match *ty {
            Type::Primitive(prim) => self.formatter.fmt_primitive_as_c(prim),
            Type::Opaque(ref op) => {
                let op_id = op.tcx_id.into();
                let type_name = self.formatter.fmt_type_name(op_id);
                let type_name_unnamespaced = self.formatter.fmt_type_name_unnamespaced(op_id);
                let def = self.c.tcx.resolve_type(op_id);

                if def.attrs().disable {
                    self.errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }
                let mutability = op.owner.mutability().unwrap_or(hir::Mutability::Mutable);
                let ret = match (op.owner.is_owned(), op.is_optional()) {
                    // unique_ptr is nullable
                    (true, _) => self.formatter.fmt_owned(&type_name),
                    (false, true) => self.formatter.fmt_optional_borrowed(&type_name, mutability),
                    (false, false) => self.formatter.fmt_borrowed(&type_name, mutability),
                };
                let ret = ret.into_owned().into();

                // We don't append a header for this, since we already have a forward.
                // Note that we also need a forward for the C type in case of structs. The forward handling manages this.
                self.decl_header
                    .append_forward(def, &type_name_unnamespaced);
                self.impl_header
                    .includes
                    .insert(self.formatter.fmt_impl_header_path(op_id.into()));
                ret
            }
            Type::Struct(ref st) => self.gen_struct_name::<P>(st),
            Type::Enum(ref e) => {
                let id = e.tcx_id.into();
                let type_name = self.formatter.fmt_type_name(id);
                let type_name_unnamespaced = self.formatter.fmt_type_name_unnamespaced(id);
                let def = self.c.tcx.resolve_type(id);
                if def.attrs().disable {
                    self.errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }

                self.decl_header
                    .append_forward(def, &type_name_unnamespaced);
                if self.generating_struct_fields {
                    self.decl_header
                        .includes
                        .insert(self.formatter.fmt_decl_header_path(id.into()));
                }
                self.impl_header
                    .includes
                    .insert(self.formatter.fmt_impl_header_path(id.into()));
                type_name
            }
            Type::Slice(hir::Slice::Str(_, encoding)) => self.formatter.fmt_borrowed_str(encoding),
            Type::Slice(hir::Slice::Primitive(b, p)) => {
                let ret = self.formatter.fmt_primitive_as_c(p);
                let ret = self.formatter.fmt_borrowed_slice(&ret, b.mutability());
                ret.into_owned().into()
            }
            Type::Slice(hir::Slice::Strs(encoding)) => format!(
                "{lib_name_ns_prefix}diplomat::span<const {}>",
                self.formatter.fmt_borrowed_str_in_slice(encoding)
            )
            .into(),
            Type::Slice(hir::Slice::Struct(b, ref st_ty)) => {
                let st_name = self.gen_struct_name::<P>(st_ty);
                let ret = self.formatter.fmt_borrowed_slice(&st_name, b.mutability());
                ret.into_owned().into()
            }
            Type::Callback(ref cb) => format!("std::function<{}>", self.gen_fn_sig(cb)).into(),
            Type::DiplomatOption(ref inner) => {
                format!("std::optional<{}>", self.gen_type_name(inner)).into()
            }
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    fn gen_struct_name<P: TyPosition>(&mut self, st: &P::StructPath) -> Cow<'ccx, str> {
        let id = st.id();
        let type_name = self.formatter.fmt_type_name(id);

        let type_name_unnamespaced = self.formatter.fmt_type_name_unnamespaced(id);
        let def = self.c.tcx.resolve_type(id);
        if def.attrs().disable {
            self.errors
                .push_error(format!("Found usage of disabled type {type_name}"))
        }

        self.decl_header
            .append_forward(def, &type_name_unnamespaced);
        if self.generating_struct_fields {
            self.decl_header
                .includes
                .insert(self.formatter.fmt_decl_header_path(id.into()));
        }
        self.impl_header
            .includes
            .insert(self.formatter.fmt_impl_header_path(id.into()));
        if let MaybeOwn::Borrow(borrow) = st.owner() {
            let mutability = borrow.mutability;
            match (borrow.is_owned(), false) {
                // unique_ptr is nullable
                (true, _) => self.formatter.fmt_owned(&type_name),
                (false, true) => self.formatter.fmt_optional_borrowed(&type_name, mutability),
                (false, false) => self.formatter.fmt_borrowed(&type_name, mutability),
            }
            .into_owned()
            .into()
        } else {
            type_name
        }
    }

    fn gen_fn_sig(&mut self, cb: &dyn CallbackInstantiationFunctionality) -> String {
        let t = cb.get_output_type().unwrap();

        let return_type = self.gen_cpp_return_type_name(t, false);

        let params_types = cb
            .get_inputs()
            .unwrap()
            .iter()
            .map(|p| self.gen_type_name(&p.ty).to_string())
            .collect::<Vec<_>>()
            .join(", ");

        format!("{return_type}({params_types})")
    }

    /// Generates a C++ expression that converts from the C++ self type to the corresponding C self type.
    pub(super) fn gen_cpp_to_c_self(&self, ty: &SelfType) -> Cow<'static, str> {
        match *ty {
            SelfType::Opaque(..) => "this->AsFFI()".into(),
            SelfType::Struct(ref s) => {
                let attrs = &s.resolve(self.c.tcx).attrs;
                if attrs.abi_compatible {
                    if let MaybeOwn::Borrow(b) = s.owner {
                        let c_name = self.formatter.namespace_c_name(
                            s.id().into(),
                            &self.formatter.fmt_type_name_unnamespaced(s.id()),
                        );

                        return match b.mutability {
                            Mutability::Immutable => {
                                format!("reinterpret_cast<const {c_name}*>(this)")
                            }
                            Mutability::Mutable => {
                                format!("reinterpret_cast<{c_name}*>(this)")
                            }
                        }
                        .into();
                    }
                }
                "this->AsFFI()".into()
            }
            SelfType::Enum(..) => "this->AsFFI()".into(),
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    /// Generates one or two C++ expressions that convert from a C++ field to the corresponding C field.
    ///
    /// Returns `NamedExpression`s whose `var_name` corresponds to the field of the C struct.
    ///
    /// `cpp_struct_access` should be code for referencing a field of the C++ struct.
    fn gen_cpp_to_c_for_field<'a, P: TyPosition>(
        &mut self,
        cpp_struct_access: &str,
        field: &'a hir::StructField<P>,
        namespace: Option<String>,
    ) -> NamedExpression<'a> {
        let var_name = self.formatter.fmt_param_name(field.name.as_str());
        let field_getter = format!("{cpp_struct_access}{var_name}");
        let expression =
            self.gen_cpp_to_c_for_type(&field.ty, field_getter.into(), None, namespace);

        NamedExpression {
            var_name,
            expression,
        }
    }

    /// Generates one or two C++ expressions that convert from a C++ type to the corresponding C type.
    ///
    /// Returns a `PartiallyNamedExpression` whose `suffix` is either empty, `_data`, or `_size` for
    /// referencing fields of the C struct.
    pub(super) fn gen_cpp_to_c_for_type<'a, P: TyPosition>(
        &mut self,
        ty: &Type<P>,
        cpp_name: Cow<'a, str>,
        method_abi_name: Option<String>,
        namespace: Option<String>,
    ) -> Cow<'a, str> {
        let lib_name_ns_prefix = &self.formatter.lib_name_ns_prefix;
        match *ty {
            Type::Primitive(..) => cpp_name.clone(),
            Type::Opaque(ref op) if op.is_optional() => {
                format!("{cpp_name} ? {cpp_name}->AsFFI() : nullptr").into()
            }
            Type::Opaque(ref path) if path.is_owned() => format!("{cpp_name}->AsFFI()").into(),
            Type::Opaque(..) => format!("{cpp_name}.AsFFI()").into(),
            Type::Struct(ref s) => {
                let attrs = match self.c.tcx.resolve_type(s.id()) {
                    TypeDef::OutStruct(s) => &s.attrs,
                    TypeDef::Struct(s) => &s.attrs,
                    _ => unreachable!()
                };

                if attrs.abi_compatible {
                    if let MaybeOwn::Borrow(borrow) = s.owner() {
                        let c_name = self.formatter.namespace_c_name(s.id().into(), &self.formatter.fmt_type_name_unnamespaced(s.id()));
                        return match borrow.mutability {
                            Mutability::Immutable => {
                                format!("reinterpret_cast<const {c_name}*>(&{cpp_name})")
                            },
                            Mutability::Mutable => {
                                format!("reinterpret_cast<{c_name}*>(&{cpp_name})")
                            }
                        }.into();
                    }
                }
                format!("{cpp_name}.AsFFI()").into()
            },
            Type::Enum(..) => format!("{cpp_name}.AsFFI()").into(),
            Type::Slice(Slice::Strs(..)) => format!(
                // This cast is valid as diplomat::string_view_for_slice is used to ensure correct layout
                "{{reinterpret_cast<const {lib_name_ns_prefix}diplomat::capi::DiplomatStringView*>({cpp_name}.data()), {cpp_name}.size()}}"
            ).into(),
            Type::Slice(Slice::Struct(b, ref st)) => format!("{{reinterpret_cast<{}{}*>({cpp_name}.data()), {cpp_name}.size()}}",
                if b.mutability().is_mutable() { "" } else { "const " },
                self.formatter.namespace_c_name(st.id().into(), &self.formatter.fmt_type_name_unnamespaced(st.id()))
            ).into(),
            Type::Slice(..) => format!("{{{cpp_name}.data(), {cpp_name}.size()}}").into(),
            Type::DiplomatOption(ref inner) => {
                let conversion =
                    self.gen_cpp_to_c_for_type(inner, format!("{cpp_name}.value()").into(), method_abi_name, namespace);
                let copt = self.c.gen_ty_name(ty, &mut Default::default());
                format!("{cpp_name}.has_value() ? ({copt}{{ {{ {conversion} }}, true }}) : ({copt}{{ {{}}, false }})").into()
            }
            Type::Callback(ref c) => {
                let run_callback = match c.get_output_type().unwrap() {
                    ReturnType::Fallible(ref ok, ref err) => {
                        let ok_type_name = match ok {
                            hir::SuccessType::Unit => "std::monostate".into(),
                            hir::SuccessType::OutType(o) => self.gen_type_name(o),
                            _ => unreachable!("unknown AST/HIR variant"),
                        };

                        let err_type_name = match err {
                            Some(o) => self.gen_type_name(o),
                            None => "std::monostate".into(),
                        };

                        let return_type = self.formatter.fmt_c_api_callback_ret(namespace, method_abi_name.unwrap(), &cpp_name);

                        self.formatter.fmt_run_callback_converter(&cpp_name, "c_run_callback_result", vec![&ok_type_name, &err_type_name, &return_type])
                    },
                    ReturnType::Nullable(ref success) => {
                        let type_name = match success {
                            hir::SuccessType::Unit => "std::monostate".into(),
                            hir::SuccessType::OutType(o) => self.gen_type_name(o),
                            _ => unreachable!("unknown AST/HIR variant"),
                        };

                        let return_type = self.formatter.fmt_c_api_callback_ret(namespace, method_abi_name.unwrap(), &cpp_name);
                        self.formatter.fmt_run_callback_converter(&cpp_name, "c_run_callback_diplomat_option", vec![&type_name, &return_type])
                    }
                    ReturnType::Infallible(SuccessType::OutType(Type::Opaque(o))) => {
                        let opaque_type = self.c.formatter.fmt_type_name_maybe_namespaced(o.tcx_id.into());
                        let ptr_ty = self.c.formatter.fmt_ptr(&opaque_type, o.owner.mutability);
                        self.formatter.fmt_run_callback_converter(&cpp_name, "c_run_callback_diplomat_opaque", vec![&ptr_ty])
                    },
                    _ => format!("{lib_name_ns_prefix}diplomat::fn_traits({cpp_name}).c_run_callback")
                };
                format!("{{new decltype({cpp_name})(std::move({cpp_name})), {run_callback}, {lib_name_ns_prefix}diplomat::fn_traits({cpp_name}).c_delete}}",).into()
            }
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    /// Generates the C++ type name of a return type.
    ///
    /// is_generic_write is whether we are generating the method that returns a string or
    /// operates on a Writeable
    pub(super) fn gen_cpp_return_type_name<P: hir::TyPosition>(
        &mut self,
        result_ty: &ReturnType<P>,
        is_generic_write: bool,
    ) -> Cow<'ccx, str> {
        let lib_name_ns_prefix = &self.formatter.lib_name_ns_prefix;
        match *result_ty {
            ReturnType::Infallible(SuccessType::Unit) => "void".into(),
            ReturnType::Infallible(SuccessType::Write) if is_generic_write => "void".into(),
            ReturnType::Infallible(SuccessType::Write) => self.formatter.fmt_owned_str(),
            ReturnType::Infallible(SuccessType::OutType(ref o)) => self.gen_type_name(o),
            ReturnType::Fallible(ref ok, ref err) => {
                let ok_type_name = match ok {
                    SuccessType::Write if is_generic_write => "std::monostate".into(),
                    SuccessType::Write => self.formatter.fmt_owned_str(),
                    SuccessType::Unit => "std::monostate".into(),
                    SuccessType::OutType(o) => self.gen_type_name(o),
                    _ => unreachable!("unknown AST/HIR variant"),
                };
                let err_type_name = match err {
                    Some(o) => self.gen_type_name(o),
                    None => "std::monostate".into(),
                };
                format!("{lib_name_ns_prefix}diplomat::result<{ok_type_name}, {err_type_name}>")
                    .into()
            }
            ReturnType::Nullable(ref ty) => {
                let type_name = match ty {
                    SuccessType::Write if is_generic_write => "std::monostate".into(),
                    SuccessType::Write => self.formatter.fmt_owned_str(),
                    SuccessType::Unit => "std::monostate".into(),
                    SuccessType::OutType(o) => self.gen_type_name(o),
                    _ => unreachable!("unknown AST/HIR variant"),
                };
                self.formatter.fmt_optional(&type_name).into()
            }
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    /// Generates a C++ expression that converts from a C field to the corresponding C++ field.
    ///
    /// `c_struct_access` should be code for referencing a field of the C struct.
    fn gen_c_to_cpp_for_field<'a, P: TyPosition>(
        &self,
        c_struct_access: &str,
        field: &'a hir::StructField<P>,
    ) -> NamedExpression<'a> {
        let var_name = self.formatter.fmt_param_name(field.name.as_str());
        let field_getter = format!("{c_struct_access}{var_name}");
        let expression = self.gen_c_to_cpp_for_type(&field.ty, field_getter.into());
        NamedExpression {
            var_name,
            expression,
        }
    }

    /// Generates a C++ expression that converts from a C type to the corresponding C++ type.
    ///
    /// If the type is a slice, this function assumes that `{var_name}_data` and `{var_name}_size` resolve
    /// to valid expressions referencing the two different C variables for the pointer and the length.
    fn gen_c_to_cpp_for_type<'a, P: TyPosition>(
        &self,
        ty: &Type<P>,
        var_name: Cow<'a, str>,
    ) -> Cow<'a, str> {
        let var_name = self.formatter.fmt_identifier(var_name);

        match *ty {
            Type::Primitive(..) => var_name,
            Type::Opaque(ref op) if op.owner.is_owned() => {
                let id = op.tcx_id.into();
                let type_name = self.formatter.fmt_type_name(id);
                // Note: The impl file is imported in gen_type_name().
                format!("std::unique_ptr<{type_name}>({type_name}::FromFFI({var_name}))").into()
            }
            Type::Opaque(ref op) if op.is_optional() => {
                let id = op.tcx_id.into();
                let type_name = self.formatter.fmt_type_name(id);
                if op.is_owned() {
                    // Note: The impl file is imported in gen_type_name().
                    format!("{var_name} ? {{ *{type_name}::FromFFI({var_name}) }} : std::nullopt")
                        .into()
                } else {
                    format!("{type_name}::FromFFI({var_name})").into()
                }
            }
            Type::Opaque(ref op) => {
                let id = op.tcx_id.into();
                let type_name = self.formatter.fmt_type_name(id);
                // Note: The impl file is imported in gen_type_name().
                format!("*{type_name}::FromFFI({var_name})").into()
            }
            Type::Struct(ref st) => {
                let is_zst = match self.c.tcx.resolve_type(ty.id().unwrap()) {
                    TypeDef::Struct(s) => s.fields.is_empty(),
                    TypeDef::OutStruct(s) => s.fields.is_empty(),
                    _ => false,
                };

                let id = st.id();
                let type_name = self.formatter.fmt_type_name(id);
                if is_zst {
                    format!("{type_name} {{}}").into()
                } else {
                    // Note: The impl file is imported in gen_type_name().
                    format!("{type_name}::FromFFI({var_name})").into()
                }
            }
            Type::Enum(ref e) => {
                let id = e.tcx_id.into();
                let type_name = self.formatter.fmt_type_name(id);
                // Note: The impl file is imported in gen_type_name().
                format!("{type_name}::FromFFI({var_name})").into()
            }
            Type::Slice(hir::Slice::Str(_, encoding)) => {
                let string_view = self.formatter.fmt_borrowed_str(encoding);
                format!("{string_view}({var_name}.data, {var_name}.len)").into()
            }
            Type::Slice(hir::Slice::Primitive(b, p)) => {
                let prim_name = self.formatter.fmt_primitive_as_c(p);
                let span = self
                    .formatter
                    .fmt_borrowed_slice(&prim_name, b.mutability());
                format!("{span}({var_name}.data, {var_name}.len)").into()
            }
            Type::Slice(hir::Slice::Struct(b, ref st_ty)) => {
                let mt = b.mutability();
                let st_name = self.formatter.fmt_type_name(st_ty.id());
                let span = self.formatter.fmt_borrowed_slice(&st_name, mt);
                format!(
                    "{span}(reinterpret_cast<{}{st_name}*>({var_name}.data), {var_name}.len)",
                    if mt.is_mutable() { "" } else { "const " }
                )
                .into()
            }
            Type::DiplomatOption(ref inner) => {
                let conversion = self.gen_c_to_cpp_for_type(inner, format!("{var_name}.ok").into());
                format!("{var_name}.is_ok ? std::optional({conversion}) : std::nullopt").into()
            }
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    /// Generates a C++ expression that converts from a C return type to the corresponding C++ return type.
    ///
    /// If the type is `SuccessType::Write`, this function assumes that there is a variable named `output` in scope.
    pub(super) fn gen_c_to_cpp_for_return_type<'a>(
        &mut self,
        result_ty: &ReturnType,
        var_name: Cow<'a, str>,
        is_generic_write: bool,
    ) -> Option<Cow<'a, str>> {
        let lib_name_ns_prefix = &self.formatter.lib_name_ns_prefix;
        match *result_ty {
            ReturnType::Infallible(SuccessType::Unit) => None,
            ReturnType::Infallible(SuccessType::Write) if is_generic_write => None,
            ReturnType::Infallible(SuccessType::Write) => Some("std::move(output)".into()),
            ReturnType::Infallible(SuccessType::OutType(ref out_ty)) => {
                Some(self.gen_c_to_cpp_for_type(out_ty, var_name))
            }
            ReturnType::Fallible(ref ok, ref err) => {
                let ok_type_name = match ok {
                    SuccessType::Write if is_generic_write => "std::monostate".into(),
                    SuccessType::Write => self.formatter.fmt_owned_str(),
                    SuccessType::Unit => "std::monostate".into(),
                    SuccessType::OutType(ref o) => self.gen_type_name(o),
                    _ => unreachable!("unknown AST/HIR variant"),
                };
                let err_type_name = match err {
                    Some(o) => self.gen_type_name(o),
                    None => "std::monostate".into(),
                };
                let ok_conversion = match ok {
                    SuccessType::Write if is_generic_write => "".into(),
                    // Note: the `output` variable is a string initialized in the template
                    SuccessType::Write => "std::move(output)".into(),
                    SuccessType::Unit => "".into(),
                    SuccessType::OutType(ref o) => {
                        self.gen_c_to_cpp_for_type(o, format!("{var_name}.ok").into())
                    }
                    _ => unreachable!("unknown AST/HIR variant"),
                };
                let err_conversion = match err {
                    Some(o) => self.gen_c_to_cpp_for_type(o, format!("{var_name}.err").into()),
                    None => "".into(),
                };
                Some(
                    format!("{var_name}.is_ok ? {lib_name_ns_prefix}diplomat::result<{ok_type_name}, {err_type_name}>({lib_name_ns_prefix}diplomat::Ok<{ok_type_name}>({ok_conversion})) : {lib_name_ns_prefix}diplomat::result<{ok_type_name}, {err_type_name}>({lib_name_ns_prefix}diplomat::Err<{err_type_name}>({err_conversion}))").into()
                )
            }
            ReturnType::Nullable(ref ty) => {
                let type_name = match ty {
                    SuccessType::Write if is_generic_write => "std::monostate".into(),
                    SuccessType::Write => self.formatter.fmt_owned_str(),
                    SuccessType::Unit => "std::monostate".into(),
                    SuccessType::OutType(o) => self.gen_type_name(o),
                    _ => unreachable!("unknown AST/HIR variant"),
                };

                let conversion = match ty {
                    SuccessType::Write if is_generic_write => "".into(),
                    // Note: the `output` variable is a string initialized in the template
                    SuccessType::Write => "std::move(output)".into(),
                    SuccessType::Unit => "".into(),
                    SuccessType::OutType(ref o) => {
                        self.gen_c_to_cpp_for_type(o, format!("{var_name}.ok").into())
                    }
                    _ => unreachable!("unknown AST/HIR variant"),
                };

                Some(format!("{var_name}.is_ok ? std::optional<{type_name}>({conversion}) : std::nullopt").into())
            }
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }
}
