//! This module contains functions for formatting types

use crate::c::{CFormatter, CAPI_NAMESPACE};
use diplomat_core::hir::{
    self, DocsUrlGenerator, SpecialMethod, StringEncoding, TypeContext, TypeId,
};
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
pub(crate) struct Cpp2Formatter<'tcx> {
    pub c: CFormatter<'tcx>,
}

impl<'tcx> Cpp2Formatter<'tcx> {
    pub fn new(tcx: &'tcx TypeContext, docs_url_gen: &'tcx DocsUrlGenerator) -> Self {
        Self {
            c: CFormatter::new(tcx, true, docs_url_gen),
        }
    }

    /// Resolve and format a named type for use in code (without the namespace)
    pub fn fmt_type_name_unnamespaced(&self, id: TypeId) -> Cow<'tcx, str> {
        let resolved = self.c.tcx().resolve_type(id);

        resolved
            .attrs()
            .rename
            .apply(resolved.name().as_str().into())
    }

    /// Resolve and format a named type for use in code
    pub fn fmt_type_name(&self, id: TypeId) -> Cow<'tcx, str> {
        let resolved = self.c.tcx().resolve_type(id);
        let name = resolved
            .attrs()
            .rename
            .apply(resolved.name().as_str().into());
        if let Some(ref ns) = resolved.attrs().namespace {
            format!("{ns}::{name}").into()
        } else {
            name
        }
    }

    /// Resolve and format the name of a type for use in header names
    pub fn fmt_decl_header_path(&self, id: TypeId) -> String {
        let resolved = self.c.tcx().resolve_type(id);
        let type_name = resolved
            .attrs()
            .rename
            .apply(resolved.name().as_str().into());
        if let Some(ref ns) = resolved.attrs().namespace {
            let ns = ns.replace("::", "/");
            format!("{ns}/{type_name}.d.hpp")
        } else {
            format!("{type_name}.d.hpp")
        }
    }

    /// Resolve and format the name of a type for use in header names
    pub fn fmt_impl_header_path(&self, id: TypeId) -> String {
        let resolved = self.c.tcx().resolve_type(id);
        let type_name = resolved
            .attrs()
            .rename
            .apply(resolved.name().as_str().into());
        if let Some(ref ns) = resolved.attrs().namespace {
            let ns = ns.replace("::", "/");
            format!("{ns}/{type_name}.hpp")
        } else {
            format!("{type_name}.hpp")
        }
    }

    /// Format an enum variant.
    pub fn fmt_enum_variant(&self, variant: &'tcx hir::EnumVariant) -> Cow<'tcx, str> {
        variant.attrs.rename.apply(variant.name.as_str().into())
    }

    /// Format the name of a c enum variant given the c name of the type it is on.
    /// This will be namespaced if the ctype is, else not
    pub fn fmt_c_enum_variant<'a>(
        &self,
        ctype: &'a str,
        variant: &'tcx hir::EnumVariant,
    ) -> Cow<'tcx, str> {
        self.c.fmt_enum_variant(ctype, variant)
    }

    /// Format a field name or parameter name
    // might need splitting in the future if we decide to support renames here
    pub fn fmt_param_name<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        self.fmt_identifier(ident.into())
    }

    pub fn fmt_c_type_name(&self, id: TypeId) -> Cow<'tcx, str> {
        self.c.fmt_type_name_maybe_namespaced(id.into())
    }

    pub fn fmt_c_ptr<'a>(&self, ident: &'a str, mutability: hir::Mutability) -> Cow<'a, str> {
        self.c.fmt_ptr(ident, mutability)
    }

    pub fn fmt_optional(&self, ident: &str) -> String {
        format!("std::optional<{ident}>")
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

    pub fn fmt_borrowed_str(&self, encoding: StringEncoding) -> Cow<'static, str> {
        // TODO: This needs to change if an abstraction other than std::u8string_view is used
        match encoding {
            StringEncoding::Utf8 | StringEncoding::UnvalidatedUtf8 => "std::string_view".into(),
            StringEncoding::UnvalidatedUtf16 => "std::u16string_view".into(),
            _ => unreachable!(),
        }
    }

    pub fn fmt_owned_str(&self) -> Cow<'static, str> {
        "std::string".into()
    }

    pub fn fmt_docs(&self, docs: &hir::Docs) -> String {
        self.c.fmt_docs(docs)
    }

    /// Format a method
    pub fn fmt_method_name<'a>(&self, method: &'a hir::Method) -> Cow<'a, str> {
        match method.attrs.special_method {
            Some(SpecialMethod::Indexer) => "operator[]".into(),
            Some(SpecialMethod::Add) => "operator+".into(),
            Some(SpecialMethod::Sub) => "operator-".into(),
            Some(SpecialMethod::Mul) => "operator*".into(),
            Some(SpecialMethod::Div) => "operator/".into(),
            Some(SpecialMethod::AddAssign) => "operator+=".into(),
            Some(SpecialMethod::SubAssign) => "operator-=".into(),
            Some(SpecialMethod::MulAssign) => "operator*=".into(),
            Some(SpecialMethod::DivAssign) => "operator/=".into(),
            Some(_) | None => {
                self.fmt_identifier(method.attrs.rename.apply(method.name.as_str().into()))
            }
        }
    }

    pub fn namespace_c_method_name(&self, ty: TypeId, name: &str) -> String {
        let resolved = self.c.tcx().resolve_type(ty);
        if let Some(ref ns) = resolved.attrs().namespace {
            format!("{ns}::{CAPI_NAMESPACE}::{name}")
        } else {
            format!("diplomat::{CAPI_NAMESPACE}::{name}")
        }
    }

    /// Get the primitive type as a C type
    pub fn fmt_primitive_as_c(&self, prim: hir::PrimitiveType) -> Cow<'static, str> {
        self.c.fmt_primitive_as_c(prim)
    }

    /// Replace any keywords used
    pub fn fmt_identifier<'a>(&self, name: Cow<'a, str>) -> Cow<'a, str> {
        self.c.fmt_identifier(name)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use proc_macro2::TokenStream;

    pub fn new_tcx(tk_stream: TokenStream) -> TypeContext {
        let file = syn::parse2::<syn::File>(tk_stream).unwrap();

        let mut attr_validator = hir::BasicAttributeValidator::new("cpp_test");
        attr_validator.support = super::super::attr_support();

        match TypeContext::from_syn(&file, attr_validator) {
            Ok(context) => context,
            Err(e) => {
                for (_cx, err) in e {
                    eprintln!("Lowering error: {}", err);
                }
                panic!("Failed to create context")
            }
        }
    }
}
