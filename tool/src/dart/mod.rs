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
    ) -> Self {
        DartContext {
            tcx,
            files,
            formatter: DartFormatter::new(tcx, docs_url_generator),
            errors: ErrorStore::default(),
        }
    }

    /// Run file generation
    ///
    /// Will populate self.files as a result
    pub fn run(&self) {
        self.files.add_file(
            "lib.dart".to_string(),
            self.tcx
                .all_types()
                .filter(|(_, ty)| !ty.attrs().disable)
                .map(|(id, _)| self.gen_ty(id))
                .collect::<std::collections::BTreeSet<_>>()
                .into_iter()
                .fold(class::Class::init(), class::Class::append)
                .render(),
        );
    }
}
