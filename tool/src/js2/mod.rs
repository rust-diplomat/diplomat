use std::collections::HashMap;
use std::path::Path;

use diplomat_core::Env;

use diplomat_core::ast::DocsUrlGenerator;
use diplomat_core::hir::{self, TypeContext};

use crate::common::FileMap;

pub fn run(tcx: &TypeContext, conf_path: Option<&Path>) -> FileMap {
    let files = FileMap::default();

    for (id, ty) in tcx.all_types() {
        if ty.attrs().disable {
            continue;
        }

        let name = tcx.resolve_type(id).name().as_str();

        files.add_file(format!("{}.mjs", name), "This is a test".into());
        files.add_file(format!("{}.d.ts", name), "This is another test".into());
    }

    files
}
