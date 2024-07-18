mod formatter;
mod header;
mod ty;

pub use self::formatter::CFormatter;
pub(crate) use self::formatter::CAPI_NAMESPACE;
pub(crate) use self::header::Header;
pub(crate) use self::ty::TyGenContext;

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

pub fn gen_runtime(is_for_cpp: bool) -> String {
    #[derive(Template)]
    #[template(path = "c2/runtime.h.jinja", escape = "none")]
    struct RuntimeTemplate {
        is_for_cpp: bool,
    }
    let mut runtime = String::new();
    RuntimeTemplate { is_for_cpp }
        .render_into(&mut runtime)
        .unwrap();
    runtime
}

pub fn run(tcx: &TypeContext) -> (FileMap, ErrorStore<String>) {
    let ctx = CContext {
        tcx,
        files: Default::default(),
        formatter: CFormatter::new(tcx, false),
        errors: ErrorStore::default(),
        is_for_cpp: false,
    };

    ctx.files
        .add_file("diplomat_runtime.h".into(), gen_runtime(false));

    for (id, ty) in ctx.tcx.all_types() {
        ctx.gen_ty(id, ty)
    }

    (ctx.files, ctx.errors)
}
