//! This module contains functions for formatting types

use crate::cpp::Cpp2Formatter;
use diplomat_core::hir::{
    Attrs, Docs, DocsTypeReferenceSyntax, DocsUrlGenerator, Method, StructPathLike, SymbolId,
    TraitIdGetter, TypeContext,
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

    pub fn fmt_binding_fn(&self, id: SymbolId) -> String {
        let name = match id {
            SymbolId::TypeId(t) => {
                let def = self.cxx.c.tcx().resolve_type(t);
                def.attrs().rename.apply(def.name().as_str().into())
            }
            SymbolId::TraitId(tr) => {
                let def = self.cxx.c.tcx().resolve_trait(tr);
                def.attrs.rename.apply(def.name.as_str().into())
            }
            _ => unreachable!("Unknown SymbolId: {id:?}"),
        };
        format!("add_{name}_binding")
    }

    pub fn fmt_binding_impl_path(&self, id: SymbolId) -> String {
        self.cxx.fmt_symbol_name(id).replace("::", "/") + "_binding.cpp"
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
            SymbolId::TraitId(tr) => self.cxx.c.tcx().resolve_trait(tr).attrs.namespace.as_ref(),
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

    pub fn symbol_to_python_type(&'tcx self, ty: crate::hir::SymbolId) -> Cow<'tcx, str> {
        let (name, namespace) = match ty {
            SymbolId::TypeId(ty) => {
                let resolved = self.cxx.c.tcx().resolve_type(ty);
                let name = resolved
                    .attrs()
                    .rename
                    .apply(resolved.name().as_str().into());
                (name, resolved.attrs().namespace.as_ref())
            }
            SymbolId::FunctionId(f) => {
                let resolved = self.cxx.c.tcx().resolve_function(f);
                let name = resolved.attrs.rename.apply(resolved.name.as_str().into());
                (name, resolved.attrs.namespace.as_ref())
            }
            SymbolId::TraitId(tr) => {
                let resolved = self.cxx.c.tcx().resolve_trait(tr);
                let name = resolved.attrs.rename.apply(resolved.name.as_str().into());
                (name, resolved.attrs.namespace.as_ref())
            }
            _ => unreachable!("Unrecognized symbol ID: {ty:?}"),
        };
        if let Some(ns) = namespace {
            format!("{}.{name}", ns.replace("::", ".")).into()
        } else {
            name
        }
    }

    pub fn primitive_to_python_type(&self, p: &crate::hir::PrimitiveType) -> Cow<'static, str> {
        match p {
            crate::hir::PrimitiveType::Bool => "bool".into(),
            crate::hir::PrimitiveType::Char => "str".into(),
            crate::hir::PrimitiveType::Byte
            | crate::hir::PrimitiveType::Ordering
            | crate::hir::PrimitiveType::Int(..)
            | crate::hir::PrimitiveType::IntSize(..)
            | crate::hir::PrimitiveType::Int128(..) => "int".into(),
            crate::hir::PrimitiveType::Float(..) => "float".into(),
        }
    }

    /// Used by trait abstract definitions.
    pub fn hir_type_to_python_type<P: crate::hir::TyPosition>(
        &'tcx self,
        ty: &crate::hir::Type<P>,
    ) -> Cow<'tcx, str> {
        match ty {
            crate::hir::Type::Primitive(p) => self.primitive_to_python_type(p),
            crate::hir::Type::Opaque(op) => self.symbol_to_python_type(op.id().into()),
            crate::hir::Type::Struct(st) => self.symbol_to_python_type(st.id().into()),
            crate::hir::Type::ImplTrait(tr) => self.symbol_to_python_type(tr.id().into()),
            crate::hir::Type::Enum(e) => self.symbol_to_python_type(e.id().into()),
            crate::hir::Type::Slice(sl) => match sl {
                crate::hir::Slice::Str(..) => "str".into(),
                crate::hir::Slice::Primitive(_, p) => {
                    format!("list[{}]", self.primitive_to_python_type(p)).into()
                }
                crate::hir::Slice::Strs(..) => "list[str]".into(),
                crate::hir::Slice::Opaque(_, op) => {
                    format!("list[{}]", self.symbol_to_python_type(op.id().into())).into()
                }
                crate::hir::Slice::Struct(_, st) => {
                    format!("list[{}]", self.symbol_to_python_type(st.id().into())).into()
                }
                crate::hir::Slice::Enum(_, e) => {
                    format!("list[{}]", self.symbol_to_python_type(e.id().into())).into()
                }
                _ => unreachable!("Unknown AST/HIR variant: {sl:?}"),
            },
            crate::hir::Type::Callback(cb) => {
                let ret =
                    diplomat_core::hir::CallbackInstantiationFunctionality::get_output_type(cb)
                        .expect("Could not get output type.");

                let params = diplomat_core::hir::CallbackInstantiationFunctionality::get_inputs(cb)
                    .expect("Could not get callback inputs.")
                    .iter()
                    .map(|p| self.hir_type_to_python_type(&p.ty))
                    .collect::<Vec<_>>();

                format!(
                    "Callable[[{}], {}]",
                    params.join(","),
                    self.return_type_to_python_type(ret)
                )
                .into()
            }
            crate::hir::Type::DiplomatOption(op) => {
                format!("{} | None", self.hir_type_to_python_type(op.as_ref())).into()
            }
            _ => unreachable!("Unrecognized AST type: {ty:?}"),
        }
    }

    fn success_type_to_python_type<P: crate::hir::TyPosition>(
        &'tcx self,
        ty: &crate::hir::SuccessType<P>,
    ) -> Cow<'tcx, str> {
        match ty {
            crate::hir::SuccessType::Unit => "None".into(),
            crate::hir::SuccessType::Write(crate::hir::WriteType::Str) => "str".into(),
            crate::hir::SuccessType::Write(crate::hir::WriteType::Primitive(p)) => {
                format!("list[{}]", self.primitive_to_python_type(p)).into()
            }
            crate::hir::SuccessType::OutType(o) => self.hir_type_to_python_type(o),
            _ => unreachable!("Unrecognized success type: {ty:?}"),
        }
    }

    pub fn return_type_to_python_type<P: crate::hir::TyPosition>(
        &'tcx self,
        ty: &crate::hir::ReturnType<P>,
    ) -> Cow<'tcx, str> {
        match ty {
            crate::hir::ReturnType::Infallible(i) => self.success_type_to_python_type(i),
            crate::hir::ReturnType::Nullable(n) => {
                format!("{} | None", self.success_type_to_python_type(n)).into()
            }
            crate::hir::ReturnType::Fallible(ok, _) => self.success_type_to_python_type(ok),
        }
    }
}
