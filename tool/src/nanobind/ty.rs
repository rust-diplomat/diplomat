use super::{binding::Binding, PyFormatter};
use crate::{c::TyGenContext as C2TyGenContext, hir, ErrorStore};
use askama::Template;
use diplomat_core::hir::{OpaqueOwner, StructPathLike, TyPosition, Type, TypeId};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::collections::HashSet;

/// A type name with a corresponding variable name, such as a struct field or a function parameter.
#[derive(Clone)]
struct NamedType<'a> {
    var_name: Cow<'a, str>,
    type_name: Cow<'a, str>,
}

/// Everything needed for rendering a method.
struct MethodInfo<'a> {
    /// HIR of the method being rendered
    method: &'a hir::Method,
    /// The python method name
    method_name: Cow<'a, str>,
    /// The C++ method name. May differ due to keyword renaming or other constraints.
    cpp_method_name: Cow<'a, str>,
    // def statement to use - def, def_static, def_prop_ro, etc
    def: String,
    /// The property name, if any
    prop_name: Option<Cow<'a, str>>,
    // If this is a property, this is the associated setter's c++ method name
    setter_name: Option<Cow<'a, str>>,
    // In the *rare* event that they're required, the C++ names & types of the function params
    param_decls: Option<Vec<NamedType<'a>>>,
    // The lifetime annotation required for the method, if any. May be keep_alive<...> or reference_internal
    // Everything else is handled by the automatic behavior depending on return type.
    lifetime_args: Option<Cow<'a, str>>,
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

