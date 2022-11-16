mod formatter;
mod header;

use core::mem;
use diplomat_core::hir::{self, TypeContext};
use diplomat_core::Env;
use std::collections::HashMap;

pub struct CContext {
    pub tcx: TypeContext,
    pub files: FileMap,
}

impl CContext {
    pub fn new(env: &Env, files: FileMap) -> Result<Self, Vec<hir::LoweringError>> {
        Ok(CContext {
            tcx: TypeContext::from_ast(env)?,
            files,
        })
    }

    pub fn run(&mut self) -> Result<(), ()> {
        Ok(())
    }
}

// todo: this should eventually be a common type shared by backends
#[derive(Default)]
pub struct FileMap {
    files: HashMap<String, String>,
}

impl FileMap {
    pub fn new(files: HashMap<String, String>) -> Self {
        FileMap { files }
    }

    pub fn take_files(&mut self) -> HashMap<String, String> {
        mem::take(&mut self.files)
    }

    pub fn add_file(&mut self, name: String, contents: String) {
        if self.files.get(&name).is_some() {
            panic!("File map already contains {}", name)
        }
        self.files.insert(name, contents);
    }
}
