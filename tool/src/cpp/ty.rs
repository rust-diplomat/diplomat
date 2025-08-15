use super::header::Header;
use super::Cpp2Formatter;
use crate::c::Header as C2Header;
use crate::c::TyGenContext as C2TyGenContext;
use crate::cpp::func::FuncGenContext;
use crate::cpp::func::MethodInfo;
use crate::ErrorStore;
use askama::Template;
use diplomat_core::hir::CallbackInstantiationFunctionality;
use diplomat_core::hir::Slice;
use diplomat_core::hir::{
    self, MaybeOwn, Mutability, OpaqueOwner, ReturnType, SelfType, StructPathLike, SuccessType,
    TyPosition, Type, TypeDef, TypeId,
};
use std::borrow::Cow;

use crate::c::CAPI_NAMESPACE;
use crate::filters;

/// An expression with a corresponding variable name, such as a struct field or a function parameter.
struct NamedExpression<'a> {
    var_name: Cow<'a, str>,
    expression: Cow<'a, str>,
}

/// A type name with a corresponding variable name, such as a struct field or a function parameter.
pub(super) struct NamedType<'a> {
    pub(super) var_name: Cow<'a, str>,
    pub(super) type_name: Cow<'a, str>,
}

/// Context for generating a particular type's header
pub(crate) struct TyGenContext<'ccx, 'tcx, 'header> {
    pub formatter: &'ccx Cpp2Formatter<'tcx>,
    pub errors: &'ccx ErrorStore<'tcx, String>,
    pub c: C2TyGenContext<'ccx, 'tcx>,
    pub impl_header: &'header mut Header,
    pub decl_header: &'header mut Header,
    /// Are we currently generating struct fields?
    pub generating_struct_fields: bool,
}

impl<'ccx, 'tcx: 'ccx> TyGenContext<'ccx, 'tcx, '_> {
    /// Adds an enum definition to the current decl and impl headers.
    ///
    /// The enum is defined in C++ using a `class` with a single private field that is the
    /// C enum type. This enables us to add methods to the enum and generally make the enum
    /// behave more like an upgraded C++ type. We don't use `enum class` because methods
    /// cannot be added to it.
    pub fn gen_enum_def(&mut self, ty: &'tcx hir::EnumDef, id: TypeId) {
        let type_name = self.formatter.fmt_type_name(id);
        let type_name_unnamespaced = self.formatter.fmt_type_name_unnamespaced(id);
        let ctype = self.formatter.fmt_c_type_name(id);
        let c_header = self.c.gen_enum_def(ty);
        let c_impl_header = self.c.gen_impl(ty.into());

        let methods = ty
            .methods
            .iter()
            .flat_map(|method| FuncGenContext::gen_method_info(id.into(), method, self))
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
            docs: &self.formatter.fmt_docs(&ty.docs),
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

    pub fn gen_opaque_def(&mut self, ty: &'tcx hir::OpaqueDef, id: TypeId) {
        let type_name = self.formatter.fmt_type_name(id);
        let type_name_unnamespaced = self.formatter.fmt_type_name_unnamespaced(id);
        let ctype = self.formatter.fmt_c_type_name(id);
        let dtor_name = self
            .formatter
            .namespace_c_name(id.into(), ty.dtor_abi_name.as_str());

        let c_header = self.c.gen_opaque_def(ty);
        let c_impl_header = self.c.gen_impl(ty.into());

        let methods = ty
            .methods
            .iter()
            .flat_map(|method| FuncGenContext::gen_method_info(id.into(), method, self))
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
            docs: &self.formatter.fmt_docs(&ty.docs),
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

    pub fn gen_struct_def<P: TyPosition>(&mut self, def: &'tcx hir::StructDef<P>, id: TypeId) {
        let type_name = self.formatter.fmt_type_name(id);
        let type_name_unnamespaced = self.formatter.fmt_type_name_unnamespaced(id);
        let ctype = self.formatter.fmt_c_type_name(id);

        let namespace = def.attrs.namespace.clone();

        let c_header = self.c.gen_struct_def(def);
        let c_impl_header = self.c.gen_impl(def.into());

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
            .flat_map(|method| FuncGenContext::gen_method_info(id.into(), method, self))
            .collect::<Vec<_>>();

        #[derive(Template)]
        #[template(path = "cpp/struct_decl.h.jinja", escape = "none")]
        struct DeclTemplate<'a> {
            // ty: &'a hir::OpaqueDef,
            // fmt: &'a Cpp2Formatter<'a>,
            type_name: &'a str,
            ctype: &'a str,
            fields: &'a [NamedType<'a>],
            methods: &'a [MethodInfo<'a>],
            namespace: Option<&'a str>,
            type_name_unnamespaced: &'a str,
            c_header: C2Header,
            is_sliceable: bool,
            docs: &'a str,
        }

        DeclTemplate {
            // ty,
            // fmt: &self.formatter,
            type_name: &type_name,
            ctype: &ctype,
            fields: field_decls.as_slice(),
            methods: methods.as_slice(),
            namespace: namespace.as_deref(),
            type_name_unnamespaced: &type_name_unnamespaced,
            c_header,
            is_sliceable: def.attrs.abi_compatible,
            docs: &self.formatter.fmt_docs(&def.docs),
        }
        .render_into(self.decl_header)
        .unwrap();

        #[derive(Template)]
        #[template(path = "cpp/struct_impl.h.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            // ty: &'a hir::OpaqueDef,
            // fmt: &'a Cpp2Formatter<'a>,
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
            // fmt: &self.formatter,
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
                "diplomat::span<const {}>",
                self.formatter.fmt_borrowed_str(encoding)
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
                // Layout of DiplomatStringView and std::string_view are guaranteed to be identical, otherwise this would be terrible
                "{{reinterpret_cast<const diplomat::capi::DiplomatStringView*>({cpp_name}.data()), {cpp_name}.size()}}"
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
                        let opaque_type = format!("diplomat::capi::{}", self.c.formatter.fmt_type_name(o.tcx_id.into()));
                        let ptr_ty = self.c.formatter.fmt_ptr(&opaque_type, o.owner.mutability);
                        self.formatter.fmt_run_callback_converter(&cpp_name, "c_run_callback_diplomat_opaque", vec![&ptr_ty])
                    },
                    _ => format!("diplomat::fn_traits({cpp_name}).c_run_callback")
                };
                format!("{{new decltype({cpp_name})(std::move({cpp_name})), {run_callback}, diplomat::fn_traits({cpp_name}).c_delete}}",).into()
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
                format!("diplomat::result<{ok_type_name}, {err_type_name}>").into()
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
                    format!("{var_name}.is_ok ? diplomat::result<{ok_type_name}, {err_type_name}>(diplomat::Ok<{ok_type_name}>({ok_conversion})) : diplomat::result<{ok_type_name}, {err_type_name}>(diplomat::Err<{err_type_name}>({err_conversion}))").into()
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
