//! Naming: how HIR items become identifiers in the generated crate.

use std::borrow::Cow;
use std::fmt::Write as _;

use diplomat_core::hir::{self, DocsUrlGenerator, TypeContext, TypeDef};
use heck::ToSnakeCase;

pub(super) fn opaque_module_name(opaque: &hir::OpaqueDef) -> String {
    type_def_name(TypeDef::Opaque(opaque)).to_snake_case()
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
    method
        .attrs
        .rename
        .apply(Cow::Borrowed(declared))
        .into_owned()
}

pub(super) fn field_name<P: hir::TyPosition>(field: &hir::StructField<P>) -> String {
    field
        .attrs
        .rename
        .apply(Cow::Borrowed(field.name.as_str()))
        .into_owned()
}

pub(super) fn enum_variant_name(variant: &hir::EnumVariant) -> String {
    variant
        .attrs
        .rename
        .apply(Cow::Borrowed(variant.name.as_str()))
        .into_owned()
}

pub(super) fn emit_docs(
    out: &mut String,
    docs: &hir::Docs,
    docs_url_gen: &DocsUrlGenerator,
    indent: &str,
) {
    for line in docs
        .to_markdown(hir::DocsTypeReferenceSyntax::SquareBrackets, docs_url_gen)
        .lines()
    {
        writeln!(out, "{indent}/// {line}").unwrap();
    }
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
