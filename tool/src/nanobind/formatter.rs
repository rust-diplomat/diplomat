//! This module contains functions for formatting types

use crate::cpp::Cpp2Formatter;
use diplomat_core::hir::{DocsUrlGenerator, TypeContext, TypeId};
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
pub(crate) struct PyFormatter<'tcx> {
    pub cxx: Cpp2Formatter<'tcx>,
}

impl<'tcx> PyFormatter<'tcx> {
    pub fn new(tcx: &'tcx TypeContext, docs_url_gen: &'tcx DocsUrlGenerator) -> Self {
        Self {
            cxx: Cpp2Formatter::new(tcx, docs_url_gen),
        }
    }

    /// Resolve and format the nested module names for this type
    /// Returns an iterator to the namespaces. Will always have at least one entry
    pub fn fmt_namespaces(&self, id: TypeId) -> impl Iterator<Item = &'tcx str> {
        let resolved = self.cxx.c.tcx().resolve_type(id);
        resolved
            .attrs()
            .namespace
            .as_ref()
            .map(|v| v.split("::"))
            .into_iter()
            .flatten()
    }

    /// Resolve the name of the module to use
    pub fn fmt_module(&'tcx self, id: TypeId, default: &'tcx str) -> Cow<'tcx, str> {
        self.fmt_namespaces(id).last().unwrap_or(default).into()
    }
}
