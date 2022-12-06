//! This module contains functions for formatting types

use diplomat_core::hir::{TypeContext, TypeId};
use std::borrow::Cow;

/// This type mediates all formatting
///
/// All identifiers from the HIR should go through here before being formatted
/// into the output: This makes it easy to handle reserved words or add rename support
///
/// If you find yourself needing an identifier formatted in a context not yet available here, please add a new method
///
/// This type may be used by other backends attempting to figure out the names
/// of C types and methods.
pub struct Cpp2Formatter<'tcx> {
    tcx: &'tcx TypeContext,
}

impl<'tcx> Cpp2Formatter<'tcx> {
    pub fn new(tcx: &'tcx TypeContext) -> Self {
        Self { tcx }
    }
    /// Resolve and format a named type for use in code
    pub fn fmt_type_name(&self, id: TypeId) -> Cow<'tcx, str> {
        // Currently don't do anything fancy
        // Eventually apply rename rules and such
        self.tcx.resolve_type(id).name().as_str().into()
    }
}
