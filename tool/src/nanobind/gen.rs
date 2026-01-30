use super::root_module::RootModule;
use super::PyFormatter;
use crate::config::Config;
use crate::read_custom_binding;
use crate::{cpp::ItemGenContext as CppItemGenContext, hir, ErrorStore};
use askama::Template;
use diplomat_core::hir::{
    IncludeLocation, IncludeSource, OpaqueOwner, StructPathLike, SymbolId, TyPosition, Type, TypeId,
};
use std::borrow::Cow;
use std::collections::BTreeSet;
use std::collections::{BTreeMap, HashMap};

#[derive(Clone)]
pub(super) struct ParamInfo<'a> {
    pub(super) params: Vec<NamedType<'a>>,
}

/// Everything needed for rendering a method.
pub(super) struct MethodInfo<'a> {
    /// HIR of the method being rendered
    pub(super) method: &'a hir::Method,
    /// The python method name
    pub(super) method_name: Cow<'a, str>,
    /// The C++ method name. May differ due to keyword renaming or other constraints.
    pub(super) cpp_method_name: Cow<'a, str>,
    // def statement to use - def, def_static, def_prop_ro, etc
    pub(super) def: String,
    /// The property name, if any
    pub(super) prop_name: Option<Cow<'a, str>>,
    // If this is a property, this is the associated setter's c++ method name
    pub(super) setter_name: Option<Cow<'a, str>>,
    /// The C++ names & types of the function params.
    /// Always required in case of overloading, and a few special methods.
    pub(super) param_decls: ParamInfo<'a>,
    // The lifetime annotation required for the method, if any. May be keep_alive<...> or reference_internal
    // Everything else is handled by the automatic behavior depending on return type.
    pub(super) lifetime_args: Option<Cow<'a, str>>,
    pub(super) overloads: Vec<ParamInfo<'a>>,
}

/// A type name with a corresponding variable name, such as a struct field or a function parameter.
#[derive(Clone)]
pub(super) struct NamedType<'a> {
    pub(super) var_name: Cow<'a, str>,
    pub(super) type_name: Cow<'a, str>,
}

/// Context for generating a particular type's impl
/// 'tcx refers to the lifetime of the typecontext
/// 'cx refers to the lifetime of the context itself
pub(super) struct ItemGenContext<'cx, 'tcx> {
    pub formatter: &'cx PyFormatter<'tcx>,
    pub errors: &'cx ErrorStore<'tcx, String>,
    pub cpp: CppItemGenContext<'cx, 'tcx, 'cx>,
    pub config: &'cx Config,
    pub root_module: &'cx mut RootModule<'tcx>,
    pub submodules: &'cx mut BTreeMap<Cow<'tcx, str>, BTreeSet<Cow<'tcx, str>>>,
    /// Are we currently generating struct fields?
    pub generating_struct_fields: bool,
}

