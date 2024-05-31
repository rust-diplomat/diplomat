use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt::Display;

use converter::StructBorrowContext;
use diplomat_core::ast::{DocsUrlGenerator, Param};

use diplomat_core::hir::borrowing_param::{BorrowedLifetimeInfo, LifetimeEdge, LifetimeEdgeKind, ParamBorrowInfo, StructBorrowInfo};
use diplomat_core::hir::{self, EnumDef, LifetimeEnv, Method, OpaqueDef, ReturnType, SpecialMethod, SpecialMethodPresence, SuccessType, Type, TypeContext, TypeDef, TypeId};

use askama::{self, Template};

use crate::common::{ErrorStore, FileMap};

use self::formatter::JSFormatter;

mod formatter;
mod converter;

/// Wrapper for generating all export types.
/// 
/// .d.ts definitions, basically. Although we include .mjs so we can do actual conversions to WebAssembly friendly definitions.
pub struct JSGenerationContext<'tcx> {
    tcx: &'tcx TypeContext,
    formatter : JSFormatter<'tcx>,

    errors : ErrorStore<'tcx, String>,

    files : FileMap,
}

/// Since the main difference between .mjs and .d.ts is typing, we just want a differentiator for our various helper functions as to what's being generated: .d.ts, or .mjs?
enum FileType {
    Module,
    Typescript
}

impl FileType {
    fn is_typescript(&self) -> bool {
        match self {
            FileType::Module => false,
            FileType::Typescript => true
        }
    }
}

impl<'tcx> JSGenerationContext<'tcx> {
    pub fn run(tcx : &'tcx TypeContext, docs : &'tcx DocsUrlGenerator, strip_prefix : Option<String>) -> Result<FileMap, Vec<(impl Display + 'tcx, String)>> {
        let this = Self {
            tcx,
            formatter: JSFormatter::new(tcx, docs, strip_prefix),

            errors: ErrorStore::default(),

            files: FileMap::default(),
        };
        this.init();

        let errors = this.errors.take_all();
        if errors.is_empty() {
            return Ok(this.files);
        } else {
            return Err(errors);
        }
    }

    /// Setup. Write out all the pre-written files.
    /// 
    /// Then iterate through all the types we get from the TypeContext to create separate out files.
    pub fn init(&self) {
        self.files.add_file("diplomat-runtime.mjs".into(), include_str!("../../templates/js2/runtime.mjs").into());
        self.files.add_file("diplomat-runtime.d.ts".into(), include_str!("../../templates/js2/runtime.d.ts").into());
        self.files.add_file("diplomat-wasm.mjs".into(), include_str!("../../templates/js2/wasm.mjs").into());

        
        // TODO: All of this.

        for (id, ty) in self.tcx.all_types() {
            let _guard = self.errors.set_context_ty(ty.name().as_str().into());
            if ty.attrs().disable {
                continue;
            }

            self.generate_file_from_type(id);
        }
        
        self.files.add_file("index.mjs".into(), "export { FFIError } from './diplomat-runtime.mjs';".into());
        self.files.add_file("index.d.ts".into(), "".into());
    }

    /// Generate a file's name and body from its given [`TypeId`]
    fn generate_file_from_type(&self, type_id : TypeId) {
        let type_def = self.tcx.resolve_type(type_id);

        let _guard = self.errors.set_context_ty(type_def.name().as_str().into());

        let name = self.formatter.fmt_type_name(type_id);

        const FILE_TYPES : [FileType; 2] = [FileType::Module, FileType::Typescript];
        for file_type in FILE_TYPES {
            let contents = match type_def {
                TypeDef::Enum(enum_def) => {
                    self.generate_enum_from_def(enum_def, type_id, &name, &file_type)
                },
                TypeDef::Opaque(opaque_def) => {
                    self.generate_opaque_from_def(opaque_def, type_id, &name, &file_type)
                },
                TypeDef::Struct(struct_def) => {
                    self.generate_struct_from_def(struct_def, type_id, false, &name, true, &file_type)
                }
                // TODO:
                _ => format!("{} has a TypeDef that is unimplemented. I am working on it!", type_def.name())
            };
            self.files.add_file(self.formatter.fmt_file_name(&name, file_type), self.generate_base(contents));
        }
    }

