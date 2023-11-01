mod class;
mod formatter;

use crate::common::{ErrorStore, FileMap};
use diplomat_core::hir::TypeContext;
use formatter::DartFormatter;

const RUNTIME_DART: &str = include_str!("diplomat_runtime.dart");

/// This is the main object that drives this backend. Most execution steps
/// for this backend will be found as methods on this context
pub(crate) struct DartContext<'tcx> {
    tcx: &'tcx TypeContext,
    formatter: DartFormatter<'tcx>,
    pub files: FileMap,
    pub errors: ErrorStore<'tcx, String>,
}

impl<'tcx> DartContext<'tcx> {
    pub fn new(tcx: &'tcx TypeContext, files: FileMap) -> Self {
        DartContext {
            tcx,
            files,
            formatter: DartFormatter::new(tcx),
            errors: ErrorStore::default(),
        }
    }

    /// Run file generation
    ///
    /// Will populate self.files as a result
    pub fn run(&self) {
        self.files
            .add_file("src/diplomat_runtime.dart".into(), RUNTIME_DART.into());

        self.files.add_file(
            "lib.dart".into(),
            self.tcx
                .all_types()
                .map(|(id, _)| {
                    let name = self.formatter.fmt_type_name(id);
                    format!("export 'src/{name}.dart' show {name};\n")
                })
                .chain(["export 'src/diplomat_runtime.dart' show init;\n".to_string()])
                .collect(),
        );

        for (id, ty) in self.tcx.all_types() {
            if ty.attrs().disable {
                continue;
            }

            let class_file = self.gen_ty(id);

            self.files
                .add_file(format!("src/{}", class_file.path), class_file.content);
        }
    }
}
