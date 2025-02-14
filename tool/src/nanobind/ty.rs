use super::binding::Binding;
use super::PyFormatter;
use crate::c::Header as C2Header;
use crate::c::TyGenContext as C2TyGenContext;
use crate::ErrorStore;
use askama::Template;
use diplomat_core::hir::{
    self, EnumVariant, Mutability, OpaqueOwner, ReturnType, StructPathLike, SuccessType,
    TyPosition, Type, TypeId,
};
use std::borrow::Cow;
use std::collections::HashSet;

/// A type name with a corresponding variable name, such as a struct field or a function parameter.
struct NamedType<'a> {
    var_name: Cow<'a, str>,
    _type_name: Cow<'a, str>,
}

/// Everything needed for rendering a method.
struct MethodInfo<'a> {
    /// HIR of the method being rendered
    method: &'a hir::Method,
    /// The C++ return type
    _return_ty: Cow<'a, str>,
    /// The C++ method name
    method_name: Cow<'a, str>,
    /// The C method name
    _abi_name: String,
    /// Qualifiers for the function that come before the declaration (like "static")
    pre_qualifiers: Vec<Cow<'a, str>>,
    /// Qualifiers for the function that come after the declaration (like "const")
    _post_qualifiers: Vec<Cow<'a, str>>,
    /// Type declarations for the C++ parameters
    _param_decls: Vec<NamedType<'a>>,
    /// Parameter validations, such as string checks
    _param_validations: Vec<String>,
}

/// Context for generating a particular type's impl
/// 'tcx refers to the lifetime of the typecontext
/// 'cx refers to the lifetime of the context itself
pub(super) struct TyGenContext<'cx, 'tcx> {
    pub formatter: &'cx PyFormatter<'tcx>,
    pub errors: &'cx ErrorStore<'tcx, String>,
    pub c2: C2TyGenContext<'cx, 'tcx>,
    pub binding: &'cx mut Binding<'tcx>,
    pub submodules: &'cx mut HashSet<Cow<'tcx, str>>,
    /// Are we currently generating struct fields?
    pub generating_struct_fields: bool,
}

impl<'ccx, 'tcx: 'ccx, 'bind> TyGenContext<'ccx, 'tcx> {
    /// Checks for & outputs a list of modules with their parents that still need to be defined for this type
    pub fn get_module_defs(
        &mut self,
        id: TypeId,
        _docstring: Option<&str>,
    ) -> Vec<(Cow<'tcx, str>, Cow<'tcx, str>)> {
        let mut namespaces = self.formatter.fmt_namespaces(id);
        let mut modules: Vec<(Cow<'_, str>, Cow<'_, str>)> = Default::default();

        let mut parent = self.binding.module_name.clone();
        while let Some(module) = namespaces.next() {
            if self.submodules.contains(module) {
                continue;
            }
            println!("Adding submodule entry for {module}");
            self.submodules.insert(module.into());

            modules.push((module.into(), parent));
            parent = module.into();
        }
        modules
    }

    pub fn get_module(&mut self, id: TypeId) -> String {
        self.formatter
            .fmt_module(id, &self.binding.module_name)
            .into_owned()
    }
    /// Adds an enum definition to the current implementation.
    ///
    /// The enum is defined in C++ using a `class` with a single private field that is the
    /// C enum type. This enables us to add methods to the enum and generally make the enum
    /// behave more like an upgraded C++ type. We don't use `enum class` because methods
    /// cannot be added to it.
    pub fn gen_enum_def(&mut self, ty: &'tcx hir::EnumDef, id: TypeId) {
        let type_name = self.formatter.fmt_type_name(id);
        let ctype = self.formatter.fmt_c_type_name(id);

        let values = ty
            .variants
            .iter()
            .map(|e| self.formatter.fmt_enum_variant(e))
            .collect::<Vec<_>>();

        #[derive(Template)]
        #[template(path = "nanobind/enum_impl.cpp.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            _ty: &'a hir::EnumDef,
            _fmt: &'a PyFormatter<'a>,
            type_name: &'a str,
            _ctype: &'a str,
            values: Vec<Cow<'a, str>>,
            module: &'a str,
            modules: Vec<(Cow<'a, str>, Cow<'a, str>)>,
        }

