mod formatter;
mod header;
mod ty;

pub use self::formatter::CFormatter;

use crate::common::{ErrorStore, FileMap};
use askama::Template;
use diplomat_core::hir::TypeContext;

/// This is the main object that drives this backend. Most execution steps
/// for this backend will be found as methods on this context
pub struct CContext<'tcx> {
    pub tcx: &'tcx TypeContext,
    pub formatter: CFormatter<'tcx>,
    pub files: FileMap,

    pub errors: ErrorStore<'tcx, String>,
    /// Whether this is being generated for C++ (needs extern C, namespaces, etc)
    pub is_for_cpp: bool,
}

#[derive(Template)]
#[template(path = "c2/runtime.h.jinja", escape = "none")]
struct RuntimeTemplate {
    is_for_cpp: bool,
}

impl<'tcx> CContext<'tcx> {
    pub fn new(tcx: &'tcx TypeContext, files: FileMap, is_for_cpp: bool) -> Self {
        CContext {
            tcx,
            files,
            formatter: CFormatter::new(tcx),
            errors: ErrorStore::default(),
            is_for_cpp,
        }
    }

    /// Run file generation
    ///
    /// Will populate self.files as a result
    pub fn run(&self) {
        let mut runtime = String::new();
        RuntimeTemplate {
            is_for_cpp: self.is_for_cpp,
        }
        .render_into(&mut runtime)
        .unwrap();

        self.files.add_file("diplomat_runtime.h".into(), runtime);
        for (id, ty) in self.tcx.all_types() {
            self.gen_ty(id, ty)
        }
    }

    // further methods can be found in ty.rs and formatter.rs
}
