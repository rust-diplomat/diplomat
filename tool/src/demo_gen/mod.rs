use std::{borrow::Cow, fmt::{Display, Write}};

use diplomat_core::hir::{self, Method, TypeContext, TypeId};

use crate::{common::{ErrorStore, FileMap}, js2::formatter::JSFormatter};

pub struct WebDemoGenerationContext<'tcx> {
    tcx: &'tcx TypeContext,

    files : FileMap,
    errors: ErrorStore<'tcx, String>,
    
    formatter : JSFormatter<'tcx>,
    exports : Vec<Cow<'tcx, str>>,
}

impl<'tcx> WebDemoGenerationContext<'tcx> {
    pub fn run(tcx: &'tcx TypeContext, docs : &'tcx diplomat_core::ast::DocsUrlGenerator, strip_prefix : Option<String>) -> Result<FileMap, Vec<(impl Display + 'tcx, String)>> {
        let mut this = WebDemoGenerationContext {
            tcx,

            files: FileMap::default(),
            errors: ErrorStore::default(),

            formatter : JSFormatter::new(tcx, docs, strip_prefix),
            exports: Vec::new(),
        };

        this.init();

        let errors = this.errors.take_all();
        if errors.is_empty() {
            return Ok(this.files);
        } else {
            return Err(errors);
        }
    }

    /// Per https://docs.google.com/document/d/1xRTmK0YtOfuAe7ClN6kqDaHyv5HpdIRIYQW6Zc_KKFU/edit?usp=sharing
    /// Generate markup.
    /// 
    /// That is, only generate .js files to be used in final rendering.
    /// This JS should include:
    /// Render Termini that can be called, and internal functions to construct dependencies that the Render Terminus function needs. 
    pub fn init(&mut self) {
        // So, here's what I'm thinking.

        // 1. Search through all methods that can be classified as a render terminus.
        for (id, ty) in self.tcx.all_types() {
            let methods = ty.methods();

            let mut method_str = String::new();
            for method in methods {
                let val = self.evaluate_method(method);
                if let Some(s) = val  {
                    writeln!(method_str, "{}", s).unwrap();
                }
            }

            if method_str.len() > 0 {
                let type_name = self.formatter.fmt_type_name(id);
                let file_name = self.formatter.fmt_file_name(&type_name, &crate::js2::FileType::Module);
                
                self.exports.push(format!(r#"export * as {type_name}Demo from "./{file_name}""#).into());

                self.files.add_file(format!("demo/{file_name}"), method_str);
            }
        }

        let mut out_str = String::new();
        for export in self.exports.iter() {
            writeln!(out_str, "{}", export).unwrap();
        }
        self.files.add_file("demo/index.mjs".into(), out_str);
    }

    /// Create a Render Terminus .js file from a method.
    /// We define this (for now) as any function that outputs [`hir::SuccessType::Write`]
    pub fn evaluate_method(&self, method : &Method) -> Option<String> {
        if !method.output.success_type().is_write() {
            return None;
        }

        let method_name = self.formatter.fmt_method_name(method);

        return Some(format!("export function {method_name}() {{}}"));
    }
}