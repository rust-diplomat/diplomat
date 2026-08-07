//! C# code generation for the .NET backend.
//!
//! Skeleton only: most paths are `todo!()` / `unimplemented!()` and meant to be
//! filled in incrementally — opaques first, then primitives, then slices,
//! strings, results, options, and finally callbacks.
//!
//! Module layout:
//!
//! * [`opaque`] — `Raw[T].cs` `[DllImport]` declarations + the idiomatic
//!   wrapper class (finalizer-only by default, optional public `IDisposable`).
//!   Self-contained for a single
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
use diplomat_core::hir::{self, DocsUrlGenerator, EnumDef, OpaqueDef, OutStructDef, TypeContext};

use crate::dotnet::r#gen::callback::DotnetCallback;
use crate::{dotnet::gen::fillable::DotnetResult, ErrorStore};

use super::formatter::DotnetFormatter;

mod accessor;
mod callback;
pub(super) mod fillable;
mod impl_struct;
mod lower;
mod method;
mod opaque;

use self::accessor::{AccessorInfo, PropertyInfo};
use self::impl_struct::StructField;
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

/// One rendered type: its display name and the optional raw + idiomatic C#.
pub(super) struct RenderedType {
    pub(super) display_name: String,
    pub(super) raw: Option<String>,
    pub(super) content: String,
}

/// A type whose render data is built but not yet emitted. The two-phase split
/// (build all, then render all) lets the run compute whether ANY type pins a
/// slice before rendering opaque cleanup sweeps that reference the pin helper.
enum PreparedType<'tcx> {
    /// No dependency on the run-level pin flag — already rendered (enums).
    Prerendered {
        display_name: String,
        raw: Option<String>,
        content: String,
    },
    Opaque {
        display_name: String,
        opaque_def: &'tcx OpaqueDef,
        members: TypeMembers<'tcx>,
    },
    Struct {
        display_name: String,
        fields: Vec<StructField>,
        members: TypeMembers<'tcx>,
    },
}

/// A type's lowered methods, routed to where they render.
///
/// `raw_methods` keeps every one of them in declaration order: the raw layer
/// declares one P/Invoke per Rust method however the idiomatic layer presents it,
/// and the run-level helper flags have to see accessors too — a getter returning
/// an owned byte slice needs `RustVec.cs` emitted just like a method would.
struct TypeMembers<'tcx> {
    raw_methods: Vec<MethodInfo<'tcx>>,
    methods: Vec<MethodInfo<'tcx>>,
    properties: Vec<PropertyInfo<'tcx>>,
}

