//! C# code generation for the .NET backend.
//!
//! Skeleton only: most paths are `todo!()` / `unimplemented!()` and meant to be
//! filled in incrementally — opaques first, then primitives, then slices,
//! strings, results, options, and finally callbacks.
//!
//! Module layout:
//!
//! * [`opaque`] — `Raw[T].cs` `[DllImport]` declarations + the idiomatic
//!   `IDisposable`-shaped wrapper class. Self-contained for a single
//!   `OpaqueDef`.
//! * [`lower`] — pure type-leaf lowering shared across opaque / struct /
//!   enum: primitives → C# keywords, opaque paths → `T*`. New backends
//!   (struct, slice) reuse these directly.
//! * `mod.rs` (this file) — [`ItemGenContext`] context struct, the enum
//!   template, and the public dispatch entry points (`gen_enum`,
//!   `gen_opaque`) the parent module routes types to.

use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{self, Display},
};

use askama::Template;
use diplomat_core::hir::{
    self, DocsUrlGenerator, EnumDef, OpaqueDef, OutStructDef, StructDef, TypeContext,
};

use crate::dotnet::r#gen::callback::DotnetCallback;
use crate::{dotnet::gen::fillable::DotnetResult, ErrorStore};

use super::formatter::DotnetFormatter;

mod callback;
pub(super) mod fillable;
mod impl_struct;
mod lower;
mod method;
mod opaque;

// ─────────────────────────────────────────────────────────────────────────────
// Codegen context
// ─────────────────────────────────────────────────────────────────────────────

/// Carries everything `gen_*` methods need to render a single type.
///
/// Mirrors the role of `kotlin::ItemGenContext` / `cpp::ItemGenContext`. Built
/// once in `mod.rs::run` and reused across every type in the `TypeContext`.
#[allow(dead_code)] // fields will be used as gen_* methods are filled in
pub(super) struct ItemGenContext<'ctx, 'tcx> {
    pub tcx: &'tcx TypeContext,
    pub formatter: &'ctx DotnetFormatter<'tcx>,
    pub errors: &'ctx ErrorStore<'tcx, String>,
    pub docs_url_gen: &'ctx DocsUrlGenerator,
    /// Crate-style library name (e.g. `dotnet_smoke`). From `[shared] lib_name`
    /// in the config; used for naming and as the default for `dylib_name`.
    pub lib_name: &'ctx str,
    /// The native cdylib name used in `[DllImport("...")]`. Defaults to
    /// `lib_name` unless `[dotnet] dylib_name` overrides it.
    pub dylib_name: &'ctx str,
    /// The C# namespace generated files declare. Defaults to `lib_name`
    /// in `UpperCamelCase` unless `[dotnet] namespace` overrides it.
    pub namespace: &'ctx str,
    pub exception_trim_suffix: Option<&'ctx str>,
    pub exception_message_method: Option<&'ctx str>,
    pub getters_prefix: Option<&'ctx str>,
    pub setters_prefix: Option<&'ctx str>,

    pub result_struct_registry: RefCell<HashMap<String, DotnetResult>>,
    pub option_struct_registry: RefCell<HashMap<String, fillable::DotnetOption>>,
    pub callback_struct_registry: RefCell<HashMap<String, DotnetCallback>>,
}

#[derive(Template)]
#[template(path = "dotnet/enum.cs.jinja", escape = "none")]
struct EnumTemplate<'ctx> {
    namespace: &'ctx str,
    name: String,
    variants: Vec<EnumVariantInfo>,
}

struct EnumVariantInfo {
    name: String,
    discriminant: isize,
}

// ─────────────────────────────────────────────────────────────────────────────
// Codegen entry points
// ─────────────────────────────────────────────────────────────────────────────

impl<'ctx, 'tcx> ItemGenContext<'ctx, 'tcx> {
    pub(super) fn gen_enum(
        &self,
        display_name: String,
        enum_def: &'tcx EnumDef,
    ) -> (Option<String>, String) {
        let variants = enum_def
            .variants
            .iter()
            .map(|variant| EnumVariantInfo {
                name: self.formatter.fmt_enum_variant(variant).into_owned(),
                discriminant: variant.discriminant,
            })
            .collect();
        (
            None,
            EnumTemplate {
                namespace: self.namespace,
                name: display_name,
                variants,
            }
            .render()
            .unwrap(),
        )
    }

