//! Experimental Safe Rust backend: generates a Rust crate that wraps the C ABI
//! of a Diplomat library in a safe, idiomatic API.

mod formatter;
mod gen;

use self::formatter::RustFormatter;
use self::gen::ItemGenContext;
use crate::{ErrorStore, FileMap};
use askama::Template;
use diplomat_core::hir::{self, BackendAttrSupport, DocsUrlGenerator};

pub(crate) fn attr_support() -> BackendAttrSupport {
    // Nothing is supported yet; capabilities are turned on as they are implemented.
    BackendAttrSupport::default()
}

#[derive(Template)]
#[template(path = "rust/lib.rs.jinja", escape = "none")]
struct LibTemplate {
    modules: Vec<(String, String)>,
}

pub(crate) fn run<'tcx>(
    tcx: &'tcx hir::TypeContext,
    docs_url_gen: &'tcx DocsUrlGenerator,
) -> (FileMap, ErrorStore<'tcx, String>) {
    let files = FileMap::default();
    let formatter = RustFormatter::new(tcx, docs_url_gen);
    let errors = ErrorStore::default();

    let mut modules = vec![];

    for (id, ty) in tcx.all_types() {
        if ty.attrs().disable {
            continue;
        }

        let _guard = errors.set_context_ty(ty.name_with_span().into());
        let context = ItemGenContext {
            tcx,
            formatter: &formatter,
            errors: &errors,
        };

        let body = match id {
            hir::TypeId::Enum(e) => context.gen_enum(e),
            hir::TypeId::Opaque(o) => context.gen_opaque(o),
            hir::TypeId::Struct(s) => context.gen_struct::<hir::Everywhere>(s),
            hir::TypeId::OutStruct(s) => context.gen_struct::<hir::OutputOnly>(s),
            _ => unreachable!("unknown AST/HIR variant"),
        };

        let module = formatter.fmt_module_name(id);
        files.add_file(format!("{module}.rs"), body);
        modules.push((module, formatter.fmt_type_name(id).into_owned()));
    }

    modules.sort();
    files.add_file("lib.rs".into(), LibTemplate { modules }.to_string());

    (files, errors)
}
