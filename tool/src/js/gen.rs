//! Built around the [`TyGenContext`] type. We use this for creating `.mjs` and `.d.ts` files from given [`hir::TypeDef`]s.
//! See [`converter`] for more conversion specific functions.

use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

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

use super::converter::{ForcePaddingStatus, JsToCConversionContext, StructBorrowContext};

/// Represents context for generating a Javascript class.
///
/// Given an enum, opaque, struct, etc. (anything from [`hir::TypeDef`] that JS supports), this handles creation of the associated `.mjs`` files.
pub(super) struct TyGenContext<'ctx, 'tcx> {
    pub tcx: &'tcx TypeContext,
    pub type_name: Cow<'tcx, str>,
    pub formatter: &'ctx JSFormatter<'tcx>,
    pub errors: &'ctx ErrorStore<'tcx, String>,
    /// Imports, stored as a type name. Imports are fully resolved in [`TyGenContext::generate_base`], with a call to [`JSFormatter::fmt_import_statement`].
    pub imports: RefCell<BTreeSet<String>>,
}

impl<'ctx, 'tcx> TyGenContext<'ctx, 'tcx> {
    /// Generates the code at the top of every `.d.ts` and `.mjs` file.
    ///
    /// This could easily be an [inherited template](https://djc.github.io/askama/template_syntax.html#template-inheritance), if you want to be a little more strict about how templates are used.
    pub(super) fn generate_base(&self, typescript: bool, body: String) -> String {
        #[derive(Template)]
        #[template(path = "js/base.js.jinja", escape = "none")]
        struct BaseTemplate {
            body: String,
            typescript: bool,
            imports: Vec<String>,
        }

        let mut new_imports = Vec::new();
        for import in self.imports.borrow().iter() {
            new_imports.push(
                self.formatter
                    .fmt_import_statement(import, typescript, "./".into()),
            );
        }

        BaseTemplate {
            body,
            typescript,
            imports: new_imports,
        }
        .render()
        .unwrap()
    }

    /// A wrapper for `borrow_mut`ably inserting new imports.
    ///
    /// I do this to avoid borrow checking madness.
    pub(super) fn add_import(&self, import_str: String) {
        self.imports.borrow_mut().insert(import_str);
    }

    /// Exists for the same reason as [`Self::add_import`].
    ///
    /// Right now, only used for removing any self imports.
    pub(super) fn remove_import(&self, import_str: String) {
        self.imports.borrow_mut().remove(&import_str);
    }

    /// Generate an enumerator type's body for a file from the given definition.
    pub(super) fn gen_enum(
        &self,
        typescript: bool,

        enum_def: &'tcx EnumDef,
        methods: &MethodsInfo,
    ) -> String {
        let is_contiguous = enum_def
            .variants
            .iter()
            .enumerate()
            .all(|(i, v)| i as isize == v.discriminant);

        #[derive(Template)]
        #[template(path = "js/enum.js.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            enum_def: &'a EnumDef,
            formatter: &'a JSFormatter<'a>,
            type_name: &'a str,
            typescript: bool,
            is_contiguous: bool,

            doc_str: String,

            methods: &'a MethodsInfo<'a>,
        }

