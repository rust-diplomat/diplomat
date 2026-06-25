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

use self::method::{MethodInfo, StructMethodContext};

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
    ) -> Option<(Option<String>, String)> {
        // A Diplomat enum crosses the FFI boundary as a plain C `enum`, i.e.
        // `int`-width (see the C backend's `enum.h.jinja`). The generated C#
        // enum is therefore always `: int`. A discriminant that doesn't fit
        // `i32` can't be represented on the wire at all, so reject it with a
        // diagnostic rather than emit a `long` that would silently mismatch
        // the C ABI when the enum is passed by value (e.g. as a struct field).
        if let Some(bad) = enum_def
            .variants
            .iter()
            .find(|v| i32::try_from(v.discriminant).is_err())
        {
            self.errors.push_error(format!(
                "[.NET backend] enum `{display_name}` variant `{}` has discriminant \
                 {} outside the `i32` range; Diplomat represents enums as a C `int` \
                 on the wire, so this value cannot be represented.",
                bad.name, bad.discriminant
            ));
            return None;
        }
        let variants = enum_def
            .variants
            .iter()
            .map(|variant| EnumVariantInfo {
                name: self.formatter.fmt_enum_variant(variant).into_owned(),
                discriminant: variant.discriminant,
            })
            .collect();
        Some((
            None,
            EnumTemplate {
                namespace: self.namespace,
                name: display_name,
                variants,
            }
            .render()
            .unwrap(),
        ))
    }

    pub(crate) fn gen_opaque(
        &self,
        display_name: String,
        opaque_def: &'tcx OpaqueDef,
    ) -> Option<(Option<String>, String)> {
        // Lower every method exactly once, then hand the same `MethodInfo`s
        // to both the raw and idiomatic templates. Lowering twice (once per
        // template) would push every unsupported-shape diagnostic twice.
        // A method that uses an unsupported shape is dropped here (the
        // diagnostic was recorded during lowering); the end-gate aborts the
        // whole run before any file is written.
        let methods: Vec<MethodInfo<'tcx>> = opaque_def
            .methods
            .iter()
            .filter_map(|m| self.build_method_info(StructMethodContext::new(m)))
            .collect();
        let raw = self.gen_opaque_raw(display_name.clone(), opaque_def, methods.clone());
        // Only lifetime-carrying opaques can be a borrowing return, so only
        // they need keep-alive edge storage.
        let has_edges = opaque_def.lifetimes.num_lifetimes() != 0;
        let content = self.gen_opaque_impl(display_name, methods, has_edges);
        Some((Some(raw), content))
    }

    pub(crate) fn gen_struct(
        &self,
        display_name: String,
        struct_def: &'tcx StructDef,
    ) -> Option<(Option<String>, String)> {
        // An unsupported field type skips the whole struct — there's no
        // partial struct to emit. Methods are lowered once and shared
        // between the raw and idiomatic templates (see `gen_opaque`).
        let fields = self.lower_fields(struct_def)?;
        let methods: Vec<MethodInfo<'tcx>> = struct_def
            .methods
            .iter()
            .filter_map(|m| self.build_method_info(StructMethodContext::new(m)))
            .collect();
        let raw = self.gen_struct_raw(display_name.clone(), fields.clone(), methods.clone());
        let content = self.gen_struct_impl(display_name, fields, methods);
        Some((Some(raw), content))
    }

    pub(crate) fn gen_out_struct(
        &self,
        out_struct_def: &'tcx OutStructDef,
    ) -> Option<(Option<String>, String)> {
        // `#[diplomat::out]` structs (return-position-only structs whose
        // fields can hold types like owned slices and Rust-owned opaques
        // by value) need their own codegen path — the regular struct
        // templates only handle primitive / enum fields. There's no
        // `BackendAttrSupport` flag to reject them at HIR validation, so
        // we record a diagnostic and skip the type here.
        self.errors.push_error(format!(
            "[.NET backend] out struct (`#[diplomat::out] struct {}`) is \
             not yet supported. Convert to a regular struct or wrap the \
             return in an opaque if possible.",
            out_struct_def.name
        ));
        None
    }

    /// Lower a HIR primitive to its C# vocabulary. Returns `None` (after
    /// recording a diagnostic) for primitives the backend can't represent
    /// yet, so callers skip the offending method/type rather than panic.
    pub(super) fn lower_primitive(
        &self,
        primitive: &hir::PrimitiveType,
    ) -> Option<DotnetPrimitives> {
        Some(match primitive {
            hir::PrimitiveType::Bool => DotnetPrimitives::Bool,
            hir::PrimitiveType::Char => DotnetPrimitives::UInt,
            hir::PrimitiveType::Byte => DotnetPrimitives::Byte,
            hir::PrimitiveType::Ordering => {
                self.errors.push_error(
                    "[.NET backend] `Ordering` primitive is not yet supported".to_string(),
                );
                return None;
            }
            hir::PrimitiveType::Int(int_type) => match int_type {
                hir::IntType::I8 => DotnetPrimitives::SByte,
                hir::IntType::I16 => DotnetPrimitives::Short,
                hir::IntType::I32 => DotnetPrimitives::Int,
                hir::IntType::I64 => DotnetPrimitives::Long,
                hir::IntType::U8 => DotnetPrimitives::Byte,
                hir::IntType::U16 => DotnetPrimitives::UShort,
                hir::IntType::U32 => DotnetPrimitives::UInt,
                hir::IntType::U64 => DotnetPrimitives::ULong,
            },
            hir::PrimitiveType::IntSize(int_size_type) => match int_size_type {
                hir::IntSizeType::Isize => DotnetPrimitives::NInt,
                hir::IntSizeType::Usize => DotnetPrimitives::NUInt,
            },
            hir::PrimitiveType::Int128(_) => {
                self.errors.push_error(
                    "[.NET backend] 128-bit integer primitives are not yet supported".to_string(),
                );
                return None;
            }
            hir::PrimitiveType::Float(float_type) => match float_type {
                hir::FloatType::F32 => DotnetPrimitives::Float,
                hir::FloatType::F64 => DotnetPrimitives::Double,
            },
        })
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Primitive type lowering
// ─────────────────────────────────────────────────────────────────────────────

/// C# primitive-type keywords. Mirror of `hir::PrimitiveType` but in C#'s
/// vocabulary — built so the type knows how to render itself (`Display`)
/// instead of a function returning `&'static str`.
///
/// `Ordering` and `Int128` have no C# representation yet; they're rejected
/// in [`ItemGenContext::lower_primitive`] with a recorded diagnostic rather
/// than constructed here.
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

    /// PascalCase token for embedding in generated *type names* (e.g.
    /// `DiplomatOptionDouble`). [`Display`] renders the lowercase C#
    /// keyword (`double`, `int`), which is right in a signature but yields
    /// awkward identifiers like `DiplomatOptiondouble` when concatenated.
    pub(super) fn name_token(&self) -> &'static str {
        match self {
            Self::Bool => "Bool",
            Self::SByte => "SByte",
            Self::Short => "Short",
            Self::Int => "Int",
            Self::Long => "Long",
            Self::Byte => "Byte",
            Self::NInt => "NInt",
            Self::NUInt => "NUInt",
            Self::UShort => "UShort",
            Self::UInt => "UInt",
            Self::ULong => "ULong",
            Self::Float => "Float",
            Self::Double => "Double",
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
