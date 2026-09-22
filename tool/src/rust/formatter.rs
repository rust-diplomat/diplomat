//! Naming: how HIR items become identifiers in the generated crate.

use diplomat_core::hir::{self, DocsUrlGenerator, TypeContext, TypeDef};
use heck::ToSnakeCase;
use std::borrow::Cow;

pub(super) fn opaque_module_name(opaque: &hir::OpaqueDef) -> String {
    type_module_name(TypeDef::Opaque(opaque))
}

pub(super) fn type_module_name(def: TypeDef<'_>) -> String {
    type_def_name(def).to_snake_case()
}

pub(super) fn type_def_name(def: TypeDef<'_>) -> String {
    def.attrs()
        .rename
        .apply(Cow::Borrowed(def.name().as_str()))
        .into_owned()
}

pub(super) fn opaque_name(id: hir::OpaqueId, tcx: &TypeContext) -> String {
    type_def_name(TypeDef::Opaque(tcx.resolve_opaque(id)))
}

pub(super) fn enum_name(id: hir::EnumId, tcx: &TypeContext) -> String {
    type_def_name(TypeDef::Enum(tcx.resolve_enum(id)))
}

/// The name a method is emitted under.
///
/// An explicit `#[diplomat::attr(named_constructor = "name")]` replaces the Rust
/// method name; a bare `named_constructor` leaves it alone. `rename` is applied on
/// top of whichever name was declared, matching the other backends.
pub(super) fn method_name(method: &hir::Method) -> String {
    let declared = match &method.attrs.special_method {
        Some(hir::SpecialMethod::NamedConstructor(Some(name))) => name.as_str(),
        _ => method.name.as_str(),
    };
    let renamed = method
        .attrs
        .rename
        .apply(Cow::Borrowed(declared))
        .into_owned();
    // A `named_constructor` name is the provider's request for the *concept*, not
    // a literal identifier: the corpus asks for `"f64BeBytes"`, and dotnet emits
    // `NewF64BeBytes`. Rust's equivalent is the snake_case form, so the generated
    // method is `f64_be_bytes` rather than an un-idiomatic `f64BeBytes`.
    escape_keyword(&renamed.to_snake_case())
}

pub(super) fn field_name<P: hir::TyPosition>(field: &hir::StructField<P>) -> String {
    let renamed = field
        .attrs
        .rename
        .apply(Cow::Borrowed(field.name.as_str()))
        .into_owned();
    escape_keyword(&renamed)
}

pub(super) fn enum_variant_name(variant: &hir::EnumVariant) -> String {
    let renamed = variant
        .attrs
        .rename
        .apply(Cow::Borrowed(variant.name.as_str()))
        .into_owned();
    escape_keyword(&renamed)
}

/// Rust keywords that cannot be written as a raw identifier. `r#self`,
/// `r#Self`, `r#super`, `r#crate` and `r#_` are all rejected by rustc, so those
/// take a trailing underscore instead.
const NON_RAW_KEYWORDS: &[&str] = &["self", "Self", "super", "crate", "_"];

/// Strict and reserved Rust keywords, plus `_`.
///
/// A provider may legitimately declare a method or field whose name is a
/// keyword — `feature_tests/src/slices.rs` names a constructor `unsafe` and
/// `lifetimes.rs` names one `static`. Those reach this backend as ordinary
/// identifiers, so the emitted name has to be escaped rather than the whole
/// provider rejected: the name is the provider's to choose, the escaping is
/// this backend's job.
const KEYWORDS: &[&str] = &[
    // Strict keywords.
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", // Reserved for future use.
    "abstract", "become", "box", "do", "final", "macro", "override", "priv", "typeof", "unsized",
    "virtual", "yield", "try", "_",
];

/// Escape a name that collides with a Rust keyword.
///
/// Returns `r#name` for keywords that permit a raw identifier, and `name_` for
/// the handful that do not. A name that is not a keyword is returned unchanged,
/// so a genuinely malformed identifier is still rejected by [`valid_rust_ident`]
/// rather than silently rewritten.
pub(super) fn escape_keyword(name: &str) -> String {
    if !KEYWORDS.contains(&name) {
        return name.to_string();
    }
    if NON_RAW_KEYWORDS.contains(&name) {
        format!("{name}_")
    } else {
        format!("r#{name}")
    }
}

pub(super) fn render_docs(
    docs: &hir::Docs,
    docs_url_gen: &DocsUrlGenerator,
    indent: &str,
) -> String {
    docs.to_markdown(hir::DocsTypeReferenceSyntax::SquareBrackets, docs_url_gen)
        .lines()
        .map(|line| format!("{indent}/// {line}\n"))
        .collect()
}

pub(super) fn valid_rust_ident(name: &str) -> bool {
    syn::parse_str::<syn::Ident>(name).is_ok()
}

pub(super) fn sanitize_package_component(name: &str) -> String {
    let sanitized: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || matches!(c, '-' | '_') {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect();
    sanitized.trim_matches('-').to_string()
}

pub(super) fn valid_package_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_'))
}