        ImplTemplate {
            enum_def,
            formatter: self.formatter,
            type_name: &self.type_name,
            typescript,

            doc_str: self.formatter.fmt_docs(&enum_def.docs),
            is_contiguous,

            methods,
        }
        .render()
        .unwrap()
    }

    /// Generate an opaque type's body for a file from the given definition.
    pub(super) fn gen_opaque(
        &self,
        typescript: bool,

        opaque_def: &'tcx OpaqueDef,
        methods: &MethodsInfo,
    ) -> String {
        let destructor = opaque_def.dtor_abi_name.as_str();

        #[derive(Template)]
        #[template(path = "js/opaque.js.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            type_name: &'a str,
            typescript: bool,

            lifetimes: &'a LifetimeEnv,
            destructor: &'a str,

            docs: String,

            methods: &'a MethodsInfo<'a>,
        }

        ImplTemplate {
            type_name: &self.type_name,
            typescript,

            lifetimes: &opaque_def.lifetimes,
            destructor,

            docs: self.formatter.fmt_docs(&opaque_def.docs),

            methods,
        }
        .render()
        .unwrap()
    }

    /// Generate a list of [`FieldInfo`] to be used in [`Self::gen_struct`].
    ///
    /// Also returns a boolean of whether the forcePadding argument is needed.
    ///
    /// We separate this step out for two reasons:
    ///
    /// 1. It allows re-use between `.d.ts` and `.mjs` files.
    /// 2. Clarity.
    pub(super) fn generate_fields<P: hir::TyPosition>(
        &self,
        struct_def: &'tcx hir::StructDef<P>,
    ) -> (Vec<FieldInfo<P>>, bool) {
        let struct_field_info =
            crate::js::layout::struct_field_info(struct_def.fields.iter().map(|f| &f.ty), self.tcx);
        let mut needs_force_padding = false;

        let fields = struct_def.fields.iter().enumerate()
        .map(|(i, field)| {
            let field_name = self.formatter.fmt_param_name(field.name.as_str());

            let js_type_name = self.gen_js_type_str(&field.ty);

            let is_option = matches!(&field.ty, hir::Type::DiplomatOption(..));

            let c_to_js_deref = self.gen_c_to_js_deref_for_type(&field.ty, "ptr".into(), struct_field_info.fields[i].offset);

            let c_to_js = self.gen_c_to_js_for_type(
                &field.ty,
                format!("{field_name}Deref").into(), 
                &struct_def.lifetimes
            );

            let alloc = if let &hir::Type::Slice(slice) = &field.ty {
                if let Some(lt) = slice.lifetime() {
                    let hir::MaybeStatic::NonStatic(lt) = lt else {
                        panic!("'static not supported in JS backend");
                    };
                    let lt_name = struct_def.lifetimes.fmt_lifetime(lt);
                    Some(
                        format!("diplomatRuntime.CleanupArena.maybeCreateWith(functionCleanupArena, ...appendArrayMap['{lt_name}AppendArray'])")
                    )
                } else {
                    None
                }
            } else if let &hir::Type::Struct(..) = &field.ty {
                Some("functionCleanupArena".into())
            } else {
                // We take ownership
                None
            };

            let maybe_struct_borrow_info = if let hir::Type::Struct(ref path) = field.ty.unwrap_option() {
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

            let padding = struct_field_info.fields[i].padding_count;

            // See docs/wasm_abi_quirks.md for when "padded direct" parameter passing kicks in
            let force_padding = match (struct_field_info.fields[i].scalar_count, struct_field_info.scalar_count) {
                // There's no padding needed
                (0 | 1, _) => ForcePaddingStatus::NoForce,
                // Non-structs don't care
                // This includes slices, which *are* aggregates but have no padding.
                _ if !matches!(&field.ty, &hir::Type::Struct(_)) => ForcePaddingStatus::NoForce,
                // 2-field struct contained in 2-field struct, caller decides
                (2, 2) => {
                    needs_force_padding = true;
                    ForcePaddingStatus::PassThrough
                }
                // Outer struct has > 3 fields, always pad
                (2, 3..) => ForcePaddingStatus::Force,
                // Larger fields will always have padding anyway
                _ => ForcePaddingStatus::NoForce

            };

            let maybe_padding_after = if padding > 0 {
                let padding_size_str = match struct_field_info.fields[i].padding_field_width {
                    1 => "i8",
                    2 => "i16",
                    4 => "i32",
                    8 => "i64",
                    other => unreachable!("Found unknown padding size {other}")
                };

                if struct_field_info.scalar_count == 2 {
                    // For structs with 2 scalar fields, we pass down whether or not padding is needed from the caller. See docs/wasm_abi_quirks.md
                    needs_force_padding = true;
                    format!(", ...diplomatRuntime.maybePaddingFields(forcePadding, {padding} /* x {padding_size_str} */)")
                } else {
                    let mut out = format!(", /* [{padding} x {padding_size_str}] padding */ ");
                    for i in 0..padding {
                        if i < padding - 1 {
                            write!(out, "0, ").unwrap();
                        } else {
                            write!(out, "0 /* end padding */").unwrap();
                        }
                    }

                    out
                }

            } else {
                "".into()
            };
            let js_name = format!("this.#{}", field_name);
            let js_to_c = self.gen_js_to_c_for_type(&field.ty, js_name.clone().into(), maybe_struct_borrow_info.as_ref(), alloc.as_deref(), JsToCConversionContext::List(force_padding));
            let js_to_c = format!("{js_to_c}{maybe_padding_after}");
            let js_to_c_write = self.gen_js_to_c_for_type(&field.ty, js_name.into(), maybe_struct_borrow_info.as_ref(), alloc.as_deref(), JsToCConversionContext::WriteToBuffer("offset", struct_field_info.fields[i].offset)).into();

            FieldInfo {
                field_name,
                field_type: &field.ty,
                js_type_name,
                c_to_js_deref,
                c_to_js,
                js_to_c,
                js_to_c_write,
                maybe_struct_borrow_info: maybe_struct_borrow_info.map(|i| i.param_info),
                is_optional: is_option
            }
        }).collect::<Vec<_>>();

        (fields, needs_force_padding)
    }

    /// Generate a struct type's body for a file from the given definition.
    ///
    /// Used for both [`hir::TypeDef::Struct`] and [`hir::TypeDef::OutStruct`], which is why `is_out` exists.
    pub(super) fn gen_struct<P: hir::TyPosition>(
        &self,
        typescript: bool,

        struct_def: &'tcx hir::StructDef<P>,
        fields: &Vec<FieldInfo<P>>,
        methods: &MethodsInfo,

        is_out: bool,
        needs_force_padding: bool,
    ) -> String {
        #[derive(Template)]
        #[template(path = "js/struct.js.jinja", escape = "none")]
        struct ImplTemplate<'a, P: hir::TyPosition> {
            type_name: &'a str,

            typescript: bool,
            mutable: bool,
            is_out: bool,
            needs_force_padding: bool,

            lifetimes: &'a LifetimeEnv,
            fields: &'a Vec<FieldInfo<'a, P>>,
            methods: &'a MethodsInfo<'a>,

            docs: String,
        }

        ImplTemplate {
            type_name: &self.type_name,

            typescript,
            is_out,
            mutable: !is_out,
            needs_force_padding,

            lifetimes: &struct_def.lifetimes,
            fields,
            methods,

            docs: self.formatter.fmt_docs(&struct_def.docs),
        }
        .render()
        .unwrap()
    }

    /// Generate required method info for all other [`TyGenContext::generate_*`] calls.
    ///
    /// For re-usability between `.d.ts` and `.mjs` files.
    pub(super) fn generate_method(
        &self,
        type_id: TypeId,
        method: &'tcx Method,
    ) -> Option<MethodInfo> {
        if method.attrs.disable {
            return None;
        }

        let mut visitor = method.borrowing_param_visitor(self.tcx);

        let _guard = self.errors.set_context_method(
            self.tcx.fmt_type_name_diagnostics(type_id),
            method.name.as_str().into(),
        );

        let abi_name = String::from(method.abi_name.as_str());

        let mut method_info = MethodInfo {
            abi_name,
            method_output_is_ffi_unit: method.output.is_ffi_unit(),
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

            // If we're a slice of strings or primitives. See [`hir::Type::Slice`].
            if let hir::Type::Slice(..) = param.ty {
                let slice_expr = self.gen_js_to_c_for_type(&param.ty, param_info.name.clone(), None, Some(
                        match param_borrow_kind {
                            // Is Rust NOT taking ownership?
                            // Then that means we can free this after the function is done.
                            ParamBorrowInfo::TemporarySlice => {
                                method_info.needs_slice_cleanup = true;
                                "functionCleanupArena"
                            },

                            // Is this function borrowing the slice?
                            // I.e., Do we need it alive for at least as long as this function call?
                            ParamBorrowInfo::BorrowedSlice => {
                                method_info.needs_slice_collection = true;
                                "functionGarbageCollectorGrip"
                            },
                            _ => unreachable!(
                                "Slices must produce slice ParamBorrowInfo, found {param_borrow_kind:?}"
                            ),
                        }
                    ),
                    // We're specifically doing slice preallocation here
                    JsToCConversionContext::SlicePrealloc
                    );
                // We add the pointer and size for slices:
                method_info
                    .param_conversions
                    .push(format!("...{}Slice.splat()", param_info.name).into());

                method_info.slice_params.push(SliceParam {
                    name: param_info.name.clone(),
                    slice_expr: slice_expr.to_string(),
                });
            } else {
                let alloc = if let hir::Type::Struct(..) = param.ty.unwrap_option() {
                    method_info.needs_slice_cleanup = true;
                    Some("functionCleanupArena")
                } else {
                    None
                };

                let struct_borrow_info =
                    if let ParamBorrowInfo::Struct(param_info) = param_borrow_kind {
                        Some(super::converter::StructBorrowContext {
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
                        alloc,
                        // Arguments need a list, and never force padding
                        JsToCConversionContext::List(ForcePaddingStatus::NoForce),
                    ));
            }

            method_info.parameters.push(param_info);
        }

        method_info.return_type = format!(": {}", self.gen_js_return_type_str(&method.output));

        method_info.return_expression = self.gen_c_to_js_for_return_type(&mut method_info, method);

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

        Some(method_info)
    }

    /// If a special method exists inside a structure, opaque, or enum through [`SpecialMethodPresence`],
    /// We need to make sure Javascript can access it.
    ///
    /// This is mostly for iterators, using https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Iteration_protocols
    pub(super) fn generate_special_method(
        &self,
        special_method_presence: &SpecialMethodPresence,
    ) -> SpecialMethodInfo {
        let mut iterator = None;

        if let Some(ref val) = special_method_presence.iterator {
            iterator = Some(self.gen_success_ty(val))
        }

        SpecialMethodInfo {
            iterator,
            typescript: false,
        }
    }
}