impl PreparedType<'_> {
    fn display_name(&self) -> &str {
        match self {
            Self::Prerendered { display_name, .. }
            | Self::Opaque { display_name, .. }
            | Self::Struct { display_name, .. } => display_name,
        }
    }

    /// Every lowered method, accessors included, or nothing for an
    /// already-rendered type.
    fn all_methods(&self) -> &[MethodInfo<'_>] {
        match self {
            Self::Prerendered { .. } => &[],
            Self::Opaque { members, .. } | Self::Struct { members, .. } => &members.raw_methods,
        }
    }

    /// True iff any of this type's methods pins a borrowed slice.
    fn uses_pinned_memory(&self) -> bool {
        self.all_methods().iter().any(|m| m.has_pinned_inputs())
    }

    /// True iff any method returns an owned `Box<[u8]>`; gates its helpers.
    fn uses_owned_byte_slice_return(&self) -> bool {
        self.all_methods()
            .iter()
            .any(|m| m.return_type.is_owned_byte_slice())
    }

    /// True iff any of this type's methods returns a `DiplomatBorrowedSpan<T>`.
    /// Gates emitting that helper type the same way `uses_pinned_memory` gates
    /// `DiplomatPinnedMemory` — independent flag, since a method can return a
    /// borrowed span without pinning any input (e.g. borrowing only from `self`).
    fn uses_borrowed_span(&self) -> bool {
        self.all_methods().iter().any(|m| m.returns_borrowed_span())
    }
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

    /// Render every non-disabled type to `(display_name, raw, content)`,
    /// alongside the run-level `uses_pinned_memory`, `uses_owned_byte_slice_return`,
    /// and `uses_borrowed_span` flags — these gate which System.Memory-only
    /// runtime helper files (`DiplomatPinnedMemory.cs`, `DiplomatBorrowedSpan.cs`,
    /// the owned-byte-slice-return helpers) actually get emitted, independent
    /// of per-type rendering. Types are BUILT first (each method lowered
    /// exactly once, which is also where result/option struct registration
    /// and diagnostics happen) so every flag reflects the whole run before
    /// any file is written.
    pub(super) fn render_all_types(&self) -> (bool, bool, bool, Vec<RenderedType>) {
        let mut prepared_types = Vec::new();
        let mut uses_pinned_memory = false;
        let mut uses_owned_byte_slice_return = false;
        let mut uses_borrowed_span = false;
        for (id, ty) in self.tcx.all_types() {
            if ty.attrs().disable {
                continue;
            }
            let display_name = self.formatter.fmt_type_name(id).into_owned();
            // Attribute any diagnostic pushed while lowering this type to it.
            let _guard = self.errors.set_context_ty(display_name.clone().into());
            // `None` means an unsupported shape (diagnostic already recorded);
            // the end-gate in `lib.rs` aborts before any file is written.
            let Some(prepared) = self.prepare_type(display_name, ty) else {
                continue;
            };
            uses_pinned_memory |= prepared.uses_pinned_memory();
            uses_owned_byte_slice_return |= prepared.uses_owned_byte_slice_return();
            uses_borrowed_span |= prepared.uses_borrowed_span();
            prepared_types.push(prepared);
        }

        let rendered = prepared_types
            .into_iter()
            .map(|prepared| {
                let display_name = prepared.display_name().to_string();
                let (raw, content) = self.render_prepared(prepared);
                RenderedType {
                    display_name,
                    raw,
                    content,
                }
            })
            .collect();
        (
            uses_pinned_memory,
            uses_owned_byte_slice_return,
            uses_borrowed_span,
            rendered,
        )
    }

    /// Build a type's render data without emitting any C#. `build_method_info`
    /// runs exactly once per method here (it registers result/option structs
    /// and pushes diagnostics — running it twice would double both), so the
    /// run can learn whether ANY type pins a slice before rendering the first
    /// one. `None` (diagnostic recorded) for an unsupported type shape.
    fn prepare_type(
        &self,
        display_name: String,
        ty: hir::TypeDef<'tcx>,
    ) -> Option<PreparedType<'tcx>> {
        Some(match ty {
            hir::TypeDef::Struct(struct_def) => {
                // An unsupported field type skips the whole struct — there's
                // no partial struct to emit.
                let fields = self.lower_fields(struct_def)?;
                let field_names: Vec<&str> =
                    fields.iter().map(|field| field.name.as_str()).collect();
                let members = self.build_members(
                    &display_name,
                    &struct_def.methods,
                    &field_names,
                    false,
                    false,
                );
                PreparedType::Struct {
                    display_name,
                    fields,
                    members,
                }
            }
            hir::TypeDef::OutStruct(out_struct_def) => {
                self.gen_out_struct(out_struct_def);
                return None;
            }
            hir::TypeDef::Opaque(opaque_def) => {
                let members = self.build_members(
                    &display_name,
                    &opaque_def.methods,
                    &[],
                    true,
                    opaque_def.attrs.manually_disposable,
                );
                PreparedType::Opaque {
                    display_name,
                    opaque_def,
                    members,
                }
            }
            hir::TypeDef::Enum(enum_def) => {
                // Enums never reference the pin helper, so render eagerly.
                let (raw, content) = self.gen_enum(display_name.clone(), enum_def)?;
                PreparedType::Prerendered {
                    display_name,
                    raw,
                    content,
                }
            }
            _ => unreachable!("unexpected type variant"),
        })
    }

    /// Render a prepared type to `(raw, content)`.
    fn render_prepared(&self, prepared: PreparedType<'tcx>) -> (Option<String>, String) {
        match prepared {
            PreparedType::Prerendered { raw, content, .. } => (raw, content),
            PreparedType::Opaque {
                display_name,
                opaque_def,
                members,
            } => {
                let TypeMembers {
                    raw_methods,
                    methods,
                    properties,
                } = members;
                let raw = self.gen_opaque_raw(display_name.clone(), opaque_def, raw_methods);
                let content = self.gen_opaque_impl(
                    display_name,
                    methods,
                    properties,
                    opaque_def.attrs.manually_disposable,
                );
                (Some(raw), content)
            }
            PreparedType::Struct {
                display_name,
                fields,
                members,
            } => {
                let TypeMembers {
                    raw_methods,
                    methods,
                    properties,
                } = members;
                let raw = self.gen_struct_raw(display_name.clone(), fields.clone(), raw_methods);
                // Structs are value types with no Dispose, so no pin sweep —
                // their only pin references live in per-method bodies.
                let content = self.gen_struct_impl(display_name, fields, methods, properties);
                (Some(raw), content)
            }
        }
    }

    /// Lower each method exactly once and route it as it is built: an accessor
    /// goes into the property it backs, everything else stays a method. The
    /// `MethodInfo`s are shared with the raw template, so no method is lowered
    /// twice. A method with an unsupported shape is dropped (its diagnostic was
    /// recorded during lowering); the end-gate aborts the whole run before any
    /// file is written.
    fn build_members(
        &self,
        display_name: &str,
        methods: &'tcx [hir::Method],
        field_names: &[&str],
        is_opaque: bool,
        has_generated_dispose: bool,
    ) -> TypeMembers<'tcx> {
        let lowered: Vec<(Option<AccessorInfo>, MethodInfo<'tcx>)> = methods
            .iter()
            .filter_map(|m| self.build_method_info(StructMethodContext::new(m)))
            .collect();
        let raw_methods = lowered.iter().map(|(_, m)| m.clone()).collect();
        let (methods, properties) = accessor::route_members(lowered, self.errors);
        accessor::reject_member_collisions(
            display_name,
            &properties,
            &methods,
            field_names,
            is_opaque,
            has_generated_dispose,
            self.errors,
        );
        TypeMembers {
            raw_methods,
            methods,
            properties,
        }
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
// `PartialEq`/`Eq` so an accessor's property type can be compared against its
// counterpart's — see `gen::accessor`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
