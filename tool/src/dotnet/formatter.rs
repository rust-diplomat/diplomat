//! Naming and type-formatting for the .NET backend.
//!
//! Mirrors the role of `kotlin/formatter.rs` and `cpp/formatter.rs`: every
//! place that needs to turn a HIR identifier into a C# fragment goes
//! through this struct, so that casing rules, `#[diplomat::rename(...)]`
//! attributes, and keyword escaping all live in one place.
//!
//! ## C# naming conventions applied here
//!
//! | Kind             | Convention | Example          |
//! |------------------|------------|------------------|
//! | Type / class     | PascalCase | `Color`, `Point2D` |
//! | Method           | PascalCase | `SetBrightness`  |
//! | Enum variant     | PascalCase | `Ok`, `Err`      |
//! | Public field     | PascalCase | `X`, `Y`         |
//! | Parameter        | camelCase  | `value`, `dx`    |
//!
//! Reserved-word collisions are escaped with `@`, the C# verbatim-identifier
//! prefix (e.g. `class` → `@class`). This is C#'s standard mechanism and
//! preserves the original name without mangling.

use std::borrow::Cow;
use std::collections::HashSet;
use std::sync::LazyLock;

use diplomat_core::hir::{DocsUrlGenerator, EnumVariant, Method, TypeContext, TypeId};
use heck::{ToLowerCamelCase, ToUpperCamelCase};

use crate::config::Config;

pub(super) struct DotnetFormatter<'tcx> {
    tcx: &'tcx TypeContext,
    #[allow(dead_code)] // wired up when we generate doc comments
    docs_url_gen: &'tcx DocsUrlGenerator,
}

/// C# reserved words that must be escaped with a leading `@` when used as
/// identifiers. Keep in sync with the C# language spec.
static KEYWORDS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [
        "abstract",
        "as",
        "base",
        "bool",
        "break",
        "byte",
        "case",
        "catch",
        "char",
        "checked",
        "class",
        "const",
        "continue",
        "decimal",
        "default",
        "delegate",
        "do",
        "double",
        "else",
        "enum",
        "event",
        "explicit",
        "extern",
        "false",
        "finally",
        "fixed",
        "float",
        "for",
        "foreach",
        "goto",
        "if",
        "implicit",
        "in",
        "int",
        "interface",
        "internal",
        "is",
        "lock",
        "long",
        "namespace",
        "new",
        "null",
        "object",
        "operator",
        "out",
        "override",
        "params",
        "private",
        "protected",
        "public",
        "readonly",
        "ref",
        "return",
        "sbyte",
        "sealed",
        "short",
        "sizeof",
        "stackalloc",
        "static",
        "string",
        "struct",
        "switch",
        "this",
        "throw",
        "true",
        "try",
        "typeof",
        "uint",
        "ulong",
        "unchecked",
        "unsafe",
        "ushort",
        "using",
        "virtual",
        "void",
        "volatile",
        "while",
    ]
    .into_iter()
    .collect()
});

impl<'tcx> DotnetFormatter<'tcx> {
    pub fn new(
        tcx: &'tcx TypeContext,
        _config: &Config, // reserved for future per-backend options (namespacing, prefix strip, ...)
        docs_url_gen: &'tcx DocsUrlGenerator,
    ) -> Self {
        Self { tcx, docs_url_gen }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Identifier formatters — one per identifier kind in C# output
    // ─────────────────────────────────────────────────────────────────────────

    /// Format a method's name as a PascalCase C# method identifier.
    /// Applies `#[diplomat::rename(...)]` if present and escapes keyword
    /// collisions with `@`.
    pub fn fmt_method_name<'a>(&self, method: &'a Method) -> Cow<'a, str> {
        let name = method.name.as_str().to_upper_camel_case();
        let name = method.attrs.rename.apply(name.into());
        escape_keyword(name)
    }

    /// Format an identifier as a camelCase C# parameter name (e.g. `value`,
    /// `dx`). Keyword collisions get `@`.
    pub fn fmt_param_name<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        let name = ident.to_lower_camel_case();
        escape_keyword(name.into())
    }

    /// Format an enum variant's name as a PascalCase C# identifier (matches
    /// .NET's `enum` casing conventions). Applies the variant's rename attr
    /// and escapes C# reserved-word collisions with `@`.
    pub fn fmt_enum_variant<'a>(&self, variant: &'a EnumVariant) -> Cow<'a, str> {
        let name = variant.name.as_str().to_upper_camel_case();
        let renamed = variant.attrs.rename.apply(name.into());
        escape_keyword(renamed)
    }

    /// Format a struct field's name as a PascalCase C# public-field
    /// identifier. C# convention for public fields is PascalCase
    /// (`public float X;`).
    pub fn fmt_field_name<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        let name = ident.to_upper_camel_case();
        escape_keyword(name.into())
    }

    /// Format a C# property name from the Rust method suffix after a
    /// configured getter/setter prefix (e.g. `width` from `get_width`).
    pub fn fmt_property_name<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        let name = ident.to_upper_camel_case();
        escape_keyword(name.into())
    }

    /// Format a HIR type's name as a C# class / struct / enum identifier.
    /// Applies `#[diplomat::rename]` if present and escapes C# keyword
    /// collisions. Used wherever a type appears in generated code by name
    /// (declaration sites, return types, params, `Raw.<Type>.<Method>`
    /// qualifications, and file names).
    ///
    /// Preserves source casing verbatim — `RDCleanPathPdu` stays
    /// `RDCleanPathPdu`, `UTCTime` stays `UTCTime`. heck's
    /// `to_upper_camel_case` would mangle these to `RdCleanPathPdu` /
    /// `UtcTime`, which would break references to the same types elsewhere
    /// (and silently divergence the surface from the previously-shipped
    /// AST-era bindings). Diplomat sources are already in idiomatic Rust
    /// PascalCase, so normalization is unnecessary and counterproductive.
    pub fn fmt_type_name(&self, id: TypeId) -> Cow<'tcx, str> {
        let resolved = self.tcx.resolve_type(id);
        let renamed = resolved
            .attrs()
            .rename
            .apply(resolved.name().as_str().into());
        escape_keyword(renamed)
    }
}

/// Prefix `@` if the identifier collides with a C# reserved word. Free
/// function (not a method) because it never needs `&self`.
fn escape_keyword(name: Cow<'_, str>) -> Cow<'_, str> {
    if KEYWORDS.contains(name.as_ref()) {
        format!("@{name}").into()
    } else {
        name
    }
}