/// Represents a parameter of a method. Used as part of [`MethodInfo`], exclusively in the method definition.
#[derive(Default)]
pub(super) struct ParamInfo<'a> {
    ty: Cow<'a, str>,
    name: Cow<'a, str>,
}

/// Represents a slice parameter of a method. Used as part of [`MethodInfo`].
///
/// Any slice is stored as both a [`ParamInfo`], and [`SliceParam`].
///
/// [`ParamInfo`] represents the conversion of the slice into C-friendly terms. This just represents an extra stage for Diplomat to convert whatever slice type we're given into a type that returns a `.ptr` and `.size` field.
///
/// See `DiplomatBuf` in `runtime.mjs` for more.
pub(super) struct SliceParam<'a> {
    name: Cow<'a, str>,
    /// How to convert the JS type into a C slice.
    slice_expr: String,
}

/// Represents a Rust method that we invoke inside of WebAssembly with JS.
///
/// Has an attached template to convert it into Javascript.
#[derive(Default, Template)]
#[template(path = "js/method.js.jinja", escape = "none")]
pub(super) struct MethodInfo<'info> {
    /// Do we return the `()` type?
    pub method_output_is_ffi_unit: bool,
    /// The declaration signature. Something like `static functionName() { /* ... */ }` versus `functionName() { /* ... */ }`
    pub method_decl: String,

    /// Native C method name
    pub abi_name: String,

    /// If we need to create a `CleanupArena` (see `runtime.mjs`) to free any [`SliceParam`]s that are present.
    pub needs_slice_cleanup: bool,
    /// For calling .releaseToGarbageCollector on slices.
    pub needs_slice_collection: bool,

    pub typescript: bool,

    /// Represents all the parameters in the method definition (mostly for `.d.ts` generation, showing names and types).
    pub parameters: Vec<ParamInfo<'info>>,
    /// See [`SliceParam`] for info on how this array is used.
    pub slice_params: Vec<SliceParam<'info>>,
    /// Represents the Javascript needed to take the parameters from the method definition into C-friendly terms. See [`TyGenContext::gen_js_to_c_for_type`] for more.
    pub param_conversions: Vec<Cow<'info, str>>,

    /// The return type, for `.d.ts` files.
    pub return_type: String,
    /// The JS expression used when this method returns.
    pub return_expression: Option<Cow<'info, str>>,

    /// Used for generating edge information when constructing items like Slices, Structs, and Opaque types. See [hir::methods::borrowing_param::BorrowedLifetimeInfo] for more.
    pub method_lifetimes_map: BTreeMap<hir::Lifetime, BorrowedLifetimeInfo<'info>>,
    /// We use this to access individual [`hir::Lifetimes`], which we then use to access the [`MethodInfo::method_lifetimes_map`].
    pub lifetimes: Option<&'info LifetimeEnv>,

    /// Anything we need to allocate for [`MethodInfo::param_conversions`]
    pub alloc_expressions: Vec<Cow<'info, str>>,
    /// Anything from [`MethodInfo::alloc_expressions`] we need to clean up afterwards.
    pub cleanup_expressions: Vec<Cow<'info, str>>,
}

