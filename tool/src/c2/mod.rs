mod formatter;
mod header;
mod ty;

use core::mem;
use diplomat_core::hir::{self, TypeContext};
use diplomat_core::Env;
use std::cell::RefCell;
use std::collections::HashMap;

/// This is the main object that drives this backend. Most execution steps
/// for this backend will be found as methods on this context
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

    /// Run file generation
    ///
    /// Will populate self.files as a result
    pub fn run(&self) {
        self.files
            .add_file("diplomat_runtime.h".into(), crate::c::RUNTIME_H.into());
        for (id, ty) in self.tcx.all_types() {
            ty::gen_ty(self, id, ty)
        }
    }

    // further methods can be found in ty.rs and formatter.rs
}

/// This type abstracts over files being written to.
// todo: this should eventually be a common type shared by backends
#[derive(Default)]
pub struct FileMap {
    // CContext exists as a way to avoid passing around a billion different
    // parameters. However, passing it around as &mut self restricts the amount of
    // borrowing that can be done. We instead use a RefCell to guard the specifically mutable bits.
    files: RefCell<HashMap<String, String>>,
}

impl FileMap {
    pub fn new(files: HashMap<String, String>) -> Self {
        FileMap {
            files: RefCell::new(files),
        }
    }

    pub fn take_files(&mut self) -> HashMap<String, String> {
        mem::take(&mut *self.files.borrow_mut())
    }

    pub fn add_file(&self, name: String, contents: String) {
        if self.files.borrow().get(&name).is_some() {
            panic!("File map already contains {}", name)
        }
        self.files.borrow_mut().insert(name, contents);
    }
}
