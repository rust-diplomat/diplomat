//! This module contains functions for formatting types

use diplomat_core::hir::{self, OpaqueOwner, Type, TypeContext, TypeId};
use std::borrow::Cow;
use crate::c2::CFormatter;

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
            c: CFormatter::new(tcx)
        }
    }

    /// Resolve and format a named type for use in code
    pub fn fmt_type_name(&self, id: TypeId) -> Cow<'tcx, str> {
        // Currently don't do anything fancy
        // Eventually apply rename rules and such
        self.c.tcx().resolve_type(id).name().as_str().into()
    }
    /// Resolve and format the name of a type for use in header names
    pub fn fmt_header_name(&self, id: TypeId) -> Cow<'tcx, str> {
        self.fmt_type_name(id)
    }
    /// Format an enum variant.
    pub fn fmt_enum_variant(&self, variant: &'tcx hir::EnumVariant) -> Cow<'tcx, str> {
        variant.name.as_str().into()
    }
    /// Format a field name or parameter name
    // might need splitting in the future if we decide to support renames here
    pub fn fmt_param_name<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        ident.into()
    }

    pub fn fmt_c_name<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        format!("capi::{ident}").into()
    }

    pub fn fmt_optional<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        format!("std::optional<{}>", ident).into()
    }

    pub fn fmt_borrowed<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        format!("{}&", ident).into()
    }

    pub fn fmt_optional_borrowed<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        format!("std::optional<{}&>", ident).into()
    }

    pub fn fmt_owned<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        format!("std::unique_ptr<{}>", ident).into()
    }

    pub fn fmt_borrowed_slice<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        format!("std::span<{}>", ident).into()
    }

    pub fn fmt_borrowed_str(&self) -> Cow<'static, str> {
        "std::string_view".into()
    }

    pub fn fmt_owned_str(&self) -> Cow<'static, str> {
        "std::string".into()
    }

    /// Format a method
    pub fn fmt_method_name<'a>(&self, ty: TypeId, method: &'a hir::Method) -> Cow<'a, str> {
        if method.name == "new" {
            "new_".into()
        } else {
            method.name.as_str().into()
        }
    }

    /// Given a mutability, format a `const ` prefix for pointers if necessary,
    /// including a space for prepending
    pub fn fmt_constness<'a>(&self, ident: &'a str, mutability: hir::Mutability) -> Cow<'a, str> {
        mutability.if_mut_else(ident.into(), format!("const {}", ident).into())
    }

    /// Get the primitive type as a C type
    pub fn fmt_primitive_as_c(&self, prim: hir::PrimitiveType) -> Cow<'static, str> {
        self.c.fmt_primitive_as_c(prim)
    }
}
