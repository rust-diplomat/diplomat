//! This module contains functions for formatting types
//!
//! All identifiers from the HIR should go through here before being formatted
//! into the output: This makes it easy to handle reserved words or add rename support
//!
//! If you find yourself needing an identifier formatted in a context not yet available here, please add a new method

use diplomat_core::hir::{self, TypeId};
use std::borrow::Cow;

// todo: eventually we shall need to encapsulate a bunch of this in a separate
// type so that other backends can fetch the C type as well
impl super::CContext {
    /// Resolve and format the name of a type for use in code
    pub fn fmt_type_name<'tcx>(&'tcx self, id: TypeId) -> Cow<'tcx, str> {
        // Currently don't do anything fancy
        // Eventually apply rename rules and such
        self.tcx.resolve_type(id).name().as_str().into()
    }
    /// Resolve and format the name of a type for use in header names
    pub fn fmt_header_name<'tcx>(&'tcx self, id: TypeId) -> Cow<'tcx, str> {
        self.fmt_type_name(id)
    }
    /// Format an enum variant.
    pub fn fmt_enum_variant<'tcx>(&'tcx self, variant: &'tcx hir::EnumVariant) -> Cow<'tcx, str> {
        variant.name.as_str().into()
    }
    /// Format a field name or parameter name
    // might need splitting in the future if we decide to support renames here
    pub fn fmt_param_name<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        ident.into()
    }

    /// Format a method
    pub fn fmt_method_name(&self, ty: TypeId, method: &hir::Method) -> String {
        let ty_name = self.fmt_type_name(ty);
        let method_name = method.name.as_str();
        format!("{ty_name}_{method_name}")
    }

    /// Given a mutability, format a `const ` prefix for pointers if necessary,
    /// including a space for prepending
    pub fn fmt_constness(&self, mutability: hir::Mutability) -> &str {
        if mutability.is_mutable() {
            ""
        } else {
            "const "
        }
    }
}
