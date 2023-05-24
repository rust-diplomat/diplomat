mod formatter;
mod header;
mod ty;

pub use self::formatter::CFormatter;

use crate::common::{ErrorStore, FileMap};
use diplomat_core::hir::TypeContext;
use std::cell::RefCell;
use std::collections::HashMap;

/// This is the main object that drives this backend. Most execution steps
/// for this backend will be found as methods on this context
pub struct CContext<'tcx> {
    pub tcx: &'tcx TypeContext,
    pub formatter: CFormatter<'tcx>,
    pub files: FileMap,
    // The results needed by various methods
    pub result_store: RefCell<HashMap<String, ty::ResultType<'tcx>>>,

    pub errors: ErrorStore<'tcx, String>,
}

impl<'tcx> CContext<'tcx> {
    pub fn new(tcx: &'tcx TypeContext, files: FileMap) -> Self {
        CContext {
            tcx,
            files,
            formatter: CFormatter::new(tcx),
            result_store: Default::default(),
            errors: ErrorStore::default(),
        }
    }

    /// Run file generation
    ///
    /// Will populate self.files as a result
    pub fn run(&self) {
        self.files
            .add_file("diplomat_runtime.h".into(), crate::c::RUNTIME_H.into());
        for (id, ty) in self.tcx.all_types() {
            self.gen_ty(id, ty)
        }

        for (result_name, result_ty) in self.result_store.borrow().iter() {
            self.gen_result(result_name, *result_ty)
        }
    }

    // further methods can be found in ty.rs and formatter.rs
}