impl<'ccx, 'tcx: 'ccx> ItemGenContext<'ccx, 'tcx> {
    /// Checks for & adds modules with their parents to the root module definition.
    pub fn gen_modules(&mut self, id: SymbolId, _docstring: Option<&str>) {
        let namespaces = self.formatter.fmt_namespaces(id);

        let mut parent = self.root_module.module_name.clone();
        for module in namespaces {
            println!("Adding {module} to parent {parent}");
            self.submodules
                .entry(parent)
                .or_default()
                .insert(module.into());
            parent = module.into();
        }
    }

    fn init_extra_code_from_attrs(
        &self,
        custom_extra_code: &HashMap<IncludeLocation, IncludeSource>,
    ) -> (String, String) {
        let extra_init_code =
            if let Some(s) = custom_extra_code.get(&IncludeLocation::InitializationBlock) {
                read_custom_binding(s, &self.config, &self.errors).unwrap_or_default()
            } else {
                Default::default()
            };

        let pre_extra_init_code =
            if let Some(s) = custom_extra_code.get(&IncludeLocation::PreInitializationBlock) {
                read_custom_binding(s, &self.config, &self.errors).unwrap_or_default()
            } else {
                Default::default()
            };

        (extra_init_code, pre_extra_init_code)
    }

    /// Adds an enum definition to the current implementation.
    ///
    /// The enum is defined in C++ using a `class` with a single private field that is the
    /// C enum type. This enables us to add methods to the enum and generally make the enum
    /// behave more like an upgraded C++ type. We don't use `enum class` because methods
    /// cannot be added to it.
    pub fn gen_enum_def<W: std::fmt::Write + ?Sized>(
        &mut self,
        ty: &'tcx hir::EnumDef,
        id: TypeId,
        out: &mut W,
    ) {
        let type_name = self.formatter.cxx.fmt_type_name(id);
        let type_name_unnamespaced = self.formatter.cxx.fmt_type_name_unnamespaced(id);

        let values = ty
            .variants
            .iter()
            .map(|e| self.formatter.cxx.fmt_enum_variant(e))
            .collect::<Vec<_>>();

        let (extra_init_code, pre_extra_init_code) =
            self.init_extra_code_from_attrs(&ty.attrs.custom_extra_code);

        #[derive(Template)]
        #[template(path = "nanobind/enum_impl.cpp.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            type_name: &'a str,
            values: Vec<Cow<'a, str>>,
            type_name_unnamespaced: &'a str,
            extra_init_code: String,
            pre_extra_init_code: String,
        }

        ImplTemplate {
            type_name: &type_name,
            values,
            type_name_unnamespaced: &type_name_unnamespaced,
            extra_init_code,
            pre_extra_init_code,
        }
        .render_into(out)
        .unwrap();

        self.add_to_root_module(id);
    }

    pub fn add_to_root_module(&mut self, id: TypeId) {
        self.gen_modules(id.into(), None);
        Self::gen_binding_fn(
            self.root_module,
            self.formatter.fmt_namespaces(id.into()),
            self.formatter.fmt_binding_fn(id),
        );
    }

    pub fn gen_binding_fn(
        root_module: &mut RootModule,
        namespaces: impl Iterator<Item = &'tcx str>,
        binding_fn_name_unnamespaced: String,
    ) {
        let ns_vec = namespaces.collect::<Vec<_>>();
        let namespace_prefix = ns_vec.join("::");
        root_module
            .fwd_decls
            .entry(namespace_prefix.clone())
            .or_default()
            .push(format!("void {binding_fn_name_unnamespaced}(nb::module_);"));

        let module_namespaces = ns_vec.iter().map(|s| s.to_string()).collect();

        let entry = root_module.module_fns.entry(module_namespaces).or_default();

        let namespaced_binding_fn = [
            ns_vec.as_slice(),
            [binding_fn_name_unnamespaced.as_str()].as_slice(),
        ]
        .concat()
        .join("::");
        entry.push(namespaced_binding_fn);
    }

    pub fn gen_opaque_def<W: std::fmt::Write + ?Sized>(
        &mut self,
        ty: &'tcx hir::OpaqueDef,
        id: TypeId,
        out: &mut W,
    ) {
        let _guard = self
            .errors
            .set_context_ty(self.cpp.c.tcx.fmt_symbol_name_diagnostics(id.into()));

        let type_name = self.formatter.cxx.fmt_type_name(id);
        let type_name_unnamespaced = self.formatter.cxx.fmt_type_name_unnamespaced(id);
        let methods = self.gen_all_method_infos(id, ty.methods.iter());

        let (extra_init_code, pre_extra_init_code) =
            self.init_extra_code_from_attrs(&ty.attrs.custom_extra_code);

        #[derive(Template)]
        #[template(path = "nanobind/opaque_impl.cpp.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            type_name: &'a str,
            methods: &'a [MethodInfo<'a>],
            type_name_unnamespaced: &'a str,
            extra_init_code: String,
            pre_extra_init_code: String,
        }

        ImplTemplate {
            type_name: &type_name,
            methods: methods.as_slice(),
            type_name_unnamespaced: &type_name_unnamespaced,
            extra_init_code,
            pre_extra_init_code,
        }
        .render_into(out)
        .unwrap();
        self.add_to_root_module(id);
    }

    pub fn gen_struct_def<P: TyPosition, W: std::fmt::Write + ?Sized>(
        &mut self,
        def: &'tcx hir::StructDef<P>,
        id: TypeId,
        out: &mut W,
        binding_prefix: &mut W,
    ) {
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

        self.gen_modules(id.into(), None);

        let (extra_init_code, pre_extra_init_code) =
            self.init_extra_code_from_attrs(&def.attrs.custom_extra_code);

        #[derive(Template)]
        #[template(path = "nanobind/struct_impl.cpp.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            type_name: &'a str,
            fields: &'a [NamedType<'a>],
            methods: &'a [MethodInfo<'a>],
            type_name_unnamespaced: &'a str,
            has_constructor: bool,
            is_sliceable: bool,
            extra_init_code: String,
            pre_extra_init_code: String,
        }

        if def.attrs.abi_compatible {
            write!(binding_prefix, "NB_MAKE_OPAQUE(std::vector<{type_name}>)")
                .expect("Could not write to header.");
        }

        ImplTemplate {
            type_name: &type_name,
            fields: field_decls.as_slice(),
            methods: methods.as_slice(),
            type_name_unnamespaced: &type_name_unnamespaced,
            has_constructor: methods.iter().any(|v| {
                matches!(
                    v.method.attrs.special_method,
                    Some(hir::SpecialMethod::Constructor)
                )
            }),
            is_sliceable: def.attrs.abi_compatible,
            extra_init_code,
            pre_extra_init_code,
        }
        .render_into(out)
        .unwrap();
        self.add_to_root_module(id);
    }

    fn gen_all_method_infos(
        &mut self,
        id: TypeId,
        methods: std::slice::Iter<'tcx, hir::Method>,
    ) -> Vec<MethodInfo<'ccx>> {
        // BTree map ensures that the output will be sorted by method name for more consistent codegen output.
        let mut method_infos = BTreeMap::<String, MethodInfo>::new();

        for method in methods {
            if let Some(info) = self.gen_method_info(id.into(), method) {
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
                            None => {
                                e.overloads.push(info.param_decls.clone());
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

    /// Generates C++ code for referencing a particular type with a given name.
    fn gen_ty_decl<'a, P: TyPosition>(&mut self, ty: &Type<P>, var_name: &'a str) -> NamedType<'a>
    where
        'ccx: 'a,
    {
        let var_name = self.formatter.cxx.fmt_param_name(var_name);
        let type_name = self.cpp.gen_type_name(ty);

        NamedType {
            var_name,
            type_name,
        }
    }

    fn gen_struct_name<P: TyPosition>(&mut self, st: &P::StructPath) -> Cow<'ccx, str> {
        let id = st.id();
        let type_name = self.formatter.cxx.fmt_type_name(id);

        let def = self.cpp.c.tcx.resolve_type(id);
        if def.attrs().disable {
            self.errors
                .push_error(format!("Found usage of disabled type {type_name}"))
        }

        self.cpp
            .impl_header
            .includes
            .insert(self.formatter.cxx.fmt_impl_header_path(id.into()));
        if let hir::MaybeOwn::Borrow(borrow) = st.owner() {
            let mutability = borrow.mutability;
            self.formatter
                .cxx
                .fmt_borrowed(&type_name, mutability)
                .into_owned()
                .into()
        } else {
            type_name
        }
    }

    /// Generates Python code for referencing a particular type.
    ///
    /// This function adds the necessary type imports to the decl and impl files.
    pub(super) fn gen_type_name<P: TyPosition>(&mut self, ty: &Type<P>) -> Cow<'ccx, str> {
        match *ty {
            Type::Primitive(prim) => self.formatter.cxx.fmt_primitive_as_c(prim),
            Type::Opaque(ref op) => {
                let op_id = op.tcx_id.into();
                let type_name = self.formatter.cxx.fmt_type_name(op_id);
                let def = self.cpp.c.tcx.resolve_type(op_id);

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

                self.cpp
                    .impl_header
                    .includes
                    .insert(self.formatter.cxx.fmt_impl_header_path(op_id.into()));
                ret
            }
            Type::Struct(ref st) => {
                let id = st.id();
                let type_name = self.formatter.cxx.fmt_type_name(id);
                let def = self.cpp.c.tcx.resolve_type(id);
                if def.attrs().disable {
                    self.errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }

                self.cpp
                    .impl_header
                    .includes
                    .insert(self.formatter.cxx.fmt_impl_header_path(id.into()));
                type_name
            }
            Type::Enum(ref e) => {
                let id = e.tcx_id.into();
                let type_name = self.formatter.cxx.fmt_type_name(id);
                let def = self.cpp.c.tcx.resolve_type(id);
                if def.attrs().disable {
                    self.errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }

                self.cpp
                    .impl_header
                    .includes
                    .insert(self.formatter.cxx.fmt_impl_header_path(id.into()));
                type_name
            }
            Type::Slice(hir::Slice::Str(_, encoding)) => {
                self.formatter.cxx.fmt_borrowed_str(encoding)
            }
            Type::Slice(hir::Slice::Primitive(b, p)) => {
                let ret = self.formatter.cxx.fmt_primitive_as_c(p);
                let ret = self.formatter.cxx.fmt_borrowed_slice(&ret, b.mutability());
                ret.into_owned().into()
            }
            Type::Slice(hir::Slice::Strs(encoding)) => format!(
                "diplomat::span<const {}>",
                self.formatter.cxx.fmt_borrowed_str(encoding)
            )
            .into(),
            Type::Slice(hir::Slice::Struct(b, ref st)) => {
                let st_name = self.gen_struct_name::<P>(st);
                let ret = self
                    .formatter
                    .cxx
                    .fmt_borrowed_slice(&st_name, b.mutability());
                ret.into_owned().into()
            }
            Type::DiplomatOption(ref inner) => {
                format!("std::optional<{}>", self.gen_type_name(inner)).into()
            }
            Type::Callback(..) => "".into(),
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    pub(super) fn gen_method_info(
        &mut self,
        id: SymbolId,
        method: &'tcx hir::Method,
    ) -> Option<MethodInfo<'ccx>> {
        if method.attrs.disable {
            return None;
        }
        let _guard = self.errors.set_context_method(method.name.as_str().into());
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
            && !matches!(id, hir::SymbolId::FunctionId(..))
        {
            def_qualifiers.extend(["static"]);
        }

        let param_decls = ParamInfo {
            params: method
                .params
                .iter()
                .map(|p| NamedType {
                    var_name: self.formatter.cxx.fmt_param_name(p.name.as_str()),
                    type_name: self.gen_type_name(&p.ty),
                })
                .collect(),
        };

        let mut visitor = method.borrowing_param_visitor(self.cpp.c.tcx, false);

        // Collect all the relevant borrowed params, with self in position 1 if present
        let mut param_borrows = Vec::new();

        let self_borrow = method
            .param_self
            .as_ref()
            .map(|s| visitor.visit_param(&s.ty.clone().into(), "self"));

        if let Some(b) = self_borrow.as_ref() {
            param_borrows.push(b.clone());
        };

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

        let mut lifetime_args = vec![];

        // Only returned values that are either created on return or return addresses previously unknown
        // to nanobind require additional annotation with keep_alive per: https://nanobind.readthedocs.io/en/latest/api_core.html#_CPPv4N8nanobind9rv_policyE
        // For any return of a reference to an existing object, nanobind is smart enough to locate its python wrapper object & correctly increment its refcount
        // No keep_alive for even borrowed string outputs, the type conversion always involves a copy
        if matches!(
            method.attrs.special_method,
            Some(hir::SpecialMethod::Iterator)
        ) || matches!(
            method.output.success_type(),
            hir::SuccessType::OutType(hir::OutType::Struct(..))
        ) || matches!(method.output.success_type(), hir::SuccessType::OutType(hir::OutType::Opaque(pth)) if pth.is_owned())
        {
            lifetime_args.extend(
                param_borrows
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
                    .collect::<Vec<_>>(),
            );
        }

        // Keep this just in case our method is returning some initially unknown value to an Opaque.
        // This won't make a difference for methods that return a reference to an already created value.
        if matches!(method.output.success_type(), hir::SuccessType::OutType(hir::Type::Opaque(path)) if !path.is_owned())
        {
            // For any type -> `Type<'a>`, as long as our self reference `&'a self` has the same lifetime (`'a`), we can assume that `&'a`` self has some kind of ownership of the returned type.
            //  Because `rv_policy` is only applied to unknown types (i.e., newly created references), then we can apply `reference_internal` to any of the cases above, without worrying about unnecessarily increasing the reference count for when we return `self` (since `self` is an already known type to Nanobind).
            let str = match self_borrow.as_ref() {
                // For non-borrowed &self however, we can just return a standard reference that has no attachment to &self:
                Some(hir::borrowing_param::ParamBorrowInfo::NotBorrowed) | None => {
                    "nb::rv_policy::reference"
                }
                _ => "nb::rv_policy::reference_internal",
            };
            lifetime_args.push(str.to_owned());
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
            overloads: vec![],
        })
    }
}
