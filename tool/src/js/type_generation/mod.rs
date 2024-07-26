use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};

use diplomat_core::hir::borrowing_param::{
    BorrowedLifetimeInfo, LifetimeEdge, LifetimeEdgeKind, ParamBorrowInfo, StructBorrowInfo,
};
use diplomat_core::hir::{
    self, EnumDef, LifetimeEnv, Method, OpaqueDef, SpecialMethod, SpecialMethodPresence, Type,
    TypeContext, TypeId,
};

use askama::{self, Template};

use super::formatter::JSFormatter;
use crate::ErrorStore;

mod converter;
use converter::StructBorrowContext;

pub(super) struct TyGenContext<'ctx, 'tcx> {
    pub tcx: &'tcx TypeContext,
    pub formatter: &'ctx JSFormatter<'tcx>,
    pub errors: &'ctx ErrorStore<'tcx, String>,
    pub typescript: bool,
    pub imports: BTreeSet<String>,
}

impl<'ctx, 'tcx> TyGenContext<'ctx, 'tcx> {
    pub(super) fn generate_base(&self, body: String) -> String {
        #[derive(Template)]
        #[template(path = "js/base.js.jinja", escape = "none")]
        struct BaseTemplate<'info> {
            body: String,
            typescript: bool,
            imports: &'info BTreeSet<String>,
        }
        BaseTemplate {
            body,
            typescript: self.typescript,
            imports: &self.imports,
        }
        .render()
        .unwrap()
    }

    /// Generate an enumerator's body for a file from the given definition. Called by [`JSGenerationContext::generate_file_from_type`]
    pub(super) fn gen_enum(
        &mut self,
        enum_def: &'tcx EnumDef,
        type_id: TypeId,
        type_name: &str,
    ) -> String {
        let mut methods = enum_def
            .methods
            .iter()
            .flat_map(|method| self.generate_method_body(type_id, method, self.typescript))
            .collect::<Vec<_>>();

        let special_method_body =
            self.generate_special_method_body(&enum_def.special_method_presence, self.typescript);
        if !special_method_body.is_empty() {
            methods.push(special_method_body);
        }

        #[derive(Template)]
        #[template(path = "js/enum.js.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            enum_def: &'a EnumDef,
            formatter: &'a JSFormatter<'a>,
            type_name: &'a str,
            typescript: bool,

            doc_str: String,

            methods: Vec<String>,
        }

        ImplTemplate {
            enum_def,
            formatter: self.formatter,
            type_name,
            typescript: self.typescript,
            doc_str: self.formatter.fmt_docs(&enum_def.docs),
            methods,
        }
        .render()
        .unwrap()
    }

    pub(super) fn gen_opaque(
        &mut self,
        opaque_def: &'tcx OpaqueDef,
        type_id: TypeId,
        type_name: &str,
    ) -> String {
        let mut methods = opaque_def
            .methods
            .iter()
            .flat_map(|method| self.generate_method_body(type_id, method, self.typescript))
            .collect::<Vec<_>>();

        let special_method_body =
            self.generate_special_method_body(&opaque_def.special_method_presence, self.typescript);
        if !special_method_body.is_empty() {
            methods.push(special_method_body);
        }

        let destructor = opaque_def.dtor_abi_name.as_str();

        #[derive(Template)]
        #[template(path = "js/opaque.js.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            type_name: &'a str,
            typescript: bool,

            lifetimes: &'a LifetimeEnv,
            methods: Vec<String>,
            destructor: &'a str,

            docs: String,
        }

        ImplTemplate {
            type_name,
            methods,
            destructor,
            typescript: self.typescript,
            docs: self.formatter.fmt_docs(&opaque_def.docs),
            lifetimes: &opaque_def.lifetimes,
        }
        .render()
        .unwrap()
    }

    pub(super) fn gen_struct<P: hir::TyPosition>(
        &mut self,
        struct_def: &'tcx hir::StructDef<P>,
        type_id: TypeId,
        is_out: bool,
        type_name: &str,
        mutable: bool,
    ) -> String {
        let (offsets, _) = crate::js::layout::struct_offsets_size_max_align(
            struct_def.fields.iter().map(|f| &f.ty),
            self.tcx,
        );

        let fields = struct_def.fields.iter().enumerate()
        .map(|field_enumerator| {
            let (i, field) = field_enumerator;

            let field_name = self.formatter.fmt_param_name(field.name.as_str());

            let js_type_name = self.gen_js_type_str(&field.ty);

            let c_to_js_deref = self.gen_c_to_js_deref_for_type(&field.ty, "ptr".into(), offsets[i]);

            let c_to_js = self.gen_c_to_js_for_type(
                &field.ty,
                format!("{field_name}Deref").into(), 
                &struct_def.lifetimes
            );

            let (js_to_c, maybe_post_cleanup_info, maybe_struct_borrow_info) = if let hir::Type::Slice(slice) = &field.ty {
                let slice_expr = self.gen_js_to_c_for_type(&field.ty, format!("this.#{}", field_name.clone()).into(), None);

                // We do not need to handle lifetime transitivity here: Methods already resolve
                // lifetime transitivity, and we have an HIR validity pass ensuring that struct lifetime bounds
                // are explicitly specified on methods.

                let post_cleanup_statement = if let Some(lt) = slice.lifetime() {
                    let hir::MaybeStatic::NonStatic(lt) = lt else {
                        panic!("'static not supported in JS2 backend");
                    };
                    format!(
                        "(appendArrayMap[{lt_name}AppendArray] || []).length > 0 ? () => {{ for (let lifetime of appendArrayMap[{lt_name}AppendArray]) {{ appendArrayMap[{lt_name}AppendArray].push({field_name}); }} {field_name}.garbageCollect(); }} : {field_name}.free",
                        lt_name = struct_def.lifetimes.fmt_lifetime(lt),
                    )
                } else {
                    // We take ownership
                    "".into()
                };

                (format!("{slice_expr}"), Some(post_cleanup_statement), None)
            } else {
                let borrow_info = if let hir::Type::Struct(path) = &field.ty {
                    StructBorrowInfo::compute_for_struct_field(struct_def, path, self.tcx).map(
                        |param_info| StructBorrowContext {
                            use_env: &struct_def.lifetimes,
                            param_info,
                            is_method: false
                        }
                    )
                } else {
                    None
                };

                (self.gen_js_to_c_for_type(&field.ty, format!("this.#{}", field_name.clone()).into(), borrow_info.as_ref()).into(), None, borrow_info.map(|s| s.param_info))
            };

            FieldInfo {
                field_name,
                field_type: &field.ty,
                js_type_name,
                c_to_js_deref,
                c_to_js,
                post_cleanup_statement: maybe_post_cleanup_info,
                js_to_c,
                maybe_struct_borrow_info
            }
        }).collect::<Vec<_>>();

        let mut methods = struct_def
            .methods
            .iter()
            .flat_map(|method| self.generate_method_body(type_id, method, self.typescript))
            .collect::<Vec<_>>();

        let special_method_body = self.generate_special_method_body(&struct_def.special_method_presence, self.typescript);
        if !special_method_body.is_empty() {
            methods.push(special_method_body);
        }

        #[derive(Template)]
        #[template(path = "js/struct.js.jinja", escape = "none")]
        struct ImplTemplate<'a, P: hir::TyPosition> {
            type_name: &'a str,
            mutable: bool,
            typescript: bool,

            has_default_constructor: bool,
            fields: Vec<FieldInfo<'a, P>>,
            methods: Vec<String>,
            docs: String,
            lifetimes: &'a LifetimeEnv,
        }

        ImplTemplate {
            type_name,
            mutable,
            typescript: self.typescript,

            has_default_constructor: is_out,
            fields,
            methods,
            docs: self.formatter.fmt_docs(&struct_def.docs),
            lifetimes: &struct_def.lifetimes,
        }
        .render()
        .unwrap()
    }

    /// Generate a string Javascript representation of a given method.
    ///
    /// Currently, this assumes that any method will be part of a class. That will probably be a parameter that's added, however.
    fn generate_method_body(
        &mut self,
        type_id: TypeId,
        method: &'tcx Method,
        typescript: bool,
    ) -> Option<String> {
        if method.attrs.disable {
            return None;
        }

        let mut visitor = method.borrowing_param_visitor(self.tcx);

        let _guard = self.errors.set_context_method(
            self.tcx.fmt_type_name_diagnostics(type_id),
            method.name.as_str().into(),
        );

        let abi_name = method.abi_name.as_str();

        let mut method_info = MethodInfo {
            abi_name,
            typescript,
            method: Some(method),
            needs_slice_cleanup: false,
            ..Default::default()
        };

        if let Some(param_self) = method.param_self.as_ref() {
            visitor.visit_param(&param_self.ty.clone().into(), "this");

            // We don't need to clean up structs for Rust because they're represented entirely in JS form.
            method_info
                .param_conversions
                .push(self.gen_js_to_c_self(&param_self.ty));

            if matches!(param_self.ty, hir::SelfType::Struct(..)) {
                method_info.needs_slice_cleanup = true;
            }
        }

        for param in method.params.iter() {
            let param_info = ParamInfo {
                name: self.formatter.fmt_param_name(param.name.as_str()),
                ty: self.gen_js_type_str(&param.ty),
            };

            let param_borrow_kind = visitor.visit_param(&param.ty, &param_info.name);

            // If we're a slice of strings or primitives. See [`hir::Types::Slice`].
            if let hir::Type::Slice(slice) = param.ty {
                let slice_expr =
                    self.gen_js_to_c_for_type(&param.ty, param_info.name.clone(), None);

                let is_borrowed = match param_borrow_kind {
                    ParamBorrowInfo::TemporarySlice => false,
                    ParamBorrowInfo::BorrowedSlice => true,
                    _ => unreachable!(
                        "Slices must produce slice ParamBorrowInfo, found {param_borrow_kind:?}"
                    ),
                };
                // We add the pointer and size for slices:
                method_info
                    .param_conversions
                    .push(format!("{}Slice.ptr", param_info.name).into());
                method_info
                    .param_conversions
                    .push(format!("{}Slice.size", param_info.name).into());

                // Then we make sure to handle clean-up for the slice:
                if is_borrowed {
                    // Is this function borrowing the slice?
                    // I.e., Do we need it alive for at least as long as this function call?
                    method_info
                        .cleanup_expressions
                        .push(format!("{}Slice.garbageCollect();", param_info.name).into());
                } else if slice.lifetime().is_some() {
                    // Is Rust NOT taking ownership?
                    // Then that means we can free this after the function is done.
                    method_info
                        .cleanup_expressions
                        .push(format!("{}Slice.free();", param_info.name).into());
                }

                method_info.slice_params.push(SliceParam {
                    name: param_info.name.clone(),
                    slice_expr,
                });
            } else {
                if let hir::Type::Struct(..) = param.ty {
                    method_info.needs_slice_cleanup = true;
                }

                let struct_borrow_info =
                    if let ParamBorrowInfo::Struct(param_info) = param_borrow_kind {
                        Some(converter::StructBorrowContext {
                            use_env: &method.lifetime_env,
                            param_info,
                            is_method: true,
                        })
                    } else {
                        None
                    };
                method_info
                    .param_conversions
                    .push(self.gen_js_to_c_for_type(
                        &param.ty,
                        param_info.name.clone(),
                        struct_borrow_info.as_ref(),
                    ));
            }

            method_info.parameters.push(param_info);
        }

        method_info.return_type = format!(": {}", self.gen_js_return_type_str(&method.output));

        method_info.return_expression =
            self.gen_c_to_js_for_return_type(&mut method_info, &method.lifetime_env);

        method_info.method_lifetimes_map = visitor.borrow_map();
        method_info.lifetimes = Some(&method.lifetime_env);

        method_info.method_decl = match &method.attrs.special_method {
            Some(SpecialMethod::Getter(name)) => {
                format!("get {}", self.formatter.fmt_method_field_name(name, method))
            }
            Some(SpecialMethod::Setter(name)) => {
                // Setters cannot have return type annotations
                method_info.return_type = Default::default();
                format!("set {}", self.formatter.fmt_method_field_name(name, method))
            }
            Some(SpecialMethod::Iterable) => "[Symbol.iterator]".to_string(),
            Some(SpecialMethod::Iterator) => "#iteratorNext".to_string(),

            _ if method.param_self.is_none() => {
                format!("static {}", self.formatter.fmt_method_name(method))
            }
            _ => self.formatter.fmt_method_name(method),
        };

        Some(method_info.render().unwrap())
    }

    /// If a special method exists inside a structure, opaque, or enum through [`SpecialMethodPresence`],
    /// We need to make sure Javascript can access it.
    ///
    /// This is mostly for iterators, using https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Iteration_protocols
    fn generate_special_method_body(
        &mut self,
        special_method_presence: &SpecialMethodPresence,
        typescript: bool,
    ) -> String {
        #[derive(Template)]
        #[template(path = "js/iterator.js.jinja", escape = "none")]
        struct SpecialMethodInfo<'a> {
            iterator: Option<Cow<'a, str>>,
            typescript: bool,
        }

        let mut iterator = None;

        if let Some(ref val) = special_method_presence.iterator {
            iterator = Some(self.gen_success_ty(val))
        }

        SpecialMethodInfo {
            iterator,
            typescript,
        }
        .render()
        .unwrap()
    }
}

