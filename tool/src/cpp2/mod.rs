mod formatter;
mod header;
mod ty;

use formatter::Cpp2Formatter;
use crate::common::FileMap;
use diplomat_core::hir::TypeContext;

/// This is the main object that drives this backend. Most execution steps
/// for this backend will be found as methods on this context
pub struct Cpp2Context<'tcx> {
    pub tcx: &'tcx TypeContext,
    pub formatter: Cpp2Formatter<'tcx>,
    pub files: FileMap,
}

impl<'tcx> Cpp2Context<'tcx> {
    pub fn new(tcx: &'tcx TypeContext, files: FileMap) -> Self {
        Cpp2Context {
            tcx,
            files,
            formatter: Cpp2Formatter::new(tcx),
        }
    }

    /// Run file generation
    ///
    /// Will populate self.files as a result
    pub fn run(&self) {
        for (id, ty) in self.tcx.all_types() {
            self.files.add_file(format!("{}.x", self.formatter.fmt_type_name(id)), format!("{:?}", ty));
        }
        for (id, ty) in self.tcx.all_types() {
            self.gen_ty(id, ty)
        }
    }

    // further methods can be found in ty.rs and formatter.rs
}
