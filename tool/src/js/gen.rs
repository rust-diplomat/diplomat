//! Built around the [`ItemGenContext`] type. We use this for creating `.mjs` and `.d.ts` files from given [`hir::TypeDef`]s.
//! See [`converter`] for more conversion specific functions.

use std::alloc::Layout;
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

use diplomat_core::hir::borrowing_param::{
    BorrowedLifetimeInfo, LifetimeEdge, LifetimeEdgeKind, ParamBorrowInfo, StructBorrowInfo,
};
use diplomat_core::hir::{
    self, EnumDef, LifetimeEnv, Method, OpaqueDef, SpecialMethod, SpecialMethodPresence,
    StructPathLike, Type, TypeContext, TypeId,
};

use askama::{self, Template};

use super::formatter::JSFormatter;
use super::JsConfig;
use crate::filters;
use crate::ErrorStore;

use super::converter::{JsToCConversionContext, StructBorrowContext};

/// Represents list of imports that our Type is going to use.
/// Resolved in [`ItemGenContext::generate_base`]
pub(super) struct Imports<'tcx> {
    pub js: BTreeSet<ImportInfo<'tcx>>,
    pub ts: BTreeSet<ImportInfo<'tcx>>,
}

/// Represents context for generating a Javascript class.
///
/// Given an enum, opaque, struct, etc. (anything from [`hir::TypeDef`] that JS supports), this handles creation of the associated `.mjs`` files.
pub(super) struct ItemGenContext<'ctx, 'tcx> {
    pub tcx: &'tcx TypeContext,
    pub type_name: Cow<'tcx, str>,
    pub formatter: &'ctx JSFormatter<'tcx>,
    pub errors: &'ctx ErrorStore<'tcx, String>,
    /// Imports, stored as a type name. Imports are fully resolved in [`ItemGenContext::generate_base`], with a call to [`JSFormatter::fmt_import_statement`].
    pub imports: RefCell<Imports<'tcx>>,
    #[allow(dead_code)]
    pub config: JsConfig,
}

