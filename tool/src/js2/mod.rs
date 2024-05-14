use std::collections::HashMap;
use std::path::Path;

use diplomat_core::Env;

use diplomat_core::hir::{self, TypeContext};
use diplomat_core::ast::DocsUrlGenerator;

use crate::common::FileMap;

pub fn run(tcx : &TypeContext, conf_path : Option<&Path>) -> FileMap {
	let files = FileMap::default();

	files
}