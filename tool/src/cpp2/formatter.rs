//! This module contains functions for formatting types

use crate::c2::CFormatter;
use diplomat_core::hir::{self, TypeContext, TypeId};
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
    c: CFormatter<'tcx>,
}

impl<'tcx> Cpp2Formatter<'tcx> {
    pub fn new(tcx: &'tcx TypeContext) -> Self {
        Self {
            c: CFormatter::new(tcx),
        }
    }

    /// Resolve and format a named type for use in code
    pub fn fmt_type_name(&self, id: TypeId) -> Cow<'tcx, str> {
        let resolved = self.c.tcx().resolve_type(id);
        if let Some(rename) = resolved.attrs().rename.as_ref() {
            rename.into()
        } else {
            resolved.name().as_str().into()
        }
    }

    /// Resolve and format a named type for use in diagnostics
    /// (don't apply rename rules and such)
    pub fn fmt_type_name_diagnostics(&self, id: TypeId) -> Cow<'tcx, str> {
        self.c.fmt_type_name_diagnostics(id)
    }

    /// Resolve and format the name of a type for use in header names
    pub fn fmt_decl_header_path(&self, id: TypeId) -> String {
        let type_name = self.fmt_type_name(id);
        format!("{type_name}.d.hpp")
    }
    /// Resolve and format the name of a type for use in header names
    pub fn fmt_impl_header_path(&self, id: TypeId) -> String {
        let type_name = self.fmt_type_name(id);
        format!("{type_name}.hpp")
    }

    pub fn fmt_c_decl_header_path(&self, id: TypeId) -> String {
        self.c.fmt_decl_header_path(id)
    }
    pub fn fmt_c_impl_header_path(&self, id: TypeId) -> String {
        self.c.fmt_impl_header_path(id)
    }

    /// Format an enum variant.
    pub fn fmt_enum_variant(&self, variant: &'tcx hir::EnumVariant) -> Cow<'tcx, str> {
        if let Some(rename) = variant.attrs.rename.as_ref() {
            rename.into()
        } else {
            variant.name.as_str().into()
        }
    }
    pub fn fmt_c_enum_variant<'a>(
        &self,
        ident: &'a str,
        variant: &'tcx hir::EnumVariant,
    ) -> Cow<'tcx, str> {
        let c_variant_name = self.c.fmt_enum_variant(ident, variant);
        format!("capi::{c_variant_name}").into()
    }
    /// Format a field name or parameter name
    // might need splitting in the future if we decide to support renames here
    pub fn fmt_param_name<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        ident.into()
    }

    pub fn fmt_c_name<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        format!("capi::{ident}").into()
    }

    pub fn fmt_c_ptr<'a>(&self, ident: &'a str, mutability: hir::Mutability) -> Cow<'a, str> {
        self.c.fmt_ptr(ident, mutability)
    }

    #[allow(dead_code)]
    pub fn fmt_optional<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        format!("std::optional<{ident}>").into()
    }

    pub fn fmt_borrowed<'a>(&self, ident: &'a str, mutability: hir::Mutability) -> Cow<'a, str> {
        // TODO: Where is the right place to put `const` here?
        if mutability.is_mutable() {
            format!("{ident}&").into()
        } else {
            format!("const {ident}&").into()
        }
    }

    pub fn fmt_move_ref<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        format!("{ident}&&").into()
    }

    pub fn fmt_optional_borrowed<'a>(
        &self,
        ident: &'a str,
        mutability: hir::Mutability,
    ) -> Cow<'a, str> {
        self.c.fmt_ptr(ident, mutability)
    }

    pub fn fmt_owned<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        format!("std::unique_ptr<{ident}>").into()
    }

    pub fn fmt_borrowed_slice<'a>(
        &self,
        ident: &'a str,
        mutability: hir::Mutability,
    ) -> Cow<'a, str> {
        // TODO: This needs to change if an abstraction other than std::span is used
        // TODO: Where is the right place to put `const` here?
        if mutability.is_mutable() {
            format!("diplomat::span<{ident}>").into()
        } else {
            format!("diplomat::span<const {ident}>").into()
        }
    }

    pub fn fmt_borrowed_utf8_str(&self) -> Cow<'static, str> {
        // TODO: This needs to change if an abstraction other than std::u8string_view is used
        "std::string_view".into()
    }

    pub fn fmt_borrowed_utf16_str(&self) -> Cow<'static, str> {
        // TODO: This needs to change if an abstraction other than std::u16string_view is used
        "std::u16string_view".into()
    }

    pub fn fmt_owned_str(&self) -> Cow<'static, str> {
        "std::string".into()
    }

    /// Format a method
    pub fn fmt_method_name<'a>(&self, method: &'a hir::Method) -> Cow<'a, str> {
        // TODO(#60): handle other keywords
        if let Some(rename) = method.attrs.rename.as_ref() {
            rename.into()
        } else if method.name == "new" {
            "new_".into()
        } else if method.name == "default" {
            "default_".into()
        } else {
            method.name.as_str().into()
        }
    }

    pub fn fmt_c_method_name<'a>(&self, ty: TypeId, method: &'a hir::Method) -> Cow<'a, str> {
        format!("capi::{}", self.c.fmt_method_name(ty, method)).into()
    }

    /// Get the primitive type as a C type
    pub fn fmt_primitive_as_c(&self, prim: hir::PrimitiveType) -> Cow<'static, str> {
        self.c.fmt_primitive_as_c(prim)
    }
}