impl<'ccx, 'tcx: 'ccx> TyGenContext<'ccx, 'tcx> {
    /// Checks for & outputs a list of modules with their parents that still need to be defined for this type
    pub fn get_module_defs(
        &mut self,
        id: TypeId,
        _docstring: Option<&str>,
    ) -> Vec<(Cow<'tcx, str>, Cow<'tcx, str>)> {
        let namespaces = self.formatter.fmt_namespaces(id);
        let mut modules: Vec<(Cow<'_, str>, Cow<'_, str>)> = Default::default();

        let mut parent = self.binding.module_name.clone();
        for module in namespaces {
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
        let type_name = self.formatter.cxx.fmt_type_name(id);
        let type_name_unnamespaced = self.formatter.cxx.fmt_type_name_unnamespaced(id);

        let values = ty
            .variants
            .iter()
            .map(|e| self.formatter.cxx.fmt_enum_variant(e))
            .collect::<Vec<_>>();

        #[derive(Template)]
        #[template(path = "nanobind/enum_impl.cpp.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            type_name: &'a str,
            values: Vec<Cow<'a, str>>,
            module: &'a str,
            modules: Vec<(Cow<'a, str>, Cow<'a, str>)>,
            type_name_unnamespaced: &'a str,
        }

        ImplTemplate {
            type_name: &type_name,
            values,
            module: &self.get_module(id),
            modules: self.get_module_defs(id, None),
            type_name_unnamespaced: &type_name_unnamespaced,
        }
        .render_into(self.binding)
        .unwrap();
    }

    pub fn gen_opaque_def(&mut self, ty: &'tcx hir::OpaqueDef, id: TypeId) {
        let type_name = self.formatter.cxx.fmt_type_name(id);
        let type_name_unnamespaced = self.formatter.cxx.fmt_type_name_unnamespaced(id);

        let methods = self.gen_all_method_infos(id, ty.methods.iter());

        #[derive(Template)]
        #[template(path = "nanobind/opaque_impl.cpp.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            type_name: &'a str,
            methods: &'a [MethodInfo<'a>],
            modules: Vec<(Cow<'a, str>, Cow<'a, str>)>,
            module: Cow<'a, str>,
            type_name_unnamespaced: &'a str,
        }

        ImplTemplate {
            type_name: &type_name,
            methods: methods.as_slice(),
            modules: self.get_module_defs(id, None),
            module: self.get_module(id).into(),
            type_name_unnamespaced: &type_name_unnamespaced,
        }
        .render_into(self.binding)
        .unwrap();
    }

    pub fn gen_struct_def<P: TyPosition>(&mut self, def: &'tcx hir::StructDef<P>, id: TypeId) {
        let type_name = self.formatter.cxx.fmt_type_name(id);
        let type_name_unnamespaced = self.formatter.cxx.fmt_type_name_unnamespaced(id);

        self.generating_struct_fields = true;
        let field_decls = def
            .fields
            .iter()
            .map(|field| self.gen_ty_decl(&field.ty, field.name.as_str()))
            .collect::<Vec<_>>();
        self.generating_struct_fields = false;

        let methods = self.gen_all_method_infos(id, def.methods.iter());

        #[derive(Template)]
        #[template(path = "nanobind/struct_impl.cpp.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            type_name: &'a str,
            fields: &'a [NamedType<'a>],
            methods: &'a [MethodInfo<'a>],
            modules: Vec<(Cow<'a, str>, Cow<'a, str>)>,
            module: Cow<'a, str>,
            type_name_unnamespaced: &'a str,
            has_constructor: bool,
        }

        ImplTemplate {
            type_name: &type_name,
            fields: field_decls.as_slice(),
            methods: methods.as_slice(),
            modules: self.get_module_defs(id, None),
            module: self.get_module(id).into(),
            type_name_unnamespaced: &type_name_unnamespaced,
            has_constructor: methods.iter().any(|v| {
                matches!(
                    v.method.attrs.special_method,
                    Some(hir::SpecialMethod::Constructor)
                )
            }),
        }
        .render_into(self.binding)
        .unwrap();
    }

    fn gen_all_method_infos(
        &mut self,
        id: TypeId,
        methods: std::slice::Iter<'tcx, hir::Method>,
    ) -> Vec<MethodInfo<'ccx>> {
        // BTree map ensures that the output will be sorted by method name for more consistent codegen output.
        let mut method_infos = BTreeMap::<String, MethodInfo>::new();

        for method in methods {
            if let Some(info) = self.gen_method_info(id, method) {
                method_infos
                    // Use the property name as the key if present so we can collapse getters & setters
                    .entry(info.prop_name.clone().unwrap_or(info.method_name.clone()).to_string())
                    .and_modify(|e| {
                        // If the entry already exists...
                        match method.attrs.special_method {
                            Some(hir::SpecialMethod::Getter(_)) => {
                                assert!(
                                    matches!(e.method.attrs.special_method, Some(hir::SpecialMethod::Setter(_))),
                                    "Method Info entry for {} already exists, but isn't a setter.",
                                    e.method_name
                                );
                                // If the entry previously associated with this was the setter,
                                // overwrite the method reference & method name with this getter
                                e.method = method;
                                e.method_name = info.method_name.clone();
                                e.cpp_method_name = info.cpp_method_name.clone();
                            }
                            Some(hir::SpecialMethod::Setter(_)) => {
                                assert!(
                                    matches!(
                                        e.method.attrs.special_method,
                                        Some(hir::SpecialMethod::Getter(_))
                                    ),
                                    "Method Info entry for {} already exists, but isn't a getter.",
                                    e.method_name
                                );
                                e.setter_name = Some(info.method_name.clone());
                                e.def = info.def.clone(); // when a setter exists, use it's qualifiers instead.
                                e.param_decls = info.param_decls.clone(); // also it's params, since the getter has none by definition.
                            }
                            _ => { panic!("Method Info for {} already exists but isn't a getter or setter!", e.method_name); }
                        };
                    })
                    .or_insert(info);
            }
        }

        for info in method_infos.values() {
            if matches!(
                info.method.attrs.special_method,
                Some(hir::SpecialMethod::Setter(_))
            ) {
                self.errors.push_error(format!(
                    "Setter {} exists without a matching getter, which is not allowed by nanobind.",
                    info.method_name
                ));
            }
        }

        method_infos.into_values().collect()
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
        let cpp_method_name = self.formatter.cxx.fmt_method_name(method);
        let method_name = self.formatter.fmt_method_name(method);
        let mut setter_name = None;

        let mut def_qualifiers = vec!["def"];

        let mut prop_name = None;
        if let Some(hir::SpecialMethod::Getter(name)) = &method.attrs.special_method {
            def_qualifiers.extend(["prop_ro"]);
            prop_name = Some(
                name.as_ref()
                    .map(|v| v.into())
                    .unwrap_or(method_name.clone()),
            );
        } else if let Some(hir::SpecialMethod::Setter(name)) = &method.attrs.special_method {
            def_qualifiers.extend(["prop_rw"]);
            setter_name = Some(method_name.clone());
            prop_name = Some(
                name.as_ref()
                    .map(|v| v.into())
                    .unwrap_or(method_name.clone()),
            );
        }

        if method.param_self.is_none()
            && !matches!(
                method.attrs.special_method,
                Some(hir::SpecialMethod::Constructor) // Constructors weirdly don't use def_static
            )
        {
            def_qualifiers.extend(["static"]);
        }

        let param_decls = {
            if matches!(
                method.attrs.special_method,
                Some(hir::SpecialMethod::Constructor) | Some(hir::SpecialMethod::Setter(_)) // We only need type info for constructors or certain setters
            ) && !matches!(
                // and even then, only when the type isn't opaque
                id,
                TypeId::Opaque(_)
            ) {
                Some(
                    method
                        .params
                        .iter()
                        .map(|p| NamedType {
                            var_name: self.formatter.cxx.fmt_param_name(p.name.as_str()),
                            type_name: self.gen_type_name(&p.ty),
                        })
                        .collect(),
                )
            } else {
                None
            }
        };

        let mut visitor = method.borrowing_param_visitor(self.c2.tcx);

        // Collect all the relevant borrowed params, with self in position 1 if present
        let mut param_borrows = method
            .param_self
            .iter()
            .map(|s| visitor.visit_param(&s.ty.clone().into(), "self"))
            .collect::<Vec<_>>();
        // Must be a separate call *after* collect to avoid double-borrowing visitor
        param_borrows.extend(
            method
                .params
                .iter()
                .map(|p| visitor.visit_param(&p.ty, p.name.as_str())),
        );

        let self_number = if matches!(
            method.attrs.special_method,
            Some(hir::SpecialMethod::Constructor)
        ) {
            1 // per the docs, constructors don't have "returns", they act on "self"
        } else {
            0
        };

        let mut lifetime_args = param_borrows
            .into_iter()
            .enumerate()
            .filter_map(|(i, p)| match p {
                hir::borrowing_param::ParamBorrowInfo::BorrowedSlice
                | hir::borrowing_param::ParamBorrowInfo::Struct(_)
                | hir::borrowing_param::ParamBorrowInfo::BorrowedOpaque => {
                    Some(format!(
                        "nb::keep_alive<{self_number}, {}>()",
                        i + 1 + self_number
                    )) // Keep 0 (the return) alive until the element at P is returned
                }
                _ => None,
            })
            .collect::<Vec<_>>();

        if matches!(method.output.success_type(), hir::SuccessType::OutType(hir::Type::Opaque(path)) if !path.is_owned())
        {
            lifetime_args.push("nb::rv_policy::reference".to_owned());
        }

        Some(MethodInfo {
            method,
            method_name,
            cpp_method_name,
            def: def_qualifiers.join("_"),
            setter_name,
            prop_name,
            param_decls,
            lifetime_args: if lifetime_args.is_empty() {
                None
            } else {
                Some(lifetime_args.join(", ").into())
            },
        })
    }

    /// Generates C++ code for referencing a particular type with a given name.
    fn gen_ty_decl<'a, P: TyPosition>(&mut self, ty: &Type<P>, var_name: &'a str) -> NamedType<'a>
    where
        'ccx: 'a,
    {
        let var_name = self.formatter.cxx.fmt_param_name(var_name);
        let type_name = self.gen_type_name(ty);

        NamedType {
            var_name,
            type_name,
        }
    }

    /// Generates Python code for referencing a particular type.
    ///
    /// This function adds the necessary type imports to the decl and impl files.
    fn gen_type_name<P: TyPosition>(&mut self, ty: &Type<P>) -> Cow<'ccx, str> {
        match *ty {
            Type::Primitive(prim) => self.formatter.cxx.fmt_primitive_as_c(prim),
            Type::Opaque(ref op) => {
                let op_id = op.tcx_id.into();
                let type_name = self.formatter.cxx.fmt_type_name(op_id);
                let def = self.c2.tcx.resolve_type(op_id);

                if def.attrs().disable {
                    self.errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }
                let mutability = op.owner.mutability().unwrap_or(hir::Mutability::Mutable);
                let ret = match (op.owner.is_owned(), op.is_optional()) {
                    // unique_ptr is nullable
                    (true, _) => self.formatter.cxx.fmt_owned(&type_name),
                    (false, true) => self
                        .formatter
                        .cxx
                        .fmt_optional_borrowed(&type_name, mutability),
                    (false, false) => self.formatter.cxx.fmt_borrowed(&type_name, mutability),
                };
                let ret = ret.into_owned().into();

                self.binding
                    .includes
                    .insert(self.formatter.cxx.fmt_impl_header_path(op_id).into());
                ret
            }
            Type::Struct(ref st) => {
                let id = st.id();
                let type_name = self.formatter.cxx.fmt_type_name(id);
                let def = self.c2.tcx.resolve_type(id);
                if def.attrs().disable {
                    self.errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }

                self.binding
                    .includes
                    .insert(self.formatter.cxx.fmt_impl_header_path(id).into());
                type_name
            }
            Type::Enum(ref e) => {
                let id = e.tcx_id.into();
                let type_name = self.formatter.cxx.fmt_type_name(id);
                let def = self.c2.tcx.resolve_type(id);
                if def.attrs().disable {
                    self.errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }

                self.binding
                    .includes
                    .insert(self.formatter.cxx.fmt_impl_header_path(id).into());
                type_name
            }
            Type::Slice(hir::Slice::Str(_, encoding)) => {
                self.formatter.cxx.fmt_borrowed_str(encoding)
            }
            Type::Slice(hir::Slice::Primitive(b, p)) => {
                let ret = self.formatter.cxx.fmt_primitive_as_c(p);
                let ret = self.formatter.cxx.fmt_borrowed_slice(
                    &ret,
                    b.map(|b| b.mutability).unwrap_or(hir::Mutability::Mutable),
                );
                ret.into_owned().into()
            }
            Type::Slice(hir::Slice::Strs(encoding)) => format!(
                "diplomat::span<const {}>",
                self.formatter.cxx.fmt_borrowed_str(encoding)
            )
            .into(),
            Type::DiplomatOption(ref inner) => {
                format!("std::optional<{}>", self.gen_type_name(inner)).into()
            }
            Type::Callback(..) => "".into(),
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }
}
