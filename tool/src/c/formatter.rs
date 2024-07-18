//! This module contains functions for formatting types

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
pub(crate) struct CFormatter<'tcx> {
    tcx: &'tcx TypeContext,
    is_for_cpp: bool,
}

pub(crate) const CAPI_NAMESPACE: &str = "capi";

impl<'tcx> CFormatter<'tcx> {
    pub fn new(tcx: &'tcx TypeContext, is_for_cpp: bool) -> Self {
        Self { tcx, is_for_cpp }
    }
    pub fn tcx(&self) -> &'tcx TypeContext {
        self.tcx
    }

    /// Resolve and format a named type for use in code (without the namespace)
    pub fn fmt_type_name(&self, id: TypeId) -> Cow<'tcx, str> {
        let resolved = self.tcx.resolve_type(id);
        let name = resolved.name().as_str().into();
        // Only apply renames in cpp mode, in pure C mode you'd want the
        // method names to match the type names.
        // Potential future improvement: Use alias attributes in pure C mode.
        if self.is_for_cpp {
            resolved.attrs().rename.apply(name)
        } else {
            name
        }
    }

    /// Resolve and format a named type for use in code (with a namespace, if needed by C++)
    pub fn fmt_type_name_maybe_namespaced(&self, id: TypeId) -> Cow<'tcx, str> {
        let resolved = self.tcx.resolve_type(id);
        let name: Cow<_> = resolved.name().as_str().into();
        // Only apply renames in cpp mode, in pure C mode you'd want the
        // method names to match the type names.
        // Potential future improvement: Use alias attributes in pure C mode.
        let name = if self.is_for_cpp {
            resolved.attrs().rename.apply(name)
        } else {
            name
        };
        if self.is_for_cpp {
            if let Some(ref ns) = resolved.attrs().namespace {
                format!("{ns}::{CAPI_NAMESPACE}::{name}").into()
            } else {
                format!("diplomat::{CAPI_NAMESPACE}::{name}").into()
            }
        } else {
            name
        }
    }

    /// Resolve and format the name of a type for use in header names: decl version
    //
    /// Enums can't be forward-declared in C, but we do want enums to have methods,
    /// which may require additional #includes leading to potential cycles.
    /// To handle this, we make a separate header file called Foo_decl.h, that contains
    /// *just* the enum. It is included from Foo.h, and external users should not be importing
    /// it directly. (We can potentially add a #define guard that makes this actually private, if needed)
    pub fn fmt_decl_header_path(&self, id: TypeId) -> String {
        let type_name = self.fmt_type_name(id);
        format!("{type_name}.d.h")
    }
    /// Resolve and format the name of a type for use in header names: impl version
    pub fn fmt_impl_header_path(&self, id: TypeId) -> String {
        let type_name = self.fmt_type_name(id);
        format!("{type_name}.h")
    }

    /// Format an enum variant.
    pub fn fmt_enum_variant(
        &self,
        type_name: &str,
        variant: &'tcx hir::EnumVariant,
    ) -> Cow<'tcx, str> {
        let variant_name = variant.name.as_str();
        format!("{type_name}_{variant_name}").into()
    }
    /// Format a field name or parameter name
    // might need splitting in the future if we decide to support renames here
    pub fn fmt_param_name<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        ident.into()
    }

    pub fn fmt_ptr<'a>(&self, ident: &'a str, mutability: hir::Mutability) -> Cow<'a, str> {
        // TODO: Where is the right place to put `const` here?
        if mutability.is_mutable() {
            format!("{ident}*").into()
        } else {
            format!("const {ident}*").into()
        }
    }

    /// Get the primitive type as a C type
    pub fn fmt_primitive_as_c(&self, prim: hir::PrimitiveType) -> Cow<'static, str> {
        use diplomat_core::hir::{FloatType, IntSizeType, IntType, PrimitiveType};
        let s = match prim {
            PrimitiveType::Bool => "bool",
            PrimitiveType::Char => "char32_t",
            PrimitiveType::Int(IntType::I8) => "int8_t",
            PrimitiveType::Int(IntType::U8) | PrimitiveType::Byte => "uint8_t",
            PrimitiveType::Int(IntType::I16) => "int16_t",
            PrimitiveType::Int(IntType::U16) => "uint16_t",
            PrimitiveType::Int(IntType::I32) => "int32_t",
            PrimitiveType::Int(IntType::U32) => "uint32_t",
            PrimitiveType::Int(IntType::I64) => "int64_t",
            PrimitiveType::Int(IntType::U64) => "uint64_t",
            PrimitiveType::Int128(_) => panic!("i128 not supported in C"),
            PrimitiveType::IntSize(IntSizeType::Isize) => "intptr_t",
            PrimitiveType::IntSize(IntSizeType::Usize) => "size_t",
            PrimitiveType::Float(FloatType::F32) => "float",
            PrimitiveType::Float(FloatType::F64) => "double",
        };
        s.into()
    }
    /// Get the primitive type as a C type
    pub fn fmt_primitive_slice_name(
        &self,
        borrow: Option<hir::Borrow>,
        prim: hir::PrimitiveType,
    ) -> String {
        use diplomat_core::hir::{FloatType, IntSizeType, IntType, PrimitiveType};
        let prim = match prim {
            PrimitiveType::Bool => "Bool",
            PrimitiveType::Char => "Char",
            PrimitiveType::Int(IntType::I8) => "I8",
            PrimitiveType::Int(IntType::U8) | PrimitiveType::Byte => "U8",
            PrimitiveType::Int(IntType::I16) => "I16",
            PrimitiveType::Int(IntType::U16) => "U16",
            PrimitiveType::Int(IntType::I32) => "I32",
            PrimitiveType::Int(IntType::U32) => "U32",
            PrimitiveType::Int(IntType::I64) => "I64",
            PrimitiveType::Int(IntType::U64) => "U64",
            PrimitiveType::Int128(_) => panic!("i128 not supported in C"),
            PrimitiveType::IntSize(IntSizeType::Isize) => "Isize",
            PrimitiveType::IntSize(IntSizeType::Usize) => "Usize",
            PrimitiveType::Float(FloatType::F32) => "F32",
            PrimitiveType::Float(FloatType::F64) => "F64",
        };
        let mtb = match borrow {
            Some(borrow) if borrow.mutability.is_immutable() => "",
            _ => "Mut",
        };
        if self.is_for_cpp {
            format!("diplomat::capi::Diplomat{prim}View{mtb}")
        } else {
            format!("Diplomat{prim}View{mtb}")
        }
    }

    pub(crate) fn fmt_diplomat_write(&self) -> &'static str {
        if self.is_for_cpp {
            "diplomat::capi::DiplomatWrite*"
        } else {
            "DiplomatWrite*"
        }
    }
}
