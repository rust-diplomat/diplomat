use std::fmt::{Display, Write};

use askama::{self, Template};
use diplomat_core::hir::TypeContext;
use terminus::{RenderTerminusContext, TerminusInfo};

use crate::{
    common::{ErrorStore, FileMap},
    js2::{formatter::JSFormatter, FileType},
};

mod terminus;

pub struct WebDemoGenerationContext<'tcx> {
    tcx: &'tcx TypeContext,

    files: FileMap,
    errors: ErrorStore<'tcx, String>,

    formatter: JSFormatter<'tcx>,
}

impl<'tcx> WebDemoGenerationContext<'tcx> {
    pub fn run(
        tcx: &'tcx TypeContext,
        docs: &'tcx diplomat_core::ast::DocsUrlGenerator,
        strip_prefix: Option<String>,
    ) -> Result<FileMap, Vec<(impl Display + 'tcx, String)>> {
        let mut this = WebDemoGenerationContext {
            tcx,

            files: FileMap::default(),
            errors: ErrorStore::default(),

            formatter: JSFormatter::new(tcx, docs, strip_prefix),
        };

        this.init();

        let errors = this.errors.take_all();
        if errors.is_empty() {
            Ok(this.files)
        } else {
            Err(errors)
        }
    }

    /// Per https://docs.google.com/document/d/1xRTmK0YtOfuAe7ClN6kqDaHyv5HpdIRIYQW6Zc_KKFU/edit?usp=sharing
    /// Generate markup.
    ///
    /// That is, only generate .js files to be used in final rendering.
    /// This JS should include:
    /// Render Termini that can be called, and internal functions to construct dependencies that the Render Terminus function needs.
    pub fn init(&mut self) {
        #[derive(Template)]
        #[template(path = "demo-gen/index.js.jinja", escape = "none")]
        struct IndexInfo {
            termini: Vec<TerminusInfo>,
        }

        let mut out_info = IndexInfo {
            termini: Vec::new(),
        };

        for (id, ty) in self.tcx.all_types() {
            let methods = ty.methods();

            const FILE_TYPES: [FileType; 2] = [FileType::Module, FileType::Typescript];

            let mut termini = Vec::new();

            {
                let type_name = self.formatter.fmt_type_name(id);
                
                let ty = self.tcx.resolve_type(id);
                if ty.attrs().disable {
                    continue;
                }

                for method in methods {
                    if method.attrs.disable {
                        continue;
                    }

                    let val = RenderTerminusContext::evaluate_terminus(
                        self,
                        type_name.to_string(),
                        method,
                    );
                    if let Some(t) = val {
                        termini.push(t);
                    }
                }
            }

            if !termini.is_empty() {
                for file_type in FILE_TYPES {
                    let type_name = self.formatter.fmt_type_name(id);
                    let file_name = self.formatter.fmt_file_name(&type_name, &file_type);

                    let mut method_str = String::new();

                    for terminus in &mut termini {
                        terminus.typescript = file_type.is_typescript();
                        writeln!(method_str, "{}", terminus.render().unwrap()).unwrap();
                    }

                    self.files.add_file(file_name.to_string(), method_str);
                }

                out_info.termini.append(&mut termini);
            }
        }

        self.files
            .add_file("index.mjs".into(), out_info.render().unwrap());
    }
}
