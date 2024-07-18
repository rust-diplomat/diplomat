mod formatter;
mod header;
mod ty;

use crate::common::{ErrorStore, FileMap};
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

pub fn run(tcx: &TypeContext) -> (FileMap, ErrorStore<String>) {
    let ctx = Cpp2Context {
        tcx,
        files: Default::default(),
        formatter: Cpp2Formatter::new(tcx),
        errors: ErrorStore::default(),
    };

    ctx.files.add_file(
        "diplomat_c_runtime.hpp".into(),
        crate::c2::gen_runtime(true),
    );

    ctx.files
        .add_file("diplomat_runtime.hpp".into(), RUNTIME_HPP.into());

    for (id, ty) in ctx.tcx.all_types() {
        ctx.gen_ty(id, ty)
    }

    (ctx.files, ctx.errors)
}
