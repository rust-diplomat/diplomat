use std::fmt::Display;

use diplomat_core::hir::TypeContext;

use crate::common::{ErrorStore, FileMap};

pub struct WebDemoGenerationContext<'tcx> {
    tcx: &'tcx TypeContext,

    files : FileMap,
    errors: ErrorStore<'tcx, String>,
}

impl<'tcx> WebDemoGenerationContext<'tcx> {
    pub fn run(tcx: &'tcx TypeContext) -> Result<FileMap, Vec<(impl Display + 'tcx, String)>> {
        let mut this = WebDemoGenerationContext {
            tcx,

            files: FileMap::default(),
            errors: ErrorStore::default(),
        };

        this.files.add_file("test.html".into(), "".into());

        let errors = this.errors.take_all();
        if errors.is_empty() {
            return Ok(this.files);
        } else {
            return Err(errors);
        }
    }
}