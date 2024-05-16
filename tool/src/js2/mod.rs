use std::collections::HashMap;
use std::path::Path;

use diplomat_core::Env;

use diplomat_core::hir::{self, EnumDef, TypeContext, TypeDef, TypeId};

use askama::{self, Template};

use crate::common::FileMap;

use self::formatter::JSFormatter;

mod formatter;

/// Wrapper for generating all export types.
/// 
/// .d.ts definitions, basically. Although we include .mjs so we can do actual conversions to WebAssembly friendly definitions.
pub struct JSGenerationContext<'tcx> {
    pub tcx: &'tcx TypeContext,
    formatter : JSFormatter<'tcx>,

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
    pub fn run(tcx : &'tcx TypeContext, strip_prefix : Option<String>) -> FileMap {
        let this = Self {
            tcx,
            formatter: JSFormatter::new(tcx, strip_prefix),

            files: FileMap::default(),
        };
        this.init();

        this.files
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

        let name = self.formatter.fmt_type_name(type_id);

        const FILE_TYPES : [FileType; 2] = [FileType::Module, FileType::Typescript];
        for file_type in FILE_TYPES {
            let contents = match type_def {
                TypeDef::Enum(enum_def) => {
                    self.generate_enum_from_def(enum_def, type_id, &name, &file_type)
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

    /// Generate an enumerator's body for a file from the given definition. Called by [`JSGenerationContext::generate_file_from_type`]
    fn generate_enum_from_def(&self, enum_def : &EnumDef, type_id : TypeId, type_name : &str, file_type : &FileType) -> String {
        // TODO: Methods

        // TODO: Finish templating
        #[derive(Template)]
        #[template(path="js2/enum.js.jinja", escape="none")]
        struct ImplTemplate<'a> {
            enum_def: &'a EnumDef,
            formatter : &'a JSFormatter<'a>,
            type_name : &'a str,
            typescript : bool,
        }

        ImplTemplate{
            enum_def,
            formatter: &self.formatter,
            type_name,
            typescript: file_type.is_typescript()
        }.render().unwrap()
    }
}
