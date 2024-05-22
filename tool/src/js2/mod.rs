use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt::Display;

use diplomat_core::ast::{DocsUrlGenerator, Param};

use diplomat_core::hir::borrowing_param::{BorrowedLifetimeInfo, LifetimeEdge, LifetimeEdgeKind};
use diplomat_core::hir::{self, EnumDef, LifetimeEnv, Method, OpaqueDef, ReturnType, SpecialMethodPresence, SuccessType, Type, TypeContext, TypeDef, TypeId};

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
    
        // Javascript is fairly easy-going, so instead of using a separate function for the presence of say, Iterators or Iterables in a separate part of defining our class,
        // We just use https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Iteration_protocols

        // let special_method_body = self.generate_special_method_body(&enum_def.special_method_presence);
        // methods.push(special_method_body);

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

        #[derive(Default, Template)]
        #[template(path="js2/method.js.jinja", escape="none")]
        struct MethodInfo<'info> {
            method : Option<&'info Method>,
            method_name : String,
            /// Native C method name
            c_method_name : Cow<'info, str>,

            typescript : bool,
            is_static : bool,

            parameters : Vec<ParamInfo<'info>>,
            return_type : Cow<'info, str>,
            return_expression : Option<Cow<'info, str>>,

            method_lifetimes_map : BTreeMap<hir::Lifetime, BorrowedLifetimeInfo<'info>>,
            lifetimes : Option<&'info LifetimeEnv>,
        }

        let mut method_info = MethodInfo::default();

        method_info.c_method_name = self.formatter.fmt_c_method_name(type_id, method);
        method_info.method_name = self.formatter.fmt_method_name(method);
        method_info.typescript = typescript;
        method_info.method = Some(method);

        method_info.is_static = true;
        if let Some(param_self) = method.param_self.as_ref() {
            visitor.visit_param(&param_self.ty.clone().into(), "this");
            method_info.is_static = false;
        }

        for param in method.params.iter() {
            let mut param_info = ParamInfo::default();

            param_info.name = self.formatter.fmt_param_name(param.name.as_str());
            param_info.ty = self.gen_js_type_str(&param.ty);

            // If we're a slice of strings or primitives. See [`hir::Types::Slice`].
            if let hir::Type::Slice(slice) = param.ty {
                // TODO:
            } else {

            }
            
            method_info.parameters.push(param_info);
        }

        method_info.return_type = self.gen_js_return_type_str(&method.output);
        method_info.return_expression = self.gen_c_to_js_for_return_type(&method.output, &method.lifetime_env);
        
        method_info.method_lifetimes_map = visitor.borrow_map();
        method_info.lifetimes = Some(&method.lifetime_env);

        Some(method_info.render().unwrap())
    }

    fn gen_success_ty(&self, out_ty: &SuccessType) -> Cow<'tcx, str> {
        todo!();
        // match out_ty {
        //     SuccessType::Writeable => self.formatter.fmt_string().into(),
        //     SuccessType::OutType(o) => self.gen_type_name(o),
        //     SuccessType::Unit => self.formatter.fmt_void().into(),
        //     _ => unreachable!(),
        // }
    }
}

// Helpers used in templates (Askama has restrictions on Rust syntax)

/// Modified from dart backend
fn display_lifetime_edge<'a>(edge: &'a LifetimeEdge) -> Cow<'a, str> {
    let param_name = &edge.param_name;
    match edge.kind {
        // Opaque parameters are just retained as edges
        LifetimeEdgeKind::OpaqueParam => param_name.into(),
        _ => todo!("Lifetime edge kind {:?} not yet implemented", edge.kind),
        // Slice parameters make an arena which is retained as an edge
        // LifetimeEdgeKind::SliceParam => format!("{param_name}Arena").into(),
        // We extract the edge-relevant fields for a borrowed struct lifetime
        // LifetimeEdgeKind::StructLifetime(def_env, def_lt) => format!(
        //     "...{param_name}._fieldsForLifetime{}",
        //     def_env.fmt_lifetime(def_lt).to_uppercase(),
        // )
        // .into(),
        // _ => unreachable!("Unknown lifetime edge kind {:?}", edge.kind),
    }
}