        ImplTemplate {
            _ty: ty,
            _fmt: self.formatter,
            type_name: &type_name,
            _ctype: &ctype,
            values: values,
            module: &self.get_module(id),
            modules: self.get_module_defs(id, None),
        }
        .render_into(self.binding)
        .unwrap();
    }

    pub fn gen_opaque_def(&mut self, ty: &'tcx hir::OpaqueDef, id: TypeId) {
        let type_name = self.formatter.fmt_type_name(id);
        let type_name_unnamespaced = self.formatter.fmt_type_name_unnamespaced(id);
        let ctype = self.formatter.fmt_c_type_name(id);
        let _dtor_name = self
            .formatter
            .namespace_c_method_name(id, ty.dtor_abi_name.as_str());

        let c_header = self.c2.gen_opaque_def(ty);

        let methods = ty
            .methods
            .iter()
            .flat_map(|method| self.gen_method_info(id, method))
            .collect::<Vec<_>>();

        #[derive(Template)]
        #[template(path = "nanobind/opaque_impl.cpp.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            // ty: &'a hir::OpaqueDef,
            fmt: &'a PyFormatter<'a>,
            type_name: &'a str,
            ctype: &'a str,
            methods: &'a [MethodInfo<'a>],
            modules: Vec<(Cow<'a, str>, Cow<'a, str>)>,
            module: Cow<'a, str>,
            type_name_unnamespaced: &'a str,
            _c_header: C2Header,
        }

        ImplTemplate {
            // ty,
            fmt: self.formatter,
            type_name: &type_name,
            ctype: &ctype,
            methods: methods.as_slice(),
            modules: self.get_module_defs(id, None),
            module: self.get_module(id).into(),
            type_name_unnamespaced: &type_name_unnamespaced,
            _c_header: c_header,
        }
        .render_into(self.binding)
        .unwrap();
    }

    pub fn gen_struct_def<P: TyPosition>(&mut self, def: &'tcx hir::StructDef<P>, id: TypeId) {
        let type_name = self.formatter.fmt_type_name(id);
        let type_name_unnamespaced = self.formatter.fmt_type_name_unnamespaced(id);
        let ctype = self.formatter.fmt_c_type_name(id);

        let c_header = self.c2.gen_struct_def(def);
        let _c_impl_header = self.c2.gen_impl(def.into());

        self.generating_struct_fields = true;
        let field_decls = def
            .fields
            .iter()
            .map(|field| self.gen_ty_decl(&field.ty, field.name.as_str()))
            .collect::<Vec<_>>();
        self.generating_struct_fields = false;

        let methods = def
            .methods
            .iter()
            .flat_map(|method| self.gen_method_info(id, method))
            .collect::<Vec<_>>();

        #[derive(Template)]
        #[template(path = "nanobind/struct_impl.cpp.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            type_name: &'a str,
            _ctype: &'a str,
            fields: &'a [NamedType<'a>],
            methods: &'a [MethodInfo<'a>],
            modules: Vec<(Cow<'a, str>, Cow<'a, str>)>,
            module: Cow<'a, str>,
            type_name_unnamespaced: &'a str,
            _c_header: C2Header,
        }

        ImplTemplate {
            type_name: &type_name,
            _ctype: &ctype,
            fields: field_decls.as_slice(),
            methods: methods.as_slice(),
            modules: self.get_module_defs(id, None),
            module: self.get_module(id).into(),
            type_name_unnamespaced: &type_name_unnamespaced,
            _c_header: c_header,
        }
        .render_into(self.binding)
        .unwrap();
    }

    fn gen_method_info(
        &mut self,
        id: TypeId,
        method: &'tcx hir::Method,
    ) -> Option<MethodInfo<'ccx>> {
        if method.attrs.disable {
            return None;
        }
        let _guard = self.errors.set_context_method(
            self.c2.tcx.fmt_type_name_diagnostics(id),
            method.name.as_str().into(),
        );
        let method_name = self.formatter.fmt_method_name(method);
        let abi_name = self
            .formatter
            .namespace_c_method_name(id, method.abi_name.as_str());
        let mut param_decls = Vec::new();

        let mut returns_utf8_err = false;

        for param in method.params.iter() {
            let decls = self.gen_ty_decl(&param.ty, param.name.as_str());
            param_decls.push(decls);
            if matches!(
                param.ty,
                Type::Slice(hir::Slice::Str(_, hir::StringEncoding::Utf8))
            ) {
                returns_utf8_err = true;
            }
        }

        let mut return_ty = self.gen_cpp_return_type_name(&method.output);

        if returns_utf8_err {
            return_ty = "diplomat::result<std::monostate, diplomat::Utf8Error>".into();
        };

        let pre_qualifiers = if method.param_self.is_none() {
            vec!["static".into()]
        } else {
            vec![]
        };

        let post_qualifiers = match &method.param_self {
            Some(param_self) if param_self.ty.is_immutably_borrowed() => vec!["const".into()],
            Some(_) => vec![],
            None => vec![],
        };

        Some(MethodInfo {
            method,
            _return_ty: return_ty,
            method_name,
            _abi_name: abi_name,
            pre_qualifiers,
            _post_qualifiers: post_qualifiers,
            _param_decls: param_decls,
            _param_validations: Default::default(),
        })
    }

    /// Generates C++ code for referencing a particular type with a given name.
    fn gen_ty_decl<'a, P: TyPosition>(&mut self, ty: &Type<P>, var_name: &'a str) -> NamedType<'a>
    where
        'ccx: 'a,
    {
        let var_name = self.formatter.fmt_param_name(var_name);
        let type_name = self.gen_type_name(ty);

        NamedType {
            var_name,
            _type_name: type_name,
        }
    }

    /// Generates Python code for referencing a particular type.
    ///
    /// This function adds the necessary type imports to the decl and impl files.
    fn gen_type_name<P: TyPosition>(&mut self, ty: &Type<P>) -> Cow<'ccx, str> {
        match *ty {
            Type::Primitive(prim) => self.formatter.fmt_primitive_as_c(prim),
            Type::Opaque(ref op) => {
                let op_id = op.tcx_id.into();
                let type_name = self.formatter.fmt_type_name(op_id);
                let _type_name_unnamespaced = self.formatter.fmt_type_name_unnamespaced(op_id);
                let def = self.c2.tcx.resolve_type(op_id);

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

                self.binding
                    .includes
                    .insert(self.formatter.fmt_impl_file_path(op_id).into());
                ret
            }
            Type::Struct(ref st) => {
                let id = st.id();
                let type_name = self.formatter.fmt_type_name(id);
                let _type_name_unnamespaced = self.formatter.fmt_type_name_unnamespaced(id);
                let def = self.c2.tcx.resolve_type(id);
                if def.attrs().disable {
                    self.errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }

                self.binding
                    .includes
                    .insert(self.formatter.fmt_impl_file_path(id).into());
                type_name
            }
            Type::Enum(ref e) => {
                let id = e.tcx_id.into();
                let type_name = self.formatter.fmt_type_name(id);
                let _type_name_unnamespaced = self.formatter.fmt_type_name_unnamespaced(id);
                let def = self.c2.tcx.resolve_type(id);
                if def.attrs().disable {
                    self.errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }

                self.binding
                    .includes
                    .insert(self.formatter.fmt_impl_file_path(id).into());
                type_name
            }
            Type::Slice(hir::Slice::Str(_, encoding)) => self.formatter.fmt_borrowed_str(encoding),
            Type::Slice(hir::Slice::Primitive(b, p)) => {
                let ret = self.formatter.fmt_primitive_as_c(p);
                let ret = self.formatter.fmt_borrowed_slice(
                    &ret,
                    b.map(|b| b.mutability).unwrap_or(hir::Mutability::Mutable),
                );
                ret.into_owned().into()
            }
            Type::Slice(hir::Slice::Strs(encoding)) => format!(
                "diplomat::span<const {}>",
                self.formatter.fmt_borrowed_str(encoding)
            )
            .into(),
            Type::DiplomatOption(ref inner) => {
                format!("std::optional<{}>", self.gen_type_name(inner)).into()
            }
            Type::Callback(..) => "".into(),
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    /// Generates the C++ type name of a return type.
    fn gen_cpp_return_type_name(&mut self, result_ty: &ReturnType) -> Cow<'ccx, str> {
        match *result_ty {
            ReturnType::Infallible(SuccessType::Unit) => "void".into(),
            ReturnType::Infallible(SuccessType::Write) => self.formatter.fmt_owned_str(),
            ReturnType::Infallible(SuccessType::OutType(ref o)) => self.gen_type_name(o),
            ReturnType::Fallible(ref ok, ref err) => {
                let ok_type_name = match ok {
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
}