    pub(crate) fn gen_opaque(
        &self,
        display_name: String,
        opaque_def: &'tcx OpaqueDef,
    ) -> (Option<String>, String) {
        (
            self.gen_opaque_raw(display_name.clone(), opaque_def),
            self.gen_opaque_impl(display_name, opaque_def),
        )
    }

    pub(crate) fn gen_struct(
        &self,
        display_name: String,
        struct_def: &'tcx StructDef,
    ) -> (Option<String>, String) {
        (
            self.gen_struct_raw(display_name.clone(), struct_def),
            self.gen_struct_impl(display_name, struct_def),
        )
    }

    pub(crate) fn gen_out_struct(
        &self,
        out_struct_def: &'tcx OutStructDef,
    ) -> (Option<String>, String) {
        // `#[diplomat::out]` structs (return-position-only structs whose
        // fields can hold types like owned slices and Rust-owned opaques
        // by value) need their own codegen path — the regular struct
        // templates only handle primitive / enum fields. There's no
        // `BackendAttrSupport` flag to reject them at HIR validation, so
        // we panic here with a clear message. Picky / IronRDP / the
        // checked-in example don't use them, so this isn't a regression.
        unimplemented!(
            "[.NET backend] out struct (`#[diplomat::out] struct {}`) is \
             not yet supported. Convert to a regular struct or wrap the \
             return in an opaque if possible.",
            out_struct_def.name
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Primitive type lowering
// ─────────────────────────────────────────────────────────────────────────────

/// C# primitive-type keywords. Mirror of `hir::PrimitiveType` but in C#'s
/// vocabulary — built so the type knows how to render itself (`Display`)
/// instead of a function returning `&'static str`.
///
/// Unimplemented variants (Char, Byte, Ordering, IntSize, Int128, Float) hit
/// `todo!()` on construction, preserving the existing behavior of
/// `lower::primitives_to_dotnet_type`.
#[derive(Debug, Clone)]
pub(super) enum DotnetPrimitives {
    Bool,
    SByte,
    Short,
    Int,
    Long,
    Byte,
    NInt,
    NUInt,
    UShort,
    UInt,
    ULong,
    Float,
    Double,
}

impl DotnetPrimitives {
    /// True for the C# `bool` variant. Used by templates to decide
    /// whether to emit `[MarshalAs(UnmanagedType.U1)]` on a field —
    /// the C# default `BOOL` marshaling is 4 bytes, but Rust's
    /// `repr(C)` `bool` is 1 byte and the ABI requires U1.
    pub(super) fn is_bool(&self) -> bool {
        matches!(self, Self::Bool)
    }
}

impl From<&hir::PrimitiveType> for DotnetPrimitives {
    fn from(primitive: &hir::PrimitiveType) -> Self {
        match primitive {
            hir::PrimitiveType::Bool => Self::Bool,
            hir::PrimitiveType::Char => Self::UInt,
            hir::PrimitiveType::Byte => Self::Byte,
            hir::PrimitiveType::Ordering => todo!(),
            hir::PrimitiveType::Int(int_type) => match int_type {
                hir::IntType::I8 => Self::SByte,
                hir::IntType::I16 => Self::Short,
                hir::IntType::I32 => Self::Int,
                hir::IntType::I64 => Self::Long,
                hir::IntType::U8 => Self::Byte,
                hir::IntType::U16 => Self::UShort,
                hir::IntType::U32 => Self::UInt,
                hir::IntType::U64 => Self::ULong,
            },
            hir::PrimitiveType::IntSize(int_size_type) => match int_size_type {
                hir::IntSizeType::Isize => Self::NInt,
                hir::IntSizeType::Usize => Self::NUInt,
            },
            hir::PrimitiveType::Int128(_) => todo!(),
            hir::PrimitiveType::Float(float_type) => match float_type {
                hir::FloatType::F32 => Self::Float,
                hir::FloatType::F64 => Self::Double,
            },
        }
    }
}

impl Display for DotnetPrimitives {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Bool => "bool",
            Self::SByte => "sbyte",
            Self::Short => "short",
            Self::Int => "int",
            Self::Long => "long",
            Self::Byte => "byte",
            Self::NInt => "nint",
            Self::NUInt => "nuint",
            Self::UShort => "ushort",
            Self::UInt => "uint",
            Self::ULong => "ulong",
            Self::Float => "float",
            Self::Double => "double",
        })
    }
}
