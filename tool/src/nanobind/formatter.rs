//! This module contains functions for formatting types

use crate::cpp::Cpp2Formatter;
use diplomat_core::hir::{
    Attrs, Docs, DocsTypeReferenceSyntax, DocsUrlGenerator, Method, SymbolId, TypeContext, TypeId,
};
use std::fmt::Write;
use std::{borrow::Cow, sync::LazyLock};

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
    docs_url_gen: &'tcx DocsUrlGenerator,
}

impl<'tcx> PyFormatter<'tcx> {
    pub fn new(
        tcx: &'tcx TypeContext,
        config_for_cpp: &crate::Config,
        docs_url_gen: &'tcx DocsUrlGenerator,
    ) -> Self {
        Self {
            cxx: Cpp2Formatter::new(tcx, config_for_cpp, docs_url_gen),
            docs_url_gen,
        }
    }

    /// Renders doc comments as plain text suitable for a Python docstring.
    pub fn fmt_docs(&self, docs: &Docs, attrs: &Attrs) -> String {
        let mut docs = docs
            .to_markdown(DocsTypeReferenceSyntax::SquareBrackets, self.docs_url_gen)
            .trim()
            .to_string();
        if let Some(deprecated) = attrs.deprecated.as_ref() {
            if !docs.is_empty() {
                docs.push('\n');
                docs.push('\n');
            }
            let _ = writeln!(&mut docs, ".. deprecated:: {deprecated}");
        }
        docs
    }

    /// Renders doc comments (plus an optional deprecation notice) as a quoted, escaped C++
    /// string literal suitable for use as a nanobind docstring argument (e.g. as the trailing
    /// argument to `.def(...)`, `nb::class_<T>(...)`, or `.def_prop_ro(...)`).
    /// Returns `None` if there's no doc text to show, so call sites can omit the argument.
    pub fn fmt_doc_literal(&self, docs: &Docs, attrs: &Attrs) -> Option<String> {
        let docs = self.fmt_docs(docs, attrs);
        if docs.is_empty() {
            return None;
        }
        let mut literal = String::with_capacity(docs.len() + 2);
        literal.push('"');
        for c in docs.chars() {
            match c {
                '\\' => literal.push_str("\\\\"),
                '"' => literal.push_str("\\\""),
                '\n' => literal.push_str("\\n"),
                '\r' => {}
                _ => literal.push(c),
            }
        }
        literal.push('"');
        Some(literal)
    }

    pub fn fmt_binding_fn(&self, id: TypeId) -> String {
        let def = self.cxx.c.tcx().resolve_type(id);
        let type_name = def.attrs().rename.apply(def.name().as_str().into());
        format!("add_{type_name}_binding")
    }

    pub fn fmt_binding_impl_path(&self, id: TypeId) -> String {
        self.cxx.fmt_type_name(id).replace("::", "/") + "_binding.cpp"
    }

    /// Resolve and format the nested module names for this type
    /// Returns an iterator to the namespaces. Will always have at least one entry
    pub fn fmt_namespaces(&self, id: SymbolId) -> impl Iterator<Item = &'tcx str> {
        let namespace = match id {
            SymbolId::FunctionId(f) => self
                .cxx
                .c
                .tcx()
                .resolve_function(f)
                .attrs
                .namespace
                .as_ref(),
            SymbolId::TypeId(ty) => self.cxx.c.tcx().resolve_type(ty).attrs().namespace.as_ref(),
            _ => panic!("Unsupported SymbolId {id:?}"),
        };
        namespace
            .as_ref()
            .map(|v| v.split("::"))
            .into_iter()
            .flatten()
    }

    pub fn fmt_method_name<'a>(&'tcx self, method: &'a Method) -> Cow<'a, str> {
        self.fmt_identifier(method.attrs.rename.apply(method.name.as_str().into()))
    }

    pub fn fmt_identifier<'a>(&'tcx self, name: Cow<'a, str>) -> Cow<'a, str> {
        // Source https://docs.python.org/3/reference/lexical_analysis.html#keywords
        #[rustfmt::skip]
        static PY_KEYWORDS: LazyLock<std::collections::HashSet<&str>> = LazyLock::new(|| {
            [
                "False", "await", "else", "import", "pass",
                "None", "break", "except", "in", "raise",
                "True", "class", "finally", "is", "return",
                "and", "continue", "for", "lambda", "try",
                "as", "def", "from", "nonlocal", "while",
                "assert", "del", "global", "not", "with",
                "async", "elif", "if", "or", "yield",
            ]
            .into()
        });

        if PY_KEYWORDS.contains(name.as_ref()) {
            format!("{name}_").into()
        } else {
            name
        }
    }
}
