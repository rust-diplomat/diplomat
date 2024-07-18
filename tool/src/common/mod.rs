//! This module contains common abstractions that multiple backends may find
//! useful.
//!
//! C-specific abstractions that are also useful for other backends
//! (since all backends eventually go through the C API), like CFormatter,
//! should live in the c2 module, not here.

use core::mem;
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;

/// This type abstracts over files being written to.
#[derive(Default, Debug)]
pub struct FileMap {
    // The context types exist as a way to avoid passing around a billion different
    // parameters. However, passing them around as &mut self restricts the amount of
    // borrowing that can be done. We instead use a RefCell to guard the specifically mutable bits.
    files: RefCell<HashMap<String, String>>,
}

impl FileMap {
    #[allow(dead_code)]
    pub fn new(files: HashMap<String, String>) -> Self {
        FileMap {
            files: RefCell::new(files),
        }
    }

    pub fn take_files(self) -> HashMap<String, String> {
        mem::take(&mut *self.files.borrow_mut())
    }

    pub fn add_file(&self, name: String, contents: String) {
        if self.files.borrow().get(&name).is_some() {
            panic!("File map already contains {}", name)
        }
        self.files.borrow_mut().insert(name, contents);
    }
}

/// This type acts as a "store" for errors, which can be appended to.
/// Keeps track of the context in which an error was generated.
///
/// You can use [`set_context_ty()`] and [`set_context_method()`] to set the context
/// as a type or method. They will return scope guards that will automatically pop the stack
/// once they go out of scope, so you don't have to worry about errors originating from code
/// that does not set a context.
#[derive(Default)]
pub struct ErrorStore<'tcx, E> {
    /// The stack of contexts reached so far
    context: RefCell<ErrorContext<'tcx>>,
    errors: RefCell<Vec<(ErrorContext<'tcx>, E)>>,
}

impl<'tcx, E> ErrorStore<'tcx, E> {
    /// Set the context to a named type. Will return a scope guard that will automatically
    /// clear the context on drop.
    pub fn set_context_ty<'a>(&'a self, ty: Cow<'tcx, str>) -> ErrorContextGuard<'a, 'tcx, E> {
        let new = ErrorContext { ty, method: None };
        let old = mem::replace(&mut *self.context.borrow_mut(), new);
        ErrorContextGuard(self, old)
    }
    /// Set the context to a named method. Will return a scope guard that will automatically
    /// clear the context on drop.
    pub fn set_context_method<'a>(
        &'a self,
        ty: Cow<'tcx, str>,
        method: Cow<'tcx, str>,
    ) -> ErrorContextGuard<'a, 'tcx, E> {
        let new = ErrorContext {
            ty,
            method: Some(method),
        };

        let old = mem::replace(&mut *self.context.borrow_mut(), new);
        ErrorContextGuard(self, old)
    }

    pub fn push_error(&self, error: E) {
        self.errors
            .borrow_mut()
            .push((self.context.borrow().clone(), error));
    }

    pub fn take_all(&self) -> Vec<(impl fmt::Display + 'tcx, E)> {
        mem::take(&mut self.errors.borrow_mut())
    }
}

/// The context in which an error was discovered
#[derive(Default, Clone)]
struct ErrorContext<'tcx> {
    ty: Cow<'tcx, str>,
    method: Option<Cow<'tcx, str>>,
}

impl<'tcx> fmt::Display for ErrorContext<'tcx> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ty = &self.ty;
        if let Some(ref method) = self.method {
            write!(f, "{ty}::{method}")
        } else {
            ty.fmt(f)
        }
    }
}

/// Scope guard terminating the context created `set_context_*` method on [`ErrorStore`]
#[must_use]
pub struct ErrorContextGuard<'a, 'tcx, E>(&'a ErrorStore<'tcx, E>, ErrorContext<'tcx>);

impl<'a, 'tcx, E> Drop for ErrorContextGuard<'a, 'tcx, E> {
    fn drop(&mut self) {
        let _ = mem::replace(&mut *self.0.context.borrow_mut(), mem::take(&mut self.1));
    }
}
