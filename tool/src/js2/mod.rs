use std::collections::HashMap;
use std::path::Path;

use diplomat_core::Env;

use diplomat_core::ast::DocsUrlGenerator;
use diplomat_core::hir::{self, TypeContext};

use crate::common::FileMap;

pub fn run(tcx: &TypeContext, conf_path: Option<&Path>) -> FileMap {
    let files = FileMap::default();

    files.add_file("diplomat-runtime.mjs".into(), include_str!("runtime.mjs").into());
    files.add_file("diplomat-runtime.d.ts".into(), include_str!("runtime.d.ts").into());
    files.add_file("diplomat-wasm.mjs".into(), include_str!("wasm.mjs").into());

    
    // TODO: All of this.
    files.add_file("index.mjs".into(), "export { FFIError } from './diplomat-runtime.mjs';".into());
    files.add_file("index.d.ts".into(), "".into());

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
