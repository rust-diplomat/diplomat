use super::header::Header;
use super::Cpp2Formatter;
use crate::c::Header as C2Header;
use crate::c::ItemGenContext as CItemGenContext;
use crate::config;
use crate::read_custom_binding;
use crate::ErrorStore;
use askama::Template;
use diplomat_core::hir::borrowing_param::ParamBorrowInfo;
use diplomat_core::hir::CallbackInstantiationFunctionality;
use diplomat_core::hir::IncludeLocation;
use diplomat_core::hir::IncludeSource;
use diplomat_core::hir::OpaqueId;
use diplomat_core::hir::OpaquePath;
use diplomat_core::hir::Slice;
use diplomat_core::hir::{
    self, MaybeOwn, Mutability, OpaqueOwner, ReturnType, SelfType, StructPathLike, SuccessType,
    SymbolId, TyPosition, Type, TypeDef,
};
use std::borrow::Cow;
use std::collections::HashMap;

use crate::c::CAPI_NAMESPACE;
use crate::filters;

/// A type name with a corresponding variable name, such as a struct field or a function parameter.
pub struct NamedType<'a> {
    pub(crate) var_name: Cow<'a, str>,
    pub(crate) type_name: Cow<'a, str>,
    /// Default value (for method params, but could eventually be for structs).
    pub(crate) default_value: Option<Cow<'a, str>>,
    pub(crate) lifetimebound: bool,
}

impl<'a> NamedType<'a> {
    fn type_name(&'a self) -> Cow<'a, str> {
        self.type_name.clone()
    }
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
    extra_impl_code: ExtraCode,
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

pub(crate) struct ExtraCode {
    pub pre: String,
    pub post: String,
    pub inner: String,
}

/// Context for generating a particular type's header
pub(crate) struct ItemGenContext<'ccx, 'tcx, 'header> {
    pub formatter: &'ccx Cpp2Formatter<'tcx>,
    /// Use instead of CppConfig to allow access to SharedConfig.
    pub config: &'ccx config::Config,
    pub errors: &'ccx ErrorStore<'tcx, String>,
    pub c: CItemGenContext<'ccx, 'tcx, 'header>,
    pub impl_header: &'header mut Header<'ccx>,
    pub decl_header: &'header mut Header<'ccx>,
    /// Are we currently generating struct fields?
    pub generating_struct_fields: bool,
}

impl<'ccx, 'tcx: 'ccx> ItemGenContext<'ccx, 'tcx, '_> {
    fn def_extra_code_from_attrs(
        &self,
        custom_extra_code: &HashMap<IncludeLocation, IncludeSource>,
    ) -> ExtraCode {
        let extra_def_code = if let Some(s) = custom_extra_code.get(&IncludeLocation::DefBlock) {
            read_custom_binding(s, self.config, self.errors).unwrap_or_default()
        } else {
            Default::default()
        };

        let pre_extra_def_code =
            if let Some(s) = custom_extra_code.get(&IncludeLocation::PreDefBlock) {
                read_custom_binding(s, self.config, self.errors).unwrap_or_default()
            } else {
                Default::default()
            };

        let post_extra_def_code =
            if let Some(s) = custom_extra_code.get(&IncludeLocation::PostDefBlock) {
                read_custom_binding(s, self.config, self.errors).unwrap_or_default()
            } else {
                Default::default()
            };
        ExtraCode {
            pre: pre_extra_def_code,
            post: post_extra_def_code,
            inner: extra_def_code,
        }
    }

    fn impl_extra_code_from_attrs(
        &self,
        custom_extra_code: &HashMap<IncludeLocation, IncludeSource>,
    ) -> ExtraCode {
        let extra_impl_code = if let Some(s) = custom_extra_code.get(&IncludeLocation::ImplBlock) {
            read_custom_binding(s, self.config, self.errors).unwrap_or_default()
        } else {
            Default::default()
        };

        let pre_extra_impl_code =
            if let Some(s) = custom_extra_code.get(&IncludeLocation::PreImplBlock) {
                read_custom_binding(s, self.config, self.errors).unwrap_or_default()
            } else {
                Default::default()
            };

        ExtraCode {
            pre: pre_extra_impl_code,
            post: Default::default(),
            inner: extra_impl_code,
        }
    }

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

        let extra_def_code = self.def_extra_code_from_attrs(&ty.attrs.custom_extra_code);

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
            extra_def_code: ExtraCode,
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
            extra_def_code,
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
            extra_impl_code: ExtraCode,
        }

