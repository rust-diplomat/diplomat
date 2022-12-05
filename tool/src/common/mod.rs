//! This module contains common abstractions that multiple backends may find
//! useful.
//!
//! C-specific abstractions that are also useful for other backends
//! (since all backends eventually go through the C API), like CFormatter,
//! should live in the c2 module, not here.

use core::mem;
use std::cell::RefCell;
use std::collections::HashMap;

/// This type abstracts over files being written to.
#[derive(Default)]
pub struct FileMap {
    // The context types exist as a way to avoid passing around a billion different
    // parameters. However, passing them around as &mut self restricts the amount of
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
