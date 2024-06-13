use std::{borrow::Cow, fmt::{Display, Write}};

use askama::Template;
use diplomat_core::hir::{self, Method, Param, SelfType, Type, TypeContext, TypeId};
use terminus::RenderTerminusContext;

use crate::{common::{ErrorStore, FileMap}, js2::{formatter::JSFormatter, FileType}};

mod terminus;

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

            const FILE_TYPES : [FileType; 2] = [FileType::Module, FileType::Typescript];
            
            let mut termini : Vec<terminus::TerminusInfo> = Vec::new();

            {
                for method in methods {
                    let val = RenderTerminusContext::evaluate_terminus(self, method);
                    if let Some(t) = val  {
                        termini.push(t.to_owned());
                    }
                }
            }

            if termini.len() > 0 {
                for file_type in FILE_TYPES {
                    let type_name = self.formatter.fmt_type_name(id);
                    let file_name = self.formatter.fmt_file_name(&type_name, &file_type);
                    
                    if !file_type.is_typescript(){
                        self.exports.push(format!(r#"export * as {type_name}Demo from "./{file_name}""#).into());
                    }

                    let mut method_str = String::new();

                    for terminus in &mut termini {
                        terminus.typescript = file_type.is_typescript();
                        writeln!(method_str, "{}", terminus.render().unwrap()).unwrap();
                    }

                    self.files.add_file(format!("{file_name}"), method_str);
                }
            }
        }

        let mut out_str = String::new();
        for export in self.exports.iter() {
            writeln!(out_str, "{}", export).unwrap();
        }
        self.files.add_file("index.mjs".into(), out_str);
    }
}