use std::{borrow::Cow, collections::BTreeSet};

use askama::Template;
use diplomat_core::hir::{self, FunctionId, SymbolId};

use crate::nanobind::{gen::NamedType, ItemGenContext, RootModule};

use diplomat_core::hir::Type;

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

/// Helper for rendering function block information.
/// Used either for creating blocks of functions that belong to structs, or for free functions that belong to no structs.
pub(super) struct FuncGenContext {
    pub(super) namespace: Option<String>,
    namespaces: Vec<String>,
    pub(super) functions: Vec<String>,
    pub(super) includes: BTreeSet<String>,
}

impl<'tcx> FuncGenContext {
    pub(super) fn new(namespace: Option<String>, namespaces: Vec<String>) -> Self {
        Self {
            namespace,
            namespaces,
            functions: Vec::new(),
            includes: BTreeSet::new(),
        }
    }

    pub fn generate_function<'b>(
        &mut self,
        id: FunctionId,
        func: &'tcx hir::Method,
        context: &mut ItemGenContext<'b, 'tcx>,
    ) {
        context.gen_modules(id.into(), None);
        let info = Self::gen_method_info(id.into(), func, context);

        #[derive(Template)]
        #[template(path = "nanobind/function_impl.cpp.jinja", escape = "none")]
        struct FunctionDef<'a> {
            m: MethodInfo<'a>,
        }

        self.includes
            .insert(context.formatter.cxx.fmt_impl_header_path(id.into()));

        if let Some(m) = info {
            let def = FunctionDef { m };
            self.functions.push(def.to_string());
            //self.includes.append(context.includes);
        }
    }

    pub fn render(&mut self, root_module: &mut RootModule) -> Result<String, askama::Error> {
        #[derive(Template)]
        #[template(path = "nanobind/binding.cpp.jinja", escape = "none")]
        struct Binding {
            includes: BTreeSet<String>,
            namespace: String,
            unqualified_type: String,
            body: String,
            binding_prefix: String,
        }

        let no_add_binding_fn_name_unnamespaced = if self.namespace.is_some() {
            format!("{}_func", self.namespaces.join("_"))
        } else {
            "diplomat_func".into()
        };

        let binding_fn_name_unnamespaced =
            format!("add_{no_add_binding_fn_name_unnamespaced}_binding");

        let binding_fn_name = if let Some(ns) = &self.namespace {
            format!("{ns}::{binding_fn_name_unnamespaced}")
        } else {
            binding_fn_name_unnamespaced.clone()
        };

        ItemGenContext::gen_binding_fn(
            root_module,
            self.namespaces.iter().map(|s| s.as_str()),
            binding_fn_name,
            binding_fn_name_unnamespaced,
        );
        let b = Binding {
            includes: self.includes.clone(),
            namespace: self.namespace.clone().unwrap_or_default(),
            unqualified_type: no_add_binding_fn_name_unnamespaced,
            body: format!("mod\n{};", self.functions.join("\n")),
            binding_prefix: String::new(),
        };
        b.render()
    }

    pub(super) fn gen_method_info<'a, 'b>(
        id: SymbolId,
        method: &'a hir::Method,
        context: &mut ItemGenContext<'b, 'a>,
    ) -> Option<MethodInfo<'b>> {
        if method.attrs.disable {
            return None;
        }
        let _guard = context.errors.set_context_method(
            context.cpp.c.tcx.fmt_symbol_name_diagnostics(id),
            method.name.as_str().into(),
        );
        let cpp_method_name = context.formatter.cxx.fmt_method_name(method);
        let method_name = context.formatter.fmt_method_name(method);
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
                    var_name: context.formatter.cxx.fmt_param_name(p.name.as_str()),
                    type_name: context.gen_type_name(&p.ty),
                })
                .collect(),
        };

        let mut visitor = method.borrowing_param_visitor(context.cpp.c.tcx, false);

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
