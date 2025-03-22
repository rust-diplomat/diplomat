//! This module contains functions for formatting types

use crate::cpp::Cpp2Formatter;
use diplomat_core::hir::{self, DocsUrlGenerator, StringEncoding, TypeContext, TypeId};
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

    /// Resolve and format a named type for use in code (without the namespace)
    pub fn fmt_type_name_unnamespaced(&self, id: TypeId) -> Cow<'tcx, str> {
        self.cxx.fmt_type_name_unnamespaced(id)
    }

    /// Resolve and format a named type for use in code
    pub fn fmt_type_name(&self, id: TypeId) -> Cow<'tcx, str> {
        self.cxx.fmt_type_name(id)
    }

    /// Resolve and format the name of a type for use in header names
    pub fn fmt_decl_header_path(&self, id: TypeId) -> String {
        self.cxx.fmt_decl_header_path(id)
    }

    /// Resolve and format the name of a type for use in header names
    pub fn fmt_impl_file_path(&self, id: TypeId) -> String {
        self.cxx.fmt_impl_header_path(id)
    }

    /// Format an enum variant.
    pub fn fmt_enum_variant(&self, variant: &'tcx hir::EnumVariant) -> Cow<'tcx, str> {
        self.cxx.fmt_enum_variant(variant)
    }

    /// Format a field name or parameter name
    // might need splitting in the future if we decide to support renames here
    pub fn fmt_param_name<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        self.cxx.fmt_param_name(ident)
    }

    pub fn fmt_c_type_name(&self, id: TypeId) -> Cow<'tcx, str> {
        self.cxx.fmt_c_type_name(id)
    }

    pub fn fmt_c_ptr<'a>(&self, ident: &'a str, mutability: hir::Mutability) -> Cow<'a, str> {
        self.cxx.fmt_c_ptr(ident, mutability)
    }

    pub fn fmt_borrowed<'a>(&self, ident: &'a str, mutability: hir::Mutability) -> Cow<'a, str> {
        self.cxx.fmt_borrowed(ident, mutability)
    }

    pub fn fmt_move_ref<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        self.cxx.fmt_move_ref(ident)
    }

    pub fn fmt_optional_borrowed<'a>(
        &self,
        ident: &'a str,
        mutability: hir::Mutability,
    ) -> Cow<'a, str> {
        self.cxx.fmt_optional_borrowed(ident, mutability)
    }

    pub fn fmt_owned<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        self.cxx.fmt_owned(ident)
    }

    pub fn fmt_borrowed_slice<'a>(
        &self,
        ident: &'a str,
        mutability: hir::Mutability,
    ) -> Cow<'a, str> {
        self.cxx.fmt_borrowed_slice(ident, mutability)
    }

    pub fn fmt_borrowed_str(&self, encoding: StringEncoding) -> Cow<'static, str> {
        self.cxx.fmt_borrowed_str(encoding)
    }

    /// Format a method
    pub fn fmt_method_name<'a>(&self, method: &'a hir::Method) -> Cow<'a, str> {
        self.cxx.fmt_method_name(method)
    }

    pub fn namespace_c_method_name(&self, ty: TypeId, name: &str) -> String {
        self.cxx.namespace_c_method_name(ty, name)
    }

    /// Get the primitive type as a C type
    pub fn fmt_primitive_as_c(&self, prim: hir::PrimitiveType) -> Cow<'static, str> {
        self.cxx.fmt_primitive_as_c(prim)
    }
}