        ImplTemplate {
            ty,
            fmt: self.formatter,
            type_name: &type_name,
            ctype: &ctype,
            methods: methods.as_slice(),
            namespace: ty.attrs.namespace.as_deref(),
            c_header: c_impl_header,
            extra_impl_code: self.impl_extra_code_from_attrs(&ty.attrs.custom_extra_code),
        }
        .render_into(self.impl_header)
        .unwrap();
    }

    pub fn gen_opaque_def(&mut self, id: OpaqueId) {
        let type_name = self.formatter.fmt_type_name(id.into());
        let type_name_unnamespaced = self.formatter.fmt_type_name_unnamespaced(id.into());
        let ctype = self.formatter.fmt_c_type_name(id.into());
        let ty = self.c.tcx.resolve_opaque(id);
        let dtor_name = self
            .formatter
            .namespace_c_name(id.into(), ty.dtor_abi_name.as_str());

        let mut c_header = self.c.gen_opaque_def(id);
        let c_impl_header = self.c.gen_impl(id.into());

        // `OpaquePointer<T, CPtr, Destructor>` needs `Destructor` (the dtor's fully-qualified
        // name) at class-definition time in the *decl* header, so the dtor's `extern "C"` decl
        // needs to be visible there too, not just in the impl header (where every other method
        // is declared). This is otherwise identical to the dtor decl the impl header emits.
        c_header.body.push_str(&format!(
            "extern \"C\" {{\nvoid {}({}* self);\n}}\n",
            ty.dtor_abi_name.as_str(),
            type_name_unnamespaced,
        ));

        let methods = ty
            .methods
            .iter()
            .flat_map(|method| self.gen_method_info(id.into(), method))
            .collect::<Vec<_>>();

        let extra_def_code = self.def_extra_code_from_attrs(&ty.attrs.custom_extra_code);

        #[derive(Template)]
        #[template(path = "cpp/opaque_decl.h.jinja", escape = "none")]
        struct DeclTemplate<'a> {
            // ty: &'a hir::OpaqueDef,
            fmt: &'a Cpp2Formatter<'a>,
            type_name: &'a str,
            ctype: &'a str,
            dtor_name: &'a str,
            methods: &'a [MethodInfo<'a>],
            namespace: Option<&'a str>,
            type_name_unnamespaced: &'a str,
            c_header: C2Header,
            docs: &'a str,
            deprecated: Option<&'a str>,
            extra_def_code: ExtraCode,
        }

        DeclTemplate {
            // ty,
            fmt: self.formatter,
            type_name: &type_name,
            ctype: &ctype,
            dtor_name: &dtor_name,
            methods: methods.as_slice(),
            namespace: ty.attrs.namespace.as_deref(),
            type_name_unnamespaced: &type_name_unnamespaced,
            c_header,
            docs: &self.formatter.fmt_docs(&ty.docs, &ty.attrs),
            deprecated: ty.attrs.deprecated.as_deref(),
            extra_def_code,
        }
        .render_into(self.decl_header)
        .unwrap();

        #[derive(Template)]
        #[template(path = "cpp/opaque_impl.h.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            // ty: &'a hir::OpaqueDef,
            fmt: &'a Cpp2Formatter<'a>,
            type_name: &'a str,
            methods: &'a [MethodInfo<'a>],
            namespace: Option<&'a str>,
            c_header: C2Header,
            extra_impl_code: ExtraCode,
        }

        ImplTemplate {
            // ty,
            fmt: self.formatter,
            type_name: &type_name,
            methods: methods.as_slice(),
            namespace: ty.attrs.namespace.as_deref(),
            c_header: c_impl_header,
            extra_impl_code: self.impl_extra_code_from_attrs(&ty.attrs.custom_extra_code),
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
            .map(|field| self.gen_field_ty_decl(&field.ty, field.name.as_str()))
            .collect::<Vec<_>>();
        self.generating_struct_fields = false;

        let cpp_to_c_fields = def
            .fields
            .iter()
            .enumerate()
            .map(|(idx, field)| {
                self.gen_cpp_to_c_for_field(
                    if def.attrs.tuple { Some(idx) } else { None },
                    field,
                    namespace.clone(),
                )
            })
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

        let extra_def_code = self.def_extra_code_from_attrs(&def.attrs.custom_extra_code);

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
            extra_def_code: ExtraCode,
            tuple: bool,
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
            is_sliceable: def.attrs.abi_compatible && def.usage.sliced,
            docs: &self.formatter.fmt_docs(&def.docs, &def.attrs),
            deprecated: def.attrs.deprecated.as_deref(),
            extra_def_code,
            tuple: def.attrs.tuple,
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
            fields: &'a [NamedType<'a>],
            cpp_to_c_fields: &'a [NamedExpression<'a>],
            c_to_cpp_fields: &'a [NamedExpression<'a>],
            methods: &'a [MethodInfo<'a>],
            namespace: Option<&'a str>,
            c_header: C2Header,
            extra_impl_code: ExtraCode,
            tuple: bool,
        }

        ImplTemplate {
            // ty,
            fmt: self.formatter,
            type_name: &type_name,
            ctype: &ctype,
            fields: field_decls.as_slice(),
            cpp_to_c_fields: cpp_to_c_fields.as_slice(),
            c_to_cpp_fields: c_to_cpp_fields.as_slice(),
            methods: methods.as_slice(),
            namespace: def.attrs.namespace.as_deref(),
            c_header: c_impl_header,
            extra_impl_code: self.impl_extra_code_from_attrs(&def.attrs.custom_extra_code),
            tuple: def.attrs.tuple,
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

        let mut this_borrowed = false;
        let mut visitor = method.borrowing_param_visitor(self.c.tcx, false);

        if let Some(param_self) = method.param_self.as_ref() {
            let info = visitor.visit_param(&param_self.ty.clone().into(), "this");
            if !matches!(
                info,
                ParamBorrowInfo::NotBorrowed | ParamBorrowInfo::TemporarySlice
            ) {
                this_borrowed = true;
            }
        }

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
            let mut decls = self.gen_ty_decl(&param.ty, param.name.as_str(), true);
            let info = visitor.visit_param(&param.ty, decls.var_name.as_ref());
            if !matches!(
                info,
                ParamBorrowInfo::NotBorrowed | ParamBorrowInfo::TemporarySlice
            ) {
                decls.lifetimebound = true;
            }
            if let Some(d) = &param.attrs.default_value {
                let s = match d {
                    hir::DefaultArgValue::Bool(b) => b.to_string(),
                    hir::DefaultArgValue::Char(c) => format!(r#"{{ "{}", {} }}"#, c, c.len_utf8()),
                    hir::DefaultArgValue::Integer(i) => i.to_string(),
                    hir::DefaultArgValue::Float(f) => f.to_string(),
                    _ => panic!("Default arg value {d:?} not implemented."),
                };
                // `Optional<T>`'s converting constructors are explicit (see runtime.hpp.jinja),
                // so a bare literal default (`= 0`) no longer implicitly converts when the param
                // itself is `Optional<T>` (i.e. a `DiplomatOption<T>` param) -- construct it
                // explicitly instead.
                let s = if matches!(param.ty, Type::DiplomatOption(_)) {
                    format!("{}({s})", decls.type_name)
                } else {
                    s
                };
                decls.default_value = Some(s.into());
            }
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
                true,
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

        let mut post_qualifiers = match &method.param_self {
            Some(param_self)
                if param_self.ty.is_immutably_borrowed() || param_self.ty.is_consuming() =>
            {
                vec!["const".into()]
            }
            Some(_) => vec![],
            None => vec![],
        };

        if this_borrowed {
            post_qualifiers.push("DIPLOMAT_LIFETIME_BOUND".into());
        }

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
            extra_impl_code: self.impl_extra_code_from_attrs(&method.attrs.custom_extra_code),
        })
    }

    /// Generates a field's type (based on [`Self::gen_ty_decl`]).
    /// `is_param` distinguishes parameter/self position (where a borrowed opaque can safely be
    /// a real `Foo&`/`const Foo&`, since the caller binds it directly to a real `Foo`) from
    /// return/field position (where a borrowed opaque falls back to the raw `capi::Foo*` /
    /// `const capi::Foo*` pointer: manufacturing a `Foo&` there would require reinterpret_casting
    /// the address of a short-lived local variable, which dangles once that variable goes out of
    /// scope). See [`Self::gen_opaque_name`].
    pub(crate) fn gen_field_ty_decl<'a, P: TyPosition>(
        &mut self,
        ty: &Type<P>,
        var_name: &'a str,
    ) -> NamedType<'a>
    where
        'ccx: 'a,
    {
        self.gen_ty_decl(ty, var_name, false)
    }

    /// Generates C++ code for referencing a particular type with a given name.
    pub(super) fn gen_ty_decl<'a, P: TyPosition>(
        &mut self,
        ty: &Type<P>,
        var_name: &'a str,
        is_param: bool,
    ) -> NamedType<'a>
    where
        'ccx: 'a,
    {
        let var_name = self.formatter.fmt_param_name(var_name);
        let type_name = self.gen_type_name(ty, is_param);

        NamedType {
            var_name,
            type_name,
            default_value: None,
            lifetimebound: false,
        }
    }

    /// Generates C++ code for referencing a particular type.
    ///
    /// This function adds the necessary type imports to the decl and impl files.
    ///
    /// `is_param` is true for parameter/self position, false for return/field position; see
    /// [`Self::gen_opaque_name`] for why this matters.
    pub(crate) fn gen_type_name<P: TyPosition>(
        &mut self,
        ty: &Type<P>,
        is_param: bool,
    ) -> Cow<'ccx, str> {
        let lib_name_ns_prefix = &self.formatter.lib_name_ns_prefix;
        match *ty {
            Type::Primitive(prim) => self.formatter.fmt_primitive_as_c(prim),
            Type::Opaque(ref op) => self.gen_opaque_name::<P>(op, is_param),
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
            Type::Slice(hir::Slice::Opaque(_, ref op_ty)) => {
                // Trigger the same forward-declare/include side effects as a normal opaque
                // reference (the return value is discarded; we compute the element type below).
                self.gen_opaque_name::<hir::Everywhere>(op_ty, false);

                // Each element is a plain `Foo` value, always non-const regardless of op_ty's own
                // mutability (never `Foo*`, even if op_ty is optional -- a null `capi::Foo*` in
                // the underlying array just becomes a `Foo` with `operator bool() == false`).
                // This is legal because `Foo`'s entire layout is one `capi::Foo*` (so an array of
                // `capi::Foo*` and an array of `Foo` are bit-identical), and unlike a struct field
                // or return value, `std::span` never constructs or destroys its elements --
                // there's no risk of `Foo`'s destructor firing on memory a slice doesn't own.
                //
                // Elements can't be `const Foo` even for an immutable op_ty: `.data()` on
                // `span<const Foo>` would be `const Foo*`, and reinterpret_cast to the
                // ABI's fixed `const capi::Foo**` target would then be casting away constness
                // (source pointee `const Foo`, target pointee `const capi::Foo*` -- itself
                // non-const -- is a strictly *lower* qualification, which reinterpret_cast
                // disallows). So, same as the container itself, the element type doesn't reflect
                // borrow mutability at the C++ type level; Rust is still the actual enforcement.
                let type_name = self.formatter.fmt_type_name(op_ty.tcx_id.into());

                let element_type: Cow<'ccx, str> = if op_ty.is_optional() {
                    // A `None` element is a null pointer in the underlying array. `Foo` (the
                    // owning wrapper used below for the non-optional case) has no safe public way
                    // to represent that: `Foo::FromFFI(nullptr)` builds a second `Foo` aliasing
                    // the same (null) pointer, and unlike the non-optional case, a *non-null*
                    // entry would alias a `Foo` that's still owned elsewhere -- if that element
                    // were ever moved out of the span, its destructor would double-free.
                    // `Optional<FooRef>`/`Optional<FooRefMut>` are the pointer-like specialization
                    // of `Optional` (non-owning, same one-pointer layout, so the reinterpret_cast
                    // below is still valid), so building this array from `.as_ref()`/
                    // `.as_mut_ref()` (for `Some`) and `std::nullopt` (for `None`) carries no such
                    // risk, and is explicit about which entries are absent rather than relying on
                    // a silently-nullable `FooRef`.
                    let mutability = op_ty.owner.mutability().unwrap_or(hir::Mutability::Mutable);
                    let ref_type = self.formatter.fmt_opaque_ref(&type_name, mutability);
                    self.formatter.fmt_optional(&ref_type).into()
                } else {
                    type_name.clone()
                };

                let ret = self
                    .formatter
                    .fmt_borrowed_slice(&element_type, hir::Mutability::Mutable);
                ret.into_owned().into()
            }
            Type::Callback(ref cb) => format!("std::function<{}>", self.gen_fn_sig(cb)).into(),
            Type::DiplomatOption(ref inner) => {
                let inner_name = self.gen_type_name(inner, is_param);
                self.formatter.fmt_optional(&inner_name).into()
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

        let struct_ty = self.c.tcx.resolve_type(st.id());
        if struct_ty.attrs().tuple && matches!(st.owner(), MaybeOwn::Own) {
            // Must agree with gen_struct_def's own field_decls computation for this same tuple
            // struct -- see gen_opaque_name's doc comment for why that's guaranteed here (both
            // call gen_type_name with the same is_param, no extra flag needed).
            let field_names = match struct_ty {
                TypeDef::Struct(st) => st
                    .fields
                    .iter()
                    .map(|f| self.gen_type_name(&f.ty, false))
                    .collect::<Vec<_>>(),
                TypeDef::OutStruct(st) => st
                    .fields
                    .iter()
                    .map(|f| self.gen_type_name(&f.ty, false))
                    .collect::<Vec<_>>(),
                _ => unreachable!(),
            };
            return self.formatter.fmt_tuple(field_names);
        }
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

    /// Computes the C++ type used to refer to an opaque type at a particular usage site.
    ///
    /// Owned paths are always `Foo` (optionally `Optional<Foo>`), regardless of position.
    ///
    /// Borrowed paths depend on `is_param`:
    /// - `is_param = true` (parameter/self position, non-optional): `Foo&`/`const Foo&`. This is
    ///   safe because the caller binds the reference directly to a real, caller-owned `Foo`; no
    ///   conversion from a raw pointer is ever needed.
    /// - `is_param = false` anywhere else (return position, struct field -- including a tuple
    ///   struct's own field and its `std::tuple<...>` element type as seen by callers, see
    ///   `gen_struct_name`'s tuple branch --, callback param/return): `Ref<Foo, const
    ///   capi::Foo>`/`Ref<Foo, capi::Foo>` (aliased `FooRef`/`FooRefMut` -- mutability is encoded
    ///   by the second argument's own const-qualification, see `Ref` in `runtime.hpp.jinja`), or
    ///   `Optional<FooRef>`/`Optional<FooRefMut>` if the borrow itself is optional (`Option<&Foo>`
    ///   on the Rust side). Unlike a raw `Foo&`, `Ref` is a self-contained, trivially-copyable,
    ///   non-owning view (it holds the raw pointer itself rather than requiring an existing `Foo`
    ///   object to bind to), so it's safe to store/return/pass through a `std::function` (or a
    ///   `std::tuple`) and reassign freely, and can be reconstructed straight from a raw pointer
    ///   via `Ref<Foo,...>::FromFFI` without ever constructing/destroying a `Foo`.
    ///   `std::optional<Foo&>` isn't valid pre-C++23, which is why the wrapping happens around
    ///   `Ref` (an ordinary value type) instead of around a real reference.
    ///
    ///   A tuple struct's element type is independently computed in two places that must agree
    ///   exactly: the struct's own `AsTupleFFI`/`TupleFromFFI` (built from `gen_struct_def`'s
    ///   `field_decls`) has to accept/return literally the same `std::tuple<...>` type that a
    ///   caller elsewhere in the API declares as a parameter/return/field type (built from
    ///   `gen_struct_name`'s tuple branch). Both go through this same function with the same
    ///   `is_param`, so they agree automatically -- no extra flag needed, unlike the two
    ///   `generating_struct_fields`-guarded cases above.
    fn gen_opaque_name<P: TyPosition>(
        &mut self,
        op: &OpaquePath<hir::Optional, P::OpaqueOwnership>,
        is_param: bool,
    ) -> Cow<'ccx, str> {
        let op_id = op.tcx_id.into();
        let type_name = self.formatter.fmt_type_name(op_id);
        let type_name_unnamespaced = self.formatter.fmt_type_name_unnamespaced(op_id);
        let def = self.c.tcx.resolve_type(op_id);

        if def.attrs().disable {
            self.errors
                .push_error(format!("Found usage of disabled type {type_name}"))
        }

        let mutability = op.owner.mutability().unwrap_or(hir::Mutability::Mutable);
        // A plain (non-optional) borrowed param can bind straight to a real `Foo&`/`const Foo&`
        // (see the arm below), but an *optional* borrowed param can't -- `Optional<Foo&>` isn't
        // representable (references can't be rebound/absent) -- so it needs the same
        // `Ref`-based representation as every other borrowed, non-param position.
        let using_opaque_ref = !op.owner.is_owned() && (!is_param || op.is_optional());
        let ret: Cow<'ccx, str> = match (op.owner.is_owned(), op.is_optional(), is_param) {
            (true, false, _) => type_name.clone(),
            (true, true, _) => self.formatter.fmt_optional(&type_name).into(),
            (false, false, true) => self
                .formatter
                .fmt_borrowed(&type_name, mutability)
                .into_owned()
                .into(),
            (false, false, false) if using_opaque_ref => {
                self.formatter.fmt_opaque_ref(&type_name, mutability).into()
            }
            (false, true, _) if using_opaque_ref => {
                let opaque_ref = self.formatter.fmt_opaque_ref(&type_name, mutability);
                self.formatter.fmt_optional(&opaque_ref).into()
            }
            (false, _, _) => {
                let ctype = self.formatter.fmt_c_type_name(op_id);
                self.c
                    .formatter
                    .fmt_ptr(&ctype, mutability)
                    .into_owned()
                    .into()
            }
        };

        // We don't append a header for this, since we already have a forward.
        // Note that we also need a forward for the C type in case of structs. The forward handling manages this.
        self.decl_header
            .append_forward(def, &type_name_unnamespaced);
        if (self.generating_struct_fields && op.owner.is_owned()) || using_opaque_ref {
            // Owned struct fields hold `Foo` by value, and `FooRef`/`FooRefMut` (used for every
            // other borrowed, non-param position) are aliases declared inside `Foo.d.hpp` itself
            // -- neither can work off just a forward declaration of `Foo`, so both need the decl
            // header to fully include `Foo.d.hpp` (which already has the full class body --
            // methods are declared but not defined there -- plus the aliases).
            self.decl_header
                .includes
                .insert(self.formatter.fmt_decl_header_path(op_id.into()));
        }
        self.impl_header
            .includes
            .insert(self.formatter.fmt_impl_header_path(op_id.into()));
        ret
    }

    pub(crate) fn gen_fn_sig(&mut self, cb: &dyn CallbackInstantiationFunctionality) -> String {
        let t = cb.get_output_type().unwrap();

        let return_type = self.gen_cpp_return_type_name(t, false);

        let params_types = cb
            .get_inputs()
            .unwrap()
            .iter()
            .map(|p| self.gen_type_name(&p.ty, false).to_string())
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
        tuple_idx: Option<usize>,
        field: &'a hir::StructField<P>,
        namespace: Option<String>,
    ) -> NamedExpression<'a> {
        let var_name = self.formatter.fmt_param_name(field.name.as_str());
        // Tuple fields come from the caller-provided `std::tuple<...>` (by positional index);
        // non-tuple fields come from `this`'s own named member. Either way, once we have that
        // base expression, borrowed-opaque fields need the same `.AsFFI()` treatment (both
        // `Ref<Foo,capi::Foo>` and `Optional<Ref<Foo,capi::Foo>>` -- the latter for an optional
        // borrow -- expose `AsFFI()` directly, no ternary needed).
        let base: Cow<str> = if let Some(idx) = tuple_idx {
            format!("std::get<{idx}>(tuple)").into()
        } else {
            var_name.clone()
        };
        let field_getter = if let Type::Opaque(op) = &field.ty {
            if op.owner.is_owned() {
                base
            } else {
                format!("{base}.AsFFI()").into()
            }
        } else {
            base
        };
        let expression =
            self.gen_cpp_to_c_for_type(&field.ty, field_getter, None, namespace, false);

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
        is_param: bool,
    ) -> Cow<'a, str> {
        let lib_name_ns_prefix = &self.formatter.lib_name_ns_prefix;
        match *ty {
            Type::Primitive(..) => cpp_name.clone(),
            // `Foo`/`Optional<Foo>` are plain values (not `std::unique_ptr`), so this is member
            // access, not `->` -- and both expose `AsFFI()` directly (`Optional<Foo>::AsFFI()`
            // returns nullptr when empty), so the optional and non-optional cases need no ternary.
            Type::Opaque(ref op) if op.owner.is_owned() => {
                if is_param {
                    format!("{cpp_name}.AsFFI()").into()
                } else {
                    // Field position: this runs inside a struct's `AsFFI() const`, so `cpp_name`
                    // is const-qualified, but the C struct field needs the mutable pointer to
                    // hand ownership back to Rust — const_cast to reach the non-const AsFFI().
                    let type_name = self.formatter.fmt_type_name(op.tcx_id.into());
                    let full_type_name: Cow<str> = if op.is_optional() {
                        self.formatter.fmt_optional(&type_name).into()
                    } else {
                        type_name
                    };
                    format!("const_cast<{full_type_name}&>({cpp_name}).AsFFI()").into()
                }
            }
            // Borrowed, parameter position: `cpp_name` is a real `Foo&`/`const Foo&` (non-optional)
            // or `Optional<FooRef>`/`Optional<FooRefMut>` (optional, since a reference itself can't
            // be optional -- see gen_opaque_name), and both expose `AsFFI()` directly, same as the
            // owned-opaque arm above.
            Type::Opaque(_) if is_param => format!("{cpp_name}.AsFFI()").into(),
            // Borrowed, non-parameter position (field/return/callback): `cpp_name` is already the
            // raw `capi::Foo*`/`const capi::Foo*` pointer -- either because gen_opaque_name itself
            // produces one directly, or because the caller (e.g. gen_cpp_to_c_for_field) already
            // called `.AsFFI()` on the `Ref`/`Optional<Ref>` before reaching here -- so no
            // conversion is needed.
            Type::Opaque(..) => cpp_name,
            Type::Struct(ref s) => {
                let attrs = match self.c.tcx.resolve_type(s.id()) {
                    TypeDef::OutStruct(s) => &s.attrs,
                    TypeDef::Struct(s) => &s.attrs,
                    _ => unreachable!(),
                };

                if attrs.tuple && matches!(s.owner(), MaybeOwn::Own) {
                    let type_name = self.formatter.fmt_type_name(s.id());
                    return format!("{type_name}::AsTupleFFI({cpp_name})").into();
                }

                if attrs.abi_compatible {
                    if let MaybeOwn::Borrow(borrow) = s.owner() {
                        let c_name = self.formatter.namespace_c_name(
                            s.id().into(),
                            &self.formatter.fmt_type_name_unnamespaced(s.id()),
                        );
                        return match borrow.mutability {
                            Mutability::Immutable => {
                                format!("reinterpret_cast<const {c_name}*>(&{cpp_name})")
                            }
                            Mutability::Mutable => {
                                format!("reinterpret_cast<{c_name}*>(&{cpp_name})")
                            }
                        }
                        .into();
                    }
                }
                format!("{cpp_name}.AsFFI()").into()
            }
            Type::Enum(..) => format!("{cpp_name}.AsFFI()").into(),
            Type::Slice(Slice::Strs(encoding)) => {
                // This cast is valid as diplomat::string_view_for_slice is used to ensure correct layout
                let str_view = self.c.formatter.fmt_str_view_name(encoding);
                format!(
                    "{{reinterpret_cast<const {str_view}*>({cpp_name}.data()), {cpp_name}.size()}}"
                )
                .into()
            }
            Type::Slice(Slice::Struct(b, ref st)) => {
                let mutability = if b.mutability().is_mutable() {
                    ""
                } else {
                    "const "
                };
                let c_name = self.formatter.namespace_c_name(
                    st.id().into(),
                    &self.formatter.fmt_type_name_unnamespaced(st.id()),
                );
                format!(
                    "{{reinterpret_cast<{mutability}{c_name}*>({cpp_name}.data()), {cpp_name}.size()}}",

                )
                .into()
            }
            // The C ABI's DiplomatFooView struct always declares `data` as `const capi::Foo**`
            // (mutable outer pointer, const-qualified pointee), regardless of slice mutability.
            // The span container itself is generated non-`const` (see gen_type_name), so
            // `.data()` is a non-`const`-qualified `FooRef*`/`FooRefMut*`/`Optional<FooRef>*`/
            // `Optional<FooRefMut>*` here (all four single-pointer, ABI-compatible with a
            // (possibly-null) `capi::Foo*`); reinterpret_cast is legal since neither that source
            // pointee nor `const capi::Foo*` (the target's pointee) is itself top-level
            // cv-qualified.
            Type::Slice(Slice::Opaque(_, ref op)) => format!(
                "{{reinterpret_cast<const {}**>({cpp_name}.data()), {cpp_name}.size()}}",
                self.formatter.namespace_c_name(
                    op.id().into(),
                    &self.formatter.fmt_type_name_unnamespaced(op.id())
                )
            )
            .into(),
            Type::Slice(..) => format!("{{{cpp_name}.data(), {cpp_name}.size()}}").into(),
            Type::DiplomatOption(ref inner) => {
                // `.value()` is non-const (matching `Ref`/`Optional`'s pointer-like
                // specializations elsewhere -- see gen_opaque_name's doc comment), so field
                // position (running inside a struct's `AsFFI() const`, where `cpp_name` is
                // const-qualified) needs the same const_cast dance as owned-opaque fields do.
                let value_access: Cow<str> = if is_param {
                    format!("{cpp_name}.value()").into()
                } else {
                    let inner_type_name = self.gen_type_name(inner, is_param);
                    let optional_type_name = self.formatter.fmt_optional(&inner_type_name);
                    format!("const_cast<{optional_type_name}&>({cpp_name}).value()").into()
                };
                let conversion = self.gen_cpp_to_c_for_type(
                    inner,
                    value_access,
                    method_abi_name,
                    namespace,
                    is_param,
                );
                let copt = self.c.gen_ty_name(ty, &mut Default::default());
                format!("{cpp_name}.has_value() ? ({copt}{{ {{ {conversion} }}, true }}) : ({copt}{{ {{}}, false }})").into()
            }
            Type::Callback(ref c) => {
                // The ok/err/success types computed below are explicit template arguments for
                // c_run_callback_result/c_run_callback_diplomat_option, and must exactly match
                // the std::function's actual Ret (result<T,E>/optional<T>) that gen_fn_sig
                // already produced for this same callback. Both go through the same
                // gen_type_name(_, false) codepath with no extra flags needed, so they agree by
                // construction.
                let run_callback = match c.get_output_type().unwrap() {
                    ReturnType::Fallible(ref ok, ref err) => {
                        let ok_type_name = match ok {
                            hir::SuccessType::Unit => "std::monostate".into(),
                            hir::SuccessType::OutType(o) => self.gen_type_name(o, false),
                            _ => unreachable!("unknown AST/HIR variant"),
                        };

                        let err_type_name = match err {
                            Some(o) => self.gen_type_name(o, false),
                            None => "std::monostate".into(),
                        };

                        let return_type = self.formatter.fmt_c_api_callback_ret(
                            namespace,
                            method_abi_name.unwrap(),
                            &cpp_name,
                        );

                        self.formatter.fmt_run_callback_converter(
                            &cpp_name,
                            "c_run_callback_result",
                            vec![&ok_type_name, &err_type_name, &return_type],
                        )
                    }
                    ReturnType::Nullable(ref success) => {
                        let type_name = match success {
                            hir::SuccessType::Unit => "std::monostate".into(),
                            hir::SuccessType::OutType(o) => self.gen_type_name(o, false),
                            _ => unreachable!("unknown AST/HIR variant"),
                        };

                        let return_type = self.formatter.fmt_c_api_callback_ret(
                            namespace,
                            method_abi_name.unwrap(),
                            &cpp_name,
                        );
                        self.formatter.fmt_run_callback_converter(
                            &cpp_name,
                            "c_run_callback_diplomat_option",
                            vec![&type_name, &return_type],
                        )
                    }
                    ReturnType::Infallible(SuccessType::OutType(Type::Opaque(o))) => {
                        let opaque_type = self
                            .c
                            .formatter
                            .fmt_type_name_maybe_namespaced(o.tcx_id.into());
                        let ptr_ty = self.c.formatter.fmt_ptr(&opaque_type, o.owner.mutability);
                        self.formatter.fmt_run_callback_converter(
                            &cpp_name,
                            "c_run_callback_diplomat_opaque",
                            vec![&ptr_ty],
                        )
                    }
                    _ => format!(
                        "{lib_name_ns_prefix}diplomat::fn_traits({cpp_name}).c_run_callback"
                    ),
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
            ReturnType::Infallible(SuccessType::OutType(ref o)) => self.gen_type_name(o, false),
            ReturnType::Fallible(ref ok, ref err) => {
                let ok_type_name = match ok {
                    SuccessType::Write if is_generic_write => "std::monostate".into(),
                    SuccessType::Write => self.formatter.fmt_owned_str(),
                    SuccessType::Unit => "std::monostate".into(),
                    SuccessType::OutType(o) => self.gen_type_name(o, false),
                    _ => unreachable!("unknown AST/HIR variant"),
                };
                let err_type_name = match err {
                    Some(o) => self.gen_type_name(o, false),
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
                    SuccessType::OutType(o) => self.gen_type_name(o, false),
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
        // Borrowed-opaque fields (tuple element or not) are Ref<Foo,const capi::Foo>/
        // Ref<Foo,capi::Foo>, reconstructed via their own FromFFI -- gen_c_to_cpp_for_type
        // already handles this uniformly for every non-param position, tuple or not.
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
        let lib_name_ns_prefix = &self.formatter.lib_name_ns_prefix;

        match *ty {
            Type::Primitive(..) => var_name,
            // `Foo`/`Optional<Foo>` both have their own `FromFFI` (the latter treating a null
            // pointer as empty), so no ternary is needed regardless of optionality.
            // Note: The impl file is imported in gen_type_name().
            Type::Opaque(ref op) if op.owner.is_owned() => {
                let type_name = self.formatter.fmt_type_name(op.tcx_id.into());
                let full_type_name: Cow<str> = if op.is_optional() {
                    self.formatter.fmt_optional(&type_name).into()
                } else {
                    type_name
                };
                format!("{full_type_name}::FromFFI({var_name})").into()
            }
            // Borrowed: this function is only ever used at return/field position (this is the
            // "C to C++" direction; parameters go the other way, via gen_cpp_to_c_for_type).
            // Matches gen_opaque_name's declared type for the same position: Ref (`FooRef`/
            // `FooRefMut`), or `Optional<Ref<...>>` for an optional borrow -- both reconstructed
            // via their own FromFFI, no ternary needed. (Tuple-struct fields are the one
            // borrowed-opaque, non-param position that stays a raw pointer -- gen_c_to_cpp_for_field
            // handles those directly without delegating here, since it has the `is_tuple` context
            // this function doesn't.)
            Type::Opaque(ref op) => {
                let type_name = self.formatter.fmt_type_name(op.tcx_id.into());
                let mutability = op.owner.mutability().unwrap_or(hir::Mutability::Mutable);
                let ref_type = self.formatter.fmt_opaque_ref(&type_name, mutability);
                let full_type_name: Cow<str> = if op.is_optional() {
                    self.formatter.fmt_optional(&ref_type).into()
                } else {
                    ref_type.into()
                };
                format!("{full_type_name}::FromFFI({var_name})").into()
            }
            Type::Struct(ref st) => {
                let (is_zst, is_tuple) = match self.c.tcx.resolve_type(ty.id().unwrap()) {
                    TypeDef::Struct(s) => (s.fields.is_empty(), s.attrs.tuple),
                    TypeDef::OutStruct(s) => (s.fields.is_empty(), s.attrs.tuple),
                    _ => (false, false),
                };

                let id = st.id();
                let type_name = self.formatter.fmt_type_name(id);

                if is_tuple && matches!(st.owner(), MaybeOwn::Own) {
                    format!("{type_name}::TupleFromFFI({var_name})").into()
                } else if is_zst {
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
                // `gen_c_to_cpp_for_type` is `&self` (no side-effecting include/forward-declare
                // tracking), so the inner type's name isn't available here the way it is in
                // `gen_c_to_cpp_for_return_type` (which computes it anyway as part of the return
                // type). `decltype` sidesteps needing it: it's never evaluated, so repeating
                // `conversion`'s text inside it is free, and it gives the exact `Optional<T>` type
                // CTAD would have deduced for the value branch, for the `Optional<T>(std::nullopt)`
                // branch to construct explicitly.
                let conversion = self.gen_c_to_cpp_for_type(inner, format!("{var_name}.ok").into());
                format!("{var_name}.is_ok ? {lib_name_ns_prefix}diplomat::Optional({conversion}) : decltype({lib_name_ns_prefix}diplomat::Optional({conversion}))(std::nullopt)").into()
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
                    SuccessType::OutType(ref o) => self.gen_type_name(o, false),
                    _ => unreachable!("unknown AST/HIR variant"),
                };
                let err_type_name = match err {
                    Some(o) => self.gen_type_name(o, false),
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
                    SuccessType::OutType(o) => self.gen_type_name(o, false),
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

                let optional_type_name = self.formatter.fmt_optional(&type_name);
                Some(format!("{var_name}.is_ok ? {optional_type_name}({conversion}) : {optional_type_name}(std::nullopt)").into())
            }
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }
}