/// See [`TyGenContext::generate_special_method`].
#[derive(Template)]
#[template(path = "js/iterator.js.jinja", escape = "none")]
pub(super) struct SpecialMethodInfo<'a> {
    iterator: Option<Cow<'a, str>>,
    pub typescript: bool,
}

/// An amalgamation of both [`SpecialMethodInfo`] and [`MethodInfo`], since these two always get passed together in methods.
pub(super) struct MethodsInfo<'a> {
    pub methods: Vec<MethodInfo<'a>>,
    pub special_methods: SpecialMethodInfo<'a>,
}

/// Represents a re-usable set of information for any [`hir::TypeDef::Struct`]s.
#[derive(Clone)]
pub(super) struct FieldInfo<'info, P: hir::TyPosition> {
    field_name: Cow<'info, str>,
    field_type: &'info Type<P>,
    /// Representation of the type in `.d.ts` terms.
    js_type_name: Cow<'info, str>,
    c_to_js: Cow<'info, str>,
    /// Because all structs are created in WebAssembly as pointers, we need to be able to de-reference those pointers. This is an expression for taking a given pointer and returning JS.
    c_to_js_deref: Cow<'info, str>,
    js_to_c: String,
    /// A version of js_to_c that writes the struct field to an arraybuffer `arrayBuffer` at offset `offset`
    js_to_c_write: String,
    /// Used in `get _fieldsForLifetime...` fields, which themselves are used in [`display_lifetime_edge`].
    maybe_struct_borrow_info: Option<StructBorrowInfo<'info>>,

    /// Used in the constructor() function to determine whether or not this field is required for construction.
    is_optional: bool,
}

