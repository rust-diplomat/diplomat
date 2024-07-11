mod formatter;
mod header;
mod ty;

use crate::common::{ErrorStore, FileMap};
use askama::Template;
use diplomat_core::hir::TypeContext;
use formatter::Cpp2Formatter;

pub(crate) static RUNTIME_HPP: &str = include_str!("../../templates/cpp2/runtime.hpp");

/// This is the main object that drives this backend. Most execution steps
/// for this backend will be found as methods on this context
pub struct Cpp2Context<'tcx> {
    pub tcx: &'tcx TypeContext,
    pub formatter: Cpp2Formatter<'tcx>,
    pub files: FileMap,
    pub errors: ErrorStore<'tcx, String>,
}

impl<'tcx> Cpp2Context<'tcx> {
    pub fn new(tcx: &'tcx TypeContext, files: FileMap) -> Self {
        Cpp2Context {
            tcx,
            files,
            formatter: Cpp2Formatter::new(tcx),
            errors: ErrorStore::default(),
        }
    }

    /// Run file generation
    ///
    /// Will populate self.files as a result
    pub fn run(&self) {
        let mut c_runtime = String::new();
        crate::c2::RuntimeTemplate { is_for_cpp: true }
            .render_into(&mut c_runtime)
            .unwrap();

        self.files
            .add_file("diplomat_c_runtime.hpp".into(), c_runtime);

        self.files
            .add_file("diplomat_runtime.hpp".into(), RUNTIME_HPP.into());

        for (id, ty) in self.tcx.all_types() {
            self.gen_ty(id, ty)
        }
    }

    // further methods can be found in ty.rs and formatter.rs
}