impl<'tcx> ItemGenContext<'_, 'tcx> {
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

        let i = self.imports.borrow();

        let mut new_imports = Vec::new();
        let imports = if typescript { i.ts.iter() } else { i.js.iter() };

        for import in imports {
            new_imports.push(self.formatter.fmt_import_statement(
                &import.import_type,
                typescript,
                "./".into(),
                &import.import_file,
            ));
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
    pub(super) fn add_import(
        &self,
        import_str: Cow<'tcx, str>,
        import_file: Option<Cow<'tcx, str>>,
        usage: ImportUsage,
    ) {
        let inf = ImportInfo {
            import_type: import_str.clone(),
            import_file: import_file.unwrap_or(
                self.formatter
                    .fmt_file_name_extensionless(&import_str)
                    .into(),
            ),
        };
        if usage == ImportUsage::Module || usage == ImportUsage::Both {
            self.imports.borrow_mut().js.insert(inf.clone());
        }
        if usage == ImportUsage::Typescript || usage == ImportUsage::Both {
            self.imports.borrow_mut().ts.insert(inf);
        }
    }

    /// Exists for the same reason as [`Self::add_import`].
    ///
    /// Right now, only used for removing any self imports.
    pub(super) fn remove_import(
        &self,
        import_str: Cow<'tcx, str>,
        import_file: Option<Cow<'tcx, str>>,
        usage: ImportUsage,
    ) {
        let inf = ImportInfo {
            import_type: import_str,
            import_file: import_file.unwrap_or_default(),
        };

        if usage == ImportUsage::Module || usage == ImportUsage::Both {
            self.imports.borrow_mut().js.remove(&inf);
        }

        if usage == ImportUsage::Typescript || usage == ImportUsage::Both {
            self.imports.borrow_mut().ts.remove(&inf);
        }
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

            /// Used by `js_class.js.jinja`. If a constructor isn't overridden by #[diplomat::attr(auto, constructor)], this is the logic that `js_class.js.jinja` will use to determine whether or not to generate constructor code.
            show_default_ctor: bool,
        }

        ImplTemplate {
            enum_def,
            formatter: self.formatter,
            type_name: &self.type_name,
            typescript,

            doc_str: self.formatter.fmt_docs(&enum_def.docs, &enum_def.attrs),
            is_contiguous,

            methods,

            show_default_ctor: true,
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

            doc_str: String,

            methods: &'a MethodsInfo<'a>,

            /// Used by `js_class.js.jinja`. If a constructor isn't overridden by #[diplomat::attr(auto, constructor)], this is the logic that `js_class.js.jinja` will use to determine whether or not to generate constructor code.
            /// Useful for hiding opaque constructors in typescript headers, for instance.
            show_default_ctor: bool,
        }

        ImplTemplate {
            type_name: &self.type_name,
            typescript,

            lifetimes: &opaque_def.lifetimes,
            destructor,

            doc_str: self.formatter.fmt_docs(&opaque_def.docs, &opaque_def.attrs),

            methods,

            show_default_ctor: !typescript,
        }
        .render()
        .unwrap()
    }

    /// Generate a list of [`FieldInfo`] to be used in [`Self::gen_struct`].
    ///
    /// We separate this step out for two reasons:
    ///
    /// 1. It allows re-use between `.d.ts` and `.mjs` files.
    /// 2. Clarity.
    pub(super) fn generate_fields<P: hir::TyPosition>(
        &self,
        struct_def: &'tcx hir::StructDef<P>,
    ) -> (Vec<FieldInfo<'_, P>>, Layout) {
        let struct_field_info =
            crate::js::layout::struct_field_info(struct_def.fields.iter().map(|f| &f.ty), self.tcx);

        let fields = struct_def.fields.iter().enumerate()
        .map(|(i, field)| {
            let field_name = self.formatter.fmt_param_name(field.name.as_str());

            let js_type_name = self.gen_js_type_str(&field.ty);

            if let Type::Struct(..) = &field.ty {
                let obj_ty: Cow<'tcx, str> = format!("{js_type_name}_obj").into();

                self.add_import(
                    obj_ty.clone(),
                    Some(
                        self.formatter
                            .fmt_file_name_extensionless(&js_type_name)
                            .into(),
                    ),
                    ImportUsage::Typescript,
                );
            }

            let is_option = matches!(&field.ty, hir::Type::DiplomatOption(..));

            let c_to_js_deref = self.gen_c_to_js_deref_for_type(&field.ty, "ptr".into(), struct_field_info.fields[i].offset);

            let c_to_js = self.gen_c_to_js_for_type(
                &field.ty,
                format!("{field_name}Deref").into(), 
                &struct_def.lifetimes
            );

            // Decide on the alloc based on the inner type if we're seeing a `DiplomatOption`
            let inner_type = if let &hir::Type::DiplomatOption(inner) = &&field.ty {
                inner.as_ref()
            } else {
                &field.ty
            };

            let alloc = match inner_type {
                hir::Type::Slice(slice) => {
                    if let Some(lt) = slice.lifetime() {
                        let hir::MaybeStatic::NonStatic(lt) = lt else {
                            panic!("'static not supported in JS backend");
                        };
                        let lt_name = struct_def.lifetimes.fmt_lifetime(lt);
                        Some(
                            format!("diplomatRuntime.CleanupArena.maybeCreateWith(functionCleanupArena, ...appendArrayMap['{lt_name}AppendArray'])")
                        )
                    } else {
                        // If there is no lifetime, this is owned, so we can clean up this field as soon as we're done passing the struct into WASM.
                        Some("functionCleanupArena".into())
                    }
                },
                hir::Type::Struct(..) => Some("functionCleanupArena".into()),
                // We take ownership
                _ => None
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

            let js_name = format!("this.#{field_name}");

            let js_to_c_write = self.gen_js_to_c_for_type(&field.ty, js_name.into(), maybe_struct_borrow_info.as_ref(), alloc.as_deref(), JsToCConversionContext::WriteToBuffer("offset", struct_field_info.fields[i].offset)).into();

            FieldInfo {
                field_name,
                field_type: &field.ty,
                js_type_name,
                c_to_js_deref,
                c_to_js,
                js_to_c_write,
                maybe_struct_borrow_info: maybe_struct_borrow_info.map(|i| i.param_info),
                is_optional: is_option
            }
        }).collect::<Vec<_>>();

        (fields, struct_field_info.struct_layout)
    }

    pub(super) fn only_primitive<P: hir::TyPosition>(&self, st: &hir::StructDef<P>) -> bool {
        if st.fields.len() != 1 {
            return false;
        }

        let first = st.fields.first().unwrap();

        match &first.ty {
            hir::Type::Primitive(..) => true,
            hir::Type::Struct(s) => match s.id() {
                hir::TypeId::Struct(s) => self.only_primitive(self.tcx.resolve_struct(s)),
                hir::TypeId::OutStruct(s) => self.only_primitive(self.tcx.resolve_out_struct(s)),
                _ => false,
            },
            _ => false,
        }
    }

    /// WASM only returns a primitive (instead of a pointer) if our struct just wraps a primitive (or nests a struct that only has one primitive as a field).
    /// This is a quick way to verify that we are grabbing a value instead of a pointer.
    pub(super) fn wraps_a_primitive(&self, st: &hir::ReturnableStructPath) -> bool {
        match st.resolve(self.tcx) {
            hir::ReturnableStructDef::OutStruct(s) => self.only_primitive(s),
            hir::ReturnableStructDef::Struct(s) => self.only_primitive(s),
            _ => false,
        }
    }

    // Going to have to be a lot of arguments for now.
    #[allow(clippy::too_many_arguments)]
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
        layout: Layout,
    ) -> String {
        #[derive(Template)]
        #[template(path = "js/struct.js.jinja", escape = "none")]
        struct ImplTemplate<'a, P: hir::TyPosition> {
            type_name: &'a str,

            typescript: bool,
            mutable: bool,
            is_out: bool,

            lifetimes: &'a LifetimeEnv,
            fields: &'a Vec<FieldInfo<'a, P>>,
            methods: &'a MethodsInfo<'a>,

            wraps_primitive: bool,
            owns_wrapped_primitive: bool,

            doc_str: String,

            /// Used by `js_class.js.jinja`. If a constructor isn't overridden by #[diplomat::attr(auto, constructor)], this is the logic that `js_class.js.jinja` will use to determine whether or not to generate constructor code.
            /// Useful for hiding the fact that an out_struct has a constructor in typescript headers, for instance.
            show_default_ctor: bool,

            size: usize,
            align: usize,
        }

        ImplTemplate {
            type_name: &self.type_name,

            typescript,
            is_out,
            mutable: !is_out,

            lifetimes: &struct_def.lifetimes,
            fields,
            methods,

            wraps_primitive: self.only_primitive(struct_def),
            owns_wrapped_primitive: !struct_def.fields.is_empty()
                && matches!(
                    struct_def.fields.first().unwrap().ty,
                    hir::Type::Primitive(..)
                ),

            doc_str: self.formatter.fmt_docs(&struct_def.docs, &struct_def.attrs),

            show_default_ctor: !typescript && !struct_def.fields.is_empty(),

            size: layout.size(),
            align: layout.align(),
        }
        .render()
        .unwrap()
    }

    /// Generate required method info for all other [`ItemGenContext::generate_*`] calls.
    ///
    /// For re-usability between `.d.ts` and `.mjs` files.
    pub(super) fn generate_method(
        &self,
        type_id: TypeId,
        method: &'tcx Method,
    ) -> Option<MethodInfo<'_>> {
        if method.attrs.disable {
            return None;
        }

        let mut visitor = method.borrowing_param_visitor(self.tcx, true);

        let _guard = self.errors.set_context_method(
            self.tcx.fmt_type_name_diagnostics(type_id),
            method.name.as_str().into(),
        );

        let abi_name = String::from(method.abi_name.as_str());

        let mut method_info = MethodInfo {
            abi_name,
            method_output_is_ffi_unit: method.output.is_ffi_unit(),
            needs_cleanup: false,
            doc_str: self.formatter.fmt_docs(&method.docs, &method.attrs),
            ..Default::default()
        };

        if let Some(param_self) = method.param_self.as_ref() {
            let self_borrow_kind = visitor.visit_param(&param_self.ty.clone().into(), "this");

            let struct_borrow = if let ParamBorrowInfo::Struct(param_info) = self_borrow_kind {
                Some(super::converter::StructBorrowContext {
                    use_env: &method.lifetime_env,
                    param_info,
                    is_method: true,
                })
            } else {
                None
            };

            // If we're the struct, we always expect to generate functionCleanupArena to generate slices.
            // It's easier to do it this way, so we don't have to check if each individual `_intoFFI` call requires this parameter or not.
            if matches!(param_self.ty, hir::SelfType::Struct(..)) {
                method_info.needs_cleanup = true;
            }

            // We don't need to clean up structs for Rust because they're represented entirely in JS form.
            method_info
                .param_conversions
                // Pretty sure we don't need to force padding because we're just passing in a pointer:
                .push(self.gen_js_to_c_self(
                    JsToCConversionContext::List,
                    struct_borrow.as_ref(),
                    &param_self.ty,
                ));

            if matches!(param_self.ty, hir::SelfType::Struct(..)) {
                method_info.needs_cleanup = true;
            }
        }

        for param in method.params.iter() {
            let base_type = self.gen_js_type_str(&param.ty);
            let param_type_str = format!(
                "{}",
                // If we're a struct, we can substitute StructType_obj (since it's the only thing we need to pass to WASM)
                if let Type::Struct(..) = &param.ty {
                    let obj_ty: Cow<'tcx, str> = format!("{base_type}_obj").into();
                    self.add_import(
                        obj_ty.clone(),
                        Some(
                            self.formatter
                                .fmt_file_name_extensionless(&base_type)
                                .into(),
                        ),
                        ImportUsage::Typescript,
                    );
                    obj_ty
                } else {
                    base_type
                }
            )
            .into();

            let param_info = ParamInfo {
                name: self.formatter.fmt_param_name(param.name.as_str()),
                ty: param_type_str,
            };

            let param_borrow_kind = visitor.visit_param(&param.ty, &param_info.name);

            // If we're a slice of strings or primitives. See [`hir::Type::Slice`].
            if let hir::Type::Slice(..) = param.ty {
                let slice_expr = self.gen_js_to_c_for_type(&param.ty, param_info.name.clone(), None, Some(
                        match param_borrow_kind {
                            // Is Rust NOT taking ownership?
                            // Then that means we can free this after the function is done.
                            ParamBorrowInfo::TemporarySlice => {
                                method_info.needs_cleanup = true;
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
                    .push(format!("{}Slice.ptr", param_info.name).into());

                method_info.slice_params.push(SliceParam {
                    name: param_info.name.clone(),
                    slice_expr: slice_expr.to_string(),
                });
            } else {
                // Set allocators for all the types we know require allocation (basically anything that's a struct in the underlying Rust):
                let alloc = if matches!(
                    param.ty,
                    hir::Type::DiplomatOption(..) | hir::Type::Struct(..)
                ) {
                    method_info.needs_cleanup = true;
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
                        JsToCConversionContext::List,
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
            // TODO: Make this hidden in typescript.
            Some(SpecialMethod::Iterator) => "#iteratorNext".to_string(),

            Some(SpecialMethod::Constructor) => "#defaultConstructor".into(),

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
    ) -> SpecialMethodInfo<'_> {
        let mut iterator = None;

        if let Some(ref val) = special_method_presence.iterator {
            iterator = Some(self.gen_success_ty(val))
        }

        SpecialMethodInfo {
            iterator,
            constructor: None,
            typescript: false,
        }
    }
}

/// Represents a parameter of a method. Used as part of [`MethodInfo`], exclusively in the method definition.
#[derive(Default, Clone)]
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
#[derive(Clone)]
pub(super) struct SliceParam<'a> {
    name: Cow<'a, str>,
    /// How to convert the JS type into a C slice.
    slice_expr: String,
}

/// Represents a Rust method that we invoke inside of WebAssembly with JS.
///
/// Has an attached template to convert it into Javascript.
#[derive(Default, Template, Clone)]
#[template(path = "js/method.js.jinja", escape = "none")]
pub(super) struct MethodInfo<'info> {
    /// Do we return the `()` type?
    pub method_output_is_ffi_unit: bool,
    /// The declaration signature. Something like `static functionName() { /* ... */ }` versus `functionName() { /* ... */ }`
    pub method_decl: String,

    /// Native C method name
    pub abi_name: String,

    /// If we need to create a `CleanupArena` (see `runtime.mjs`) to free any [`SliceParam`]s or structs that are present.
    pub needs_cleanup: bool,
    /// For calling .releaseToGarbageCollector on slices.
    pub needs_slice_collection: bool,

    pub typescript: bool,

    /// Represents all the parameters in the method definition (mostly for `.d.ts` generation, showing names and types).
    pub parameters: Vec<ParamInfo<'info>>,
    /// See [`SliceParam`] for info on how this array is used.
    pub slice_params: Vec<SliceParam<'info>>,
    /// Represents the Javascript needed to take the parameters from the method definition into C-friendly terms. See [`ItemGenContext::gen_js_to_c_for_type`] for more.
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

    doc_str: String,
}

/// See [`ItemGenContext::generate_special_method`].
/// Used in `js_class.js.jinja`
pub(super) struct SpecialMethodInfo<'a> {
    iterator: Option<Cow<'a, str>>,
    pub typescript: bool,
    pub constructor: Option<MethodInfo<'a>>,
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
    /// A version of js_to_c that writes the struct field to an arraybuffer `arrayBuffer` at offset `offset`
    js_to_c_write: String,
    /// Used in `get _fieldsForLifetime...` fields, which themselves are used in [`display_lifetime_edge`].
    maybe_struct_borrow_info: Option<StructBorrowInfo<'info>>,

    /// Used in the constructor() function to determine whether or not this field is required for construction.
    is_optional: bool,
}

/// Where the imports are going to be used in.
#[derive(PartialEq, Clone)]
pub(super) enum ImportUsage {
    /// .mjs files only
    Module,
    /// .d.ts files only
    Typescript,
    /// Both .mjs and .d.ts
    Both,
}

#[derive(Clone)]
pub(super) struct ImportInfo<'info> {
    import_type: Cow<'info, str>,
    import_file: Cow<'info, str>,
}

/// Imports are only unique if they use a different type. We don't care about anything else.
impl Ord for ImportInfo<'_> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.import_type.cmp(&other.import_type)
    }
}

impl PartialOrd for ImportInfo<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ImportInfo<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.import_type.eq(&other.import_type)
    }
}

impl Eq for ImportInfo<'_> {}

// Helpers used in templates (Askama has restrictions on Rust syntax)

/// Used in `method.js.jinja`. Used to create JS friendly interpretations of lifetime edges, to be passed into newly created JS structures (see [`JSFormatter::fmt_lifetime_edge_array`] and see [`ItemGenContext::gen_c_to_js_for_type`] for more.)
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
