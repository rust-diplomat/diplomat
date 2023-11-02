//! This module contains functions for formatting types

use crate::c2::CFormatter;
use diplomat_core::ast::{DocsUrlGenerator, MarkdownStyle};
use diplomat_core::hir::{self, TypeContext, TypeId};
use heck::ToLowerCamelCase;
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
pub(super) struct DartFormatter<'tcx> {
    c: CFormatter<'tcx>,
    docs_url_generator: &'tcx DocsUrlGenerator,
}

impl<'tcx> DartFormatter<'tcx> {
    pub fn new(tcx: &'tcx TypeContext, docs_url_generator: &'tcx DocsUrlGenerator) -> Self {
        Self {
            c: CFormatter::new(tcx),
            docs_url_generator,
        }
    }

    pub fn fmt_docs(&self, docs: &hir::Docs) -> String {
        docs.to_markdown(self.docs_url_generator, MarkdownStyle::Normal)
            .replace('\n', "\n/// ")
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

    /// Format an enum variant.
    pub fn fmt_enum_variant(&self, variant: &'tcx hir::EnumVariant) -> Cow<'tcx, str> {
        if let Some(rename) = variant.attrs.rename.as_ref() {
            rename.into()
        } else {
            variant.name.as_str().into()
        }
    }

    /// Format a field name or parameter name
    // might need splitting in the future if we decide to support renames here
    pub fn fmt_param_name<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        ident.to_lower_camel_case().into()
    }

    pub fn fmt_nullable<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        format!("{ident}?").into()
    }

    pub fn fmt_primitive_list<'a>(&self, prim: hir::PrimitiveType) -> Cow<'a, str> {
        use diplomat_core::hir::{FloatType, IntType, PrimitiveType};
        match prim {
            PrimitiveType::Char => "Uint32List",
            PrimitiveType::Int(IntType::I8) => "Int8List",
            PrimitiveType::Int(IntType::U8) => "Uint8List",
            PrimitiveType::Int(IntType::I16) => "Int16List",
            PrimitiveType::Int(IntType::U16) => "Uint16List",
            PrimitiveType::Int(IntType::I32) => "Int32List",
            PrimitiveType::Int(IntType::U32) => "Uint32List",
            PrimitiveType::Int(IntType::I64) => "Int64List",
            PrimitiveType::Int(IntType::U64) => "Uint64List",
            PrimitiveType::Int128(_) => panic!("i128 not supported in Dart"),
            PrimitiveType::IntSize(_) => "Uint64List", // TODO this won't work but is used by ICU4X
            PrimitiveType::Float(FloatType::F32) => "Float32List",
            PrimitiveType::Float(FloatType::F64) => "Float64List",
            _ => panic!("Primitive {:?} not supported in lists", prim),
        }
        .into()
    }

    pub fn fmt_string(&self) -> Cow<'static, str> {
        "String".into()
    }

    /// Format a method
    pub fn fmt_method_name<'a>(&self, method: &'a hir::Method) -> Cow<'a, str> {
        // TODO(#60): handle other keywords
        if let Some(rename) = method.attrs.rename.as_ref() {
            rename.into()
        } else {
            method.name.as_str().to_lower_camel_case().into()
        }
    }

    pub fn fmt_c_method_name<'a>(&self, ty: TypeId, method: &'a hir::Method) -> Cow<'a, str> {
        self.c.fmt_method_name(ty, method).into()
    }

    /// Get the primitive type as a Dart FFI type
    pub fn fmt_primitive_as_ffi(&self, prim: hir::PrimitiveType) -> Cow<'static, str> {
        use diplomat_core::hir::{FloatType, IntSizeType, IntType, PrimitiveType};
        match prim {
            PrimitiveType::Bool => "ffi.Bool",
            PrimitiveType::Char => "ffi.Uint32",
            PrimitiveType::Int(IntType::I8) => "ffi.Int8",
            PrimitiveType::Int(IntType::U8) => "ffi.Uint8",
            PrimitiveType::Int(IntType::I16) => "ffi.Int16",
            PrimitiveType::Int(IntType::U16) => "ffi.Uint16",
            PrimitiveType::Int(IntType::I32) => "ffi.Int32",
            PrimitiveType::Int(IntType::U32) => "ffi.Uint32",
            PrimitiveType::Int(IntType::I64) => "ffi.Int64",
            PrimitiveType::Int(IntType::U64) => "ffi.Uint64",
            PrimitiveType::Int128(_) => panic!("i128 not supported in Dart"),
            // TODO: verify these
            PrimitiveType::IntSize(IntSizeType::Isize) => "ffi.Int64",
            PrimitiveType::IntSize(IntSizeType::Usize) => "ffi.Uint64",
            PrimitiveType::Float(FloatType::F32) => "ffi.Float",
            PrimitiveType::Float(FloatType::F64) => "ffi.Double",
        }
        .into()
    }

    /// Get the primitive type as a Dart type
    pub fn fmt_primitive_as_dart(&self, prim: hir::PrimitiveType) -> Cow<'static, str> {
        use diplomat_core::hir::PrimitiveType;
        match prim {
            PrimitiveType::Bool => "bool",
            PrimitiveType::Char => "int",
            PrimitiveType::Int(_) => "int",
            PrimitiveType::Int128(_) => panic!("i128 not supported in Dart"),
            PrimitiveType::IntSize(_) => "int",
            PrimitiveType::Float(_) => "double",
        }
        .into()
    }
}
