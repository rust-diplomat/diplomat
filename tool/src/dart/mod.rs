mod class;
mod formatter;

use crate::common::{ErrorStore, FileMap};
use diplomat_core::ast::DocsUrlGenerator;
use diplomat_core::hir::TypeContext;
use formatter::DartFormatter;

/// This is the main object that drives this backend. Most execution steps
/// for this backend will be found as methods on this context
pub(crate) struct DartContext<'tcx> {
    tcx: &'tcx TypeContext,
    formatter: DartFormatter<'tcx>,
    pub files: FileMap,
    pub errors: ErrorStore<'tcx, String>,
}

impl<'tcx> DartContext<'tcx> {
    pub fn new(
        tcx: &'tcx TypeContext,
        files: FileMap,
        docs_url_generator: &'tcx DocsUrlGenerator,
        strip_prefix: Option<String>,
    ) -> Self {
        DartContext {
            tcx,
            files,
            formatter: DartFormatter::new(tcx, docs_url_generator, strip_prefix),
            errors: ErrorStore::default(),
        }
    }

    /// Run file generation
    ///
    /// Will populate self.files as a result
    pub fn run(&self) {
        let mut directives = Default::default();
        let mut helper_classes = Default::default();

        for (id, ty) in self.tcx.all_types() {
            if ty.attrs().disable {
                continue;
            }

            self.gen_ty(id, &mut directives, &mut helper_classes);
        }

        self.gen_root(directives, helper_classes);
    }
}
