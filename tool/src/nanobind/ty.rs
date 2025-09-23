use super::root_module::RootModule;
use super::PyFormatter;
use crate::nanobind::func::{FuncGenContext, MethodInfo};
use crate::{cpp::ItemGenContext as CppItemGenContext, hir, ErrorStore};
use askama::Template;
use diplomat_core::hir::{OpaqueOwner, StructPathLike, SymbolId, TyPosition, Type, TypeId};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

/// A type name with a corresponding variable name, such as a struct field or a function parameter.
#[derive(Clone)]
pub(super) struct NamedType<'a> {
    pub(super) var_name: Cow<'a, str>,
    pub(super) type_name: Cow<'a, str>,
}

/// Context for generating a particular type's impl
/// 'tcx refers to the lifetime of the typecontext
/// 'cx refers to the lifetime of the context itself
pub(super) struct TyGenContext<'cx, 'tcx> {
    pub formatter: &'cx PyFormatter<'tcx>,
    pub errors: &'cx ErrorStore<'tcx, String>,
    pub cpp: CppItemGenContext<'cx, 'tcx, 'cx>,
    pub root_module: &'cx mut RootModule<'tcx>,
    pub submodules: &'cx mut BTreeMap<Cow<'tcx, str>, BTreeSet<Cow<'tcx, str>>>,
    /// Are we currently generating struct fields?
    pub generating_struct_fields: bool,
}

impl<'ccx, 'tcx: 'ccx> TyGenContext<'ccx, 'tcx> {
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

        #[derive(Template)]
        #[template(path = "nanobind/enum_impl.cpp.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            type_name: &'a str,
            values: Vec<Cow<'a, str>>,
            type_name_unnamespaced: &'a str,
        }

        ImplTemplate {
            type_name: &type_name,
            values,
            type_name_unnamespaced: &type_name_unnamespaced,
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
            self.formatter.fmt_binding_fn(id, true),
            self.formatter.fmt_binding_fn(id, false),
        );
    }

    pub fn gen_binding_fn(
        root_module: &mut RootModule,
        namespaces: impl Iterator<Item = &'tcx str>,
        binding_fn_name: String,
        binding_fn_name_unnamespaced: String,
    ) {
        let vec = namespaces.collect::<Vec<_>>();
        root_module
            .fwd_decls
            .entry(vec.join("::"))
            .or_default()
            .push(format!("void {binding_fn_name_unnamespaced}(nb::module_);"));

        let module_namespaces = [root_module.module_name.to_string()]
            .into_iter()
            .chain(vec.iter().map(|s| s.to_string()))
            .collect();

        let entry = root_module.module_fns.entry(module_namespaces).or_default();

        entry.push(binding_fn_name);
    }

    pub fn gen_opaque_def<W: std::fmt::Write + ?Sized>(
        &mut self,
        ty: &'tcx hir::OpaqueDef,
        id: TypeId,
        out: &mut W,
    ) {
        let type_name = self.formatter.cxx.fmt_type_name(id);
        let type_name_unnamespaced = self.formatter.cxx.fmt_type_name_unnamespaced(id);

        let methods = self.gen_all_method_infos(id, ty.methods.iter());

        #[derive(Template)]
        #[template(path = "nanobind/opaque_impl.cpp.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            type_name: &'a str,
            methods: &'a [MethodInfo<'a>],
            type_name_unnamespaced: &'a str,
        }

        ImplTemplate {
            type_name: &type_name,
            methods: methods.as_slice(),
            type_name_unnamespaced: &type_name_unnamespaced,
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
        #[derive(Template)]
        #[template(path = "nanobind/struct_impl.cpp.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            type_name: &'a str,
            fields: &'a [NamedType<'a>],
            methods: &'a [MethodInfo<'a>],
            type_name_unnamespaced: &'a str,
            has_constructor: bool,
            is_sliceable: bool,
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
            if let Some(info) = FuncGenContext::gen_method_info(id.into(), method, self) {
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
}
