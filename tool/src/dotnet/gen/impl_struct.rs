//! Struct-type codegen.
//!
//! Two outputs per `StructDef`, mirroring the opaque split:
//!
//! 1. **Raw layer** (`Raw<Name>.cs`) — `[StructLayout(Sequential)]` struct
//!    holding the public fields by value, plus `[DllImport]` declarations
//!    for the struct's methods.
//! 2. **Idiomatic layer** (`<Name>.cs`) — public C# `partial struct` with
//!    PascalCase fields and wrapper methods that forward to the raw layer
//!    via `AsFFI()` / `FromFFI(...)` bridge helpers.
//!
//! No `IDisposable`, no `_inner`, no destructor — value type, GC handles
//! cleanup. The bridge methods exist because the raw and idiomatic structs
//! are *separate* C# types (in `Raw.X` vs `X` namespaces) even though they
//! share `[StructLayout(Sequential)]` layout.

use askama::Template;
use diplomat_core::hir::{self, StructDef};

use crate::dotnet::r#gen::method::{self, MethodInfo, PropertyInfo};
use crate::dotnet::r#gen::{DotnetPrimitives, ItemGenContext};

// ─────────────────────────────────────────────────────────────────────────────
// Shared field view (used by both raw and idiomatic templates)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone)]
pub(super) struct StructField {
    /// PascalCase C# field name (e.g. `X`, `Y`), already run through the
    /// formatter so the template can drop it in verbatim.
    name: String,
    field_type: StructFieldType,
}

/// Field type that templates render verbatim. Enums use the same C# type
/// name on both the raw and idiomatic side (they're declared once in the
/// project namespace, not under `Raw.`), and they P/Invoke as their
/// underlying integer with zero ceremony.
#[derive(Clone)]
enum StructFieldType {
    Primitive(DotnetPrimitives),
    Enum(String),
}

impl StructFieldType {
    /// True only for `bool` — used to gate `[MarshalAs(UnmanagedType.U1)]`
    /// on the field. Enums never need this; C#'s default marshaling already
    /// matches Rust's discriminant width.
    fn is_bool(&self) -> bool {
        matches!(self, Self::Primitive(p) if p.is_bool())
    }
}

impl std::fmt::Display for StructFieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Primitive(p) => p.fmt(f),
            Self::Enum(name) => f.write_str(name),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Templates
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Template)]
#[template(path = "dotnet/struct.raw.cs.jinja", escape = "none")]
struct RawStructTemplate<'ctx, 'tcx> {
    /// C#-side name after `#[diplomat::rename]` + keyword escaping.
    name: String,
    fields: Vec<StructField>,
    methods: Vec<MethodInfo<'tcx>>,
    namespace: &'ctx str,
}

#[derive(Template)]
#[template(path = "dotnet/struct.impl.cs.jinja", escape = "none")]
struct ImplStructTemplate<'ctx, 'tcx> {
    /// C#-side name after `#[diplomat::rename]` + keyword escaping.
    name: String,
    namespace: &'ctx str,
    fields: Vec<StructField>,
    methods: Vec<MethodInfo<'tcx>>,
    properties: Vec<PropertyInfo>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Codegen entry points
// ─────────────────────────────────────────────────────────────────────────────

impl<'ctx, 'tcx> ItemGenContext<'ctx, 'tcx> {
    // `fields` and `methods` are lowered once by `gen_struct` and shared with
    // both templates, so an unsupported shape is reported once, not twice.
    pub(super) fn gen_struct_raw(
        &self,
        display_name: String,
        fields: Vec<StructField>,
        methods: Vec<MethodInfo<'tcx>>,
    ) -> String {
        RawStructTemplate {
            namespace: self.namespace,
            name: display_name,
            fields,
            methods,
        }
        .render()
        .expect("Failed to render struct raw template")
    }

    pub(super) fn gen_struct_impl(
        &self,
        display_name: String,
        fields: Vec<StructField>,
        methods: Vec<MethodInfo<'tcx>>,
    ) -> String {
        let properties = method::collect_properties(&methods);

        ImplStructTemplate {
            name: display_name,
            namespace: self.namespace,
            fields,
            methods,
            properties,
        }
        .render()
        .expect("Failed to render struct impl template")
    }

    /// Lower a struct's fields into the `StructField` view used by both raw
    /// and idiomatic templates. Returns `None` (after recording a diagnostic)
    /// for a field type the regular struct templates can't represent, so the
    /// caller skips the whole struct.
    pub(super) fn lower_fields(&self, struct_def: &'tcx StructDef) -> Option<Vec<StructField>> {
        struct_def
            .fields
            .iter()
            .map(|field| {
                let field_type = match &field.ty {
                    hir::Type::Primitive(p) => StructFieldType::Primitive(self.lower_primitive(p)?),
                    hir::Type::Enum(enum_path) => StructFieldType::Enum(self.enum_name(enum_path)),
                    other => {
                        self.errors.push_error(format!(
                            "[.NET backend] only primitive and enum struct fields are \
                             supported; field `{}` has unsupported type {:?}",
                            field.name, other
                        ));
                        return None;
                    }
                };
                Some(StructField {
                    name: self
                        .formatter
                        .fmt_field_name(field.name.as_str())
                        .into_owned(),
                    field_type,
                })
            })
            .collect()
    }
}