#[derive(Default)]
struct ParamInfo<'a> {
    ty: Cow<'a, str>,
    name: Cow<'a, str>,
}

struct SliceParam<'a> {
    name: Cow<'a, str>,
    /// How to convert the JS type into a C slice.
    slice_expr: Cow<'a, str>,
}

#[derive(Default, Template)]
#[template(path = "js/method.js.jinja", escape = "none")]
struct MethodInfo<'info> {
    method: Option<&'info Method>,
    method_decl: String,
    /// Native C method name
    abi_name: &'info str,

    needs_slice_cleanup: bool,

    typescript: bool,

    parameters: Vec<ParamInfo<'info>>,
    slice_params: Vec<SliceParam<'info>>,
    param_conversions: Vec<Cow<'info, str>>,

    return_type: String,
    return_expression: Option<Cow<'info, str>>,

    method_lifetimes_map: BTreeMap<hir::Lifetime, BorrowedLifetimeInfo<'info>>,
    lifetimes: Option<&'info LifetimeEnv>,

    alloc_expressions: Vec<Cow<'info, str>>,
    cleanup_expressions: Vec<Cow<'info, str>>,
}

struct FieldInfo<'info, P: hir::TyPosition> {
    field_name: Cow<'info, str>,
    field_type: &'info Type<P>,
    js_type_name: Cow<'info, str>,
    c_to_js: Cow<'info, str>,
    c_to_js_deref: Cow<'info, str>,
    js_to_c: String,
    post_cleanup_statement: Option<String>,
    maybe_struct_borrow_info: Option<StructBorrowInfo<'info>>,
}