// Helpers used in templates (Askama has restrictions on Rust syntax)

/// Used in `method.js.jinja`. Used to create JS friendly interpretations of lifetime edges, to be passed into newly created JS structures (see [`JSFormatter::fmt_lifetime_edge_array`] and see [`TyGenContext::gen_c_to_js_for_type`] for more.)
///
/// Modified from dart backend.
fn display_lifetime_edge<'a>(edge: &'a LifetimeEdge) -> Cow<'a, str> {
    let param_name = &edge.param_name;
    match edge.kind {
        // Opaque parameters are just retained as edges
        LifetimeEdgeKind::OpaqueParam => param_name.into(),
        // Slice parameters are constructed from diplomatRuntime.mjs:
        LifetimeEdgeKind::SliceParam => format!("{param_name}Slice").into(),
        // We extract the edge-relevant fields for a borrowed struct lifetime
        LifetimeEdgeKind::StructLifetime(def_env, def_lt, is_option) => {
            let lt = def_env.fmt_lifetime(def_lt).to_uppercase();
            if is_option {
                format!("...({param_name}?._fieldsForLifetime{lt} || [])").into()
            } else {
                format!("...{param_name}._fieldsForLifetime{lt}").into()
            }
        }

        _ => unreachable!("Unknown lifetime edge kind {:?}", edge.kind),
    }
}

/// Helper function, since Askama can't use iterators quite like this.
///
/// Simple way to check if a lifetime is present within our map.
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
                panic!("'static not supported in JS backend");
            };
            lt == *lifetime
        })
    }

    fields
        .iter()
        .filter(move |f| does_type_use_lifetime_from_set(f.field_type, lifetime))
}
