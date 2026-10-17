//! This module contains functions for formatting types

use diplomat_core::hir::{self, DocsTypeReferenceSyntax, DocsUrlGenerator, TypeContext, TypeId};
use heck::ToSnakeCase;
use std::borrow::Cow;

/// This type mediates all formatting
///
/// All identifiers from the HIR should go through here before being formatted
/// into the output: This makes it easy to handle reserved words or add rename support
pub(super) struct RustFormatter<'tcx> {
    tcx: &'tcx TypeContext,
    docs_url_gen: &'tcx DocsUrlGenerator,
}

impl<'tcx> RustFormatter<'tcx> {
    pub fn new(tcx: &'tcx TypeContext, docs_url_gen: &'tcx DocsUrlGenerator) -> Self {
        Self { tcx, docs_url_gen }
    }

    /// Resolve and format a named type for use in code
    pub fn fmt_type_name(&self, id: TypeId) -> Cow<'tcx, str> {
        let resolved = self.tcx.resolve_type(id);
        resolved
            .attrs()
            .rename
            .apply(resolved.name().as_str().into())
    }

    /// The name of the module (and file) a type is generated into
    pub fn fmt_module_name(&self, id: TypeId) -> String {
        self.fmt_type_name(id).to_snake_case()
    }

    pub fn fmt_docs(&self, docs: &hir::Docs) -> String {
        docs.to_markdown(DocsTypeReferenceSyntax::SquareBrackets, self.docs_url_gen)
            .trim()
            .to_string()
    }
}