    fn generate_base(&self, body : String) -> String {
        #[derive(Template)]
        #[template(path="js2/base.js.jinja", escape="none")]
        struct BaseTemplate {
            body : String,
        }
        BaseTemplate {body}.render().unwrap()
    }

    // #region Base generator functions (enum, opaque, struct, outstruct)
    /// Generate an enumerator's body for a file from the given definition. Called by [`JSGenerationContext::generate_file_from_type`]
    fn generate_enum_from_def(&self, enum_def : &'tcx EnumDef, type_id : TypeId, type_name : &str, file_type : &FileType) -> String {
        let mut methods = enum_def.methods
        .iter()
        .flat_map(|method| self.generate_method_body(type_id, type_name, method, file_type.is_typescript()))
        .collect::<Vec<_>>();

        let special_method_body = self.generate_special_method_body(&enum_def.special_method_presence, file_type.is_typescript());
        methods.push(special_method_body);

        #[derive(Template)]
        #[template(path="js2/enum.js.jinja", escape="none")]
        struct ImplTemplate<'a> {
            enum_def: &'a EnumDef,
            formatter : &'a JSFormatter<'a>,
            type_name : &'a str,
            typescript : bool,

            doc_str : String,

            methods : Vec<String>
        }

        ImplTemplate{
            enum_def,
            formatter: &self.formatter,
            type_name,
            typescript: file_type.is_typescript(),

            doc_str: self.formatter.fmt_docs(&enum_def.docs),

            methods
        }.render().unwrap()
    }

    fn generate_opaque_from_def(&self, opaque_def: &'tcx OpaqueDef, type_id : TypeId, type_name : &str, file_type : &FileType) -> String {
        let mut methods = opaque_def.methods.iter()
        .flat_map(|method| { self.generate_method_body(type_id, type_name, method, file_type.is_typescript()) })
        .collect::<Vec<_>>();

        let special_method_body = self.generate_special_method_body(&opaque_def.special_method_presence, file_type.is_typescript());
        methods.push(special_method_body);

        let destructor = self.formatter.fmt_destructor_name(type_id);

        #[derive(Template)]
        #[template(path = "js2/opaque.js.jinja", escape="none")]
        struct ImplTemplate<'a> {
            type_name: &'a str,
            typescript : bool,

            lifetimes : &'a LifetimeEnv,
            methods : Vec<String>,
            destructor : String,

            docs : String,
        }

        ImplTemplate {
            type_name,
            methods,
            destructor,
            typescript: file_type.is_typescript(),
            docs: self.formatter.fmt_docs(&opaque_def.docs),
            lifetimes : &opaque_def.lifetimes,
        }.render().unwrap()
    }

    fn generate_struct_from_def(&self, struct_def : &'tcx hir::StructDef, type_id : TypeId, is_out : bool, type_name : &str, mutable: bool, file_type : &FileType) -> String {
        struct FieldInfo<'info, P: hir::TyPosition> {
            field_name: Cow<'info, str>,
            field_type : &'info Type<P>,
            annotation : Option<&'static str>,
            js_type_name  : Cow<'info, str>,
            c_to_js : Cow<'info, str>,
            js_to_c : Vec<String>,
            maybe_struct_borrow_info : Option<StructBorrowInfo<'info>>,
        };

        let fields = struct_def.fields.iter()
        .map(|field| {
            let field_name = self.formatter.fmt_param_name(field.name.as_str());

            let field_annotation = match field.ty {
                hir::Type::Primitive(p) => Some(self.formatter.fmt_primitive_as_ffi(p)),
                hir::Type::Enum(_) => Some(self.formatter.fmt_enum_as_ffi()),
                _ => None,
            };

            // Don't need for JS
            // let ffi_cast_type_name = if let hir::Type::Slice(s) = field.ty {
            //     todo!()
            // } else {
            //     self.gen_type_name_ffi(&field.ty, true)
            // };

            let js_type_name = self.gen_js_type_str(&field.ty);

            let c_to_js = self.gen_c_to_js_for_type(
                &field.ty, 
                field_name.clone().into(), 
                &struct_def.lifetimes
            );

            let (js_to_c, maybe_struct_borrow_info) = if let hir::Type::Slice(slice) = &field.ty {
                let slice_expr = self.gen_js_to_c_for_type(&field.ty, field_name.clone(), None);

                let mut ret = vec![
                    format!("/*TODO: struct Slice fields*/"),
                ];
                (ret, None)
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
                (vec!(format!("/*TODO: Other struct fields. {}*/",
                    self.gen_js_to_c_for_type(&field.ty, field_name.clone(), borrow_info.as_ref())
                )), borrow_info.map(|s| s.param_info))
            };

            FieldInfo {
                field_name,
                field_type: &field.ty,
                annotation: field_annotation,
                js_type_name,
                c_to_js,
                js_to_c,
                maybe_struct_borrow_info
            }
        }).collect::<Vec<_>>();

        let mut methods = struct_def.methods
        .iter()
        .flat_map(|method| self.generate_method_body(type_id, type_name, method, file_type.is_typescript()))
        .collect::<Vec<_>>();

        methods.push(self.generate_special_method_body(&struct_def.special_method_presence, file_type.is_typescript()));

        // TODO: Default constructors? (Could be expanded with Opaque default constructors??)

        #[derive(Template)]
        #[template(path="js2/struct.js.jinja", escape = "none")]
        struct ImplTemplate<'a, P: hir::TyPosition> {
            type_name : &'a str,
            mutable : bool,
            fields : Vec<FieldInfo<'a, P>>,
            methods: Vec<String>,
            docs: String,
            lifetimes : &'a LifetimeEnv,
        }

        ImplTemplate {
            type_name,
            mutable,
            fields,
            methods,
            docs: self.formatter.fmt_docs(&struct_def.docs),
            lifetimes: &struct_def.lifetimes
        }.render().unwrap()
    }

    // #endregion

    /// Generate a string Javascript representation of a given method.
    /// 
    /// Currently, this assumes that any method will be part of a class. That will probably be a parameter that's added, however.
    fn generate_method_body(&self, type_id : TypeId, type_name : &str, method : &'tcx Method, typescript : bool) -> Option<String> {
        if method.attrs.disable {
            return None;
        }

        let mut visitor = method.borrowing_param_visitor(self.tcx);

        let _guard = self.errors.set_context_method(self.formatter.fmt_type_name_diagnostics(type_id), method.name.as_str().into());

        #[derive(Default)]
        struct ParamInfo<'a> {
            ty : Cow<'a, str>,
            name : Cow<'a, str>
        }

        struct SliceParam<'a> {
            name : Cow<'a, str>,
            /// How to convert the JS type into a C slice.
            slice_expr : Cow<'a, str>,
            is_borrowed : bool,
        }

        #[derive(Default, Template)]
        #[template(path="js2/method.js.jinja", escape="none")]
        struct MethodInfo<'info> {
            method : Option<&'info Method>,
            method_decl : String,
            /// Native C method name
            c_method_name : Cow<'info, str>,

            typescript : bool,

            parameters : Vec<ParamInfo<'info>>,
            slice_params : Vec<SliceParam<'info>>,
            param_conversions : Vec<Cow<'info, str>>,

            return_type : Cow<'info, str>,
            return_expression : Option<Cow<'info, str>>,

            method_lifetimes_map : BTreeMap<hir::Lifetime, BorrowedLifetimeInfo<'info>>,
            lifetimes : Option<&'info LifetimeEnv>,
            cleanup_expressions : Vec<Cow<'info, str>>
        }

        let mut method_info = MethodInfo::default();

        method_info.c_method_name = self.formatter.fmt_c_method_name(type_id, method);
        method_info.typescript = typescript;
        method_info.method = Some(method);

        if let Some(param_self) = method.param_self.as_ref() {
            visitor.visit_param(&param_self.ty.clone().into(), "this");

            method_info.param_conversions.push(self.gen_js_to_c_self(&param_self.ty));
            if matches!(param_self.ty, hir::SelfType::Struct(..)) {
                // TODO: Does this work?
                method_info.cleanup_expressions.push(
                    "this.free(); /* TODO: Does this work? */".into()
                );
            }
        }

        for param in method.params.iter() {
            let mut param_info = ParamInfo::default();

            param_info.name = self.formatter.fmt_param_name(param.name.as_str());;
            param_info.ty = self.gen_js_type_str(&param.ty);
            
            let param_borrow_kind = visitor.visit_param(&param.ty, &param_info.name);

            // If we're a slice of strings or primitives. See [`hir::Types::Slice`].
            if let hir::Type::Slice(slice) = param.ty {
                let slice_expr = self.gen_js_to_c_for_type(&param.ty, param_info.name.clone(), None);

                let is_borrowed = match param_borrow_kind {
                    ParamBorrowInfo::TemporarySlice => false,
                    ParamBorrowInfo::BorrowedSlice => true,
                    _ => unreachable!(
                        "Slices must produce slice ParamBorrowInfo, found {param_borrow_kind:?}"
                    ),
                };
                // We add the pointer and size for slices:
                method_info.param_conversions.push(format!("{}Slice.ptr", param_info.name).into());
                method_info.param_conversions.push(format!("{}Slice.size", param_info.name).into());

                // Then we make sure to handle clean-up for the slice:
                if is_borrowed {
                    // Is this function borrowing the slice?
                    // I.e., Do we need it alive for at least as long as this function call?
                    method_info.cleanup_expressions.push(
                        format!("{}Slice.garbageCollect();", param_info.name).into()
                    );
                } else if !slice.lifetime().is_none() {
                    // Is Rust NOT taking ownership?
                    // Then that means we can free this after the function is done.
                    method_info.cleanup_expressions.push(
                        format!("{}Slice.free();", param_info.name).into()
                    );
                }

                method_info.slice_params.push(SliceParam {
                    name: param_info.name.clone(),
                    slice_expr,
                    is_borrowed,
                });
            } else {
                if let hir::Type::Struct(..) = param.ty {
                    // TODO: Does this work?
                    method_info.cleanup_expressions.push(
                        "this.free(); /* TODO: Does this work? */".into()
                    );
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
                method_info.param_conversions.push(
                    self.gen_js_to_c_for_type(&param.ty, param_info.name.clone(), struct_borrow_info.as_ref())
                );
            }
            
            method_info.parameters.push(param_info);
        }

        if method.output.is_write() {
            // TODO:
        }

        method_info.return_type = self.gen_js_return_type_str(&method.output);
        method_info.return_expression = self.gen_c_to_js_for_return_type(&method.output, &method.lifetime_env);
        
        method_info.method_lifetimes_map = visitor.borrow_map();
        method_info.lifetimes = Some(&method.lifetime_env);

        method_info.method_decl = match &method.attrs.special_method {
            Some(SpecialMethod::Getter(name))
            =>format!("get {}", self.formatter.fmt_method_field_name(name, method)),
            Some(SpecialMethod::Setter(name))
            => format!("set {}", self.formatter.fmt_method_field_name(name, method)),

            Some(SpecialMethod::Iterable) => format!("[Symbol.iterator]"),
            Some(SpecialMethod::Iterator) => format!("#iteratorNext"),
            
            None if method.param_self.is_none() => format!(
                "static {}",
                self.formatter.fmt_method_name(method)
            ),
            None => self.formatter.fmt_method_name(method),
            _ => todo!("Method Declaration {:?} not implemented", method.attrs.special_method),
        };

        Some(method_info.render().unwrap())
    }
    
    /// If a special method exists inside a structure, opaque, or enum through [`SpecialMethodPresence`],
    /// We need to make sure Javascript can access it.
    /// 
    /// This is mostly for iterators, using https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Iteration_protocols
    fn generate_special_method_body(&self, special_method_presence : &SpecialMethodPresence, typescript : bool) -> String {
        #[derive(Template)]
        #[template(path="js2/special_method.js.jinja", escape="none")]
        struct SpecialMethodInfo<'a> {
            iterator : Option<Cow<'a, str>>,
            iterable : Option<Cow<'a, str>>,
            typescript : bool,
        }

        let mut iterator = None;

        if let Some(ref val) = special_method_presence.iterator {
            iterator = Some(self.gen_success_ty(val))
        }

        let mut iterable = None;
        if let Some(ref iterator) = special_method_presence.iterable {
            let iterator_def = self.tcx.resolve_opaque(*iterator);
            let Some(ref val) = iterator_def.special_method_presence.iterator else {
                self.errors
                    .push_error("Found iterable not returning an iterator type".into());
                return "".to_string();
            };
            iterable = Some(self.gen_success_ty(val))
        }

        SpecialMethodInfo {
            iterator,
            iterable,
            typescript
        }.render().unwrap()
    }
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