// Helpers used in templates (Askama has restrictions on Rust syntax)

/// Modified from dart backend
fn display_lifetime_edge<'a>(edge: &'a LifetimeEdge) -> Cow<'a, str> {
    let param_name = &edge.param_name;
    match edge.kind {
        // Opaque parameters are just retained as edges
        LifetimeEdgeKind::OpaqueParam => param_name.into(),
        // Slice parameters are constructed from diplomatRuntime.mjs:
        LifetimeEdgeKind::SliceParam => format!("{param_name}Slice").into(),
        // We extract the edge-relevant fields for a borrowed struct lifetime
        LifetimeEdgeKind::StructLifetime(def_env, def_lt) => format!(
            "...{param_name}._fieldsForLifetime{}",
            def_env.fmt_lifetime(def_lt).to_uppercase(),
        )
        .into(),
        _ => unreachable!("Unknown lifetime edge kind {:?}", edge.kind),
    }
}

fn iter_def_lifetimes_matching_use_lt<'a>(
    use_lt: &'a hir::Lifetime,
    info: &'a StructBorrowInfo,
) -> impl Iterator<Item = hir::Lifetime> + 'a {
    info.borrowed_struct_lifetime_map
        .iter()
        .filter(|(_def_lt, use_lts)| use_lts.contains(use_lt))
        .map(|(def_lt, _use_lts)| def_lt)
        .copied()
}

/// Iterate over fields, filtering by fields that actually use lifetimes from `lifetimes`
fn iter_fields_with_lifetimes_from_set<'a, P: hir::TyPosition>(
    fields: &'a [FieldInfo<'a, P>],
    lifetime: &'a hir::Lifetime,
) -> impl Iterator<Item = &'a FieldInfo<'a, P>> + 'a {
    /// Does `ty` use any lifetime from `lifetimes`?
    fn does_type_use_lifetime_from_set<P: hir::TyPosition>(
        ty: &Type<P>,
        lifetime: &hir::Lifetime,
    ) -> bool {
        ty.lifetimes().any(|lt| {
            let hir::MaybeStatic::NonStatic(lt) = lt else {
                panic!("'static not supported in JS2 backend");
            };
            lt == *lifetime
        })
    }

    fields
        .iter()
        .filter(move |f| does_type_use_lifetime_from_set(f.field_type, lifetime))
}
