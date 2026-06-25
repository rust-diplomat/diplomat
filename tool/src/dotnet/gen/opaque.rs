//! Opaque-type codegen.
//!
//! Two outputs per `OpaqueDef`:
//!
//! 1. **Raw layer** (`Raw<Name>.cs`) — `[DllImport]` declarations, one per
//!    user method plus the auto-generated `<Name>_destroy`. Fed to
//!    `opaque.raw.cs.jinja`.
//! 2. **Idiomatic layer** (`<Name>.cs`) — `IDisposable`-shaped wrapper class
//!    that calls into the raw layer. Fed to `opaque.impl.cs.jinja`.
//!
//! Both templates consume the same [`super::method::MethodInfo`] — the
//! kind-agnostic, layer-agnostic method view. The split between raw and
//! idiomatic lives entirely in the template files.

use askama::Template;
use diplomat_core::hir::{IdentBuf, OpaqueDef};

use super::method::{self, MethodInfo, PropertyInfo};
use super::ItemGenContext;

// ─────────────────────────────────────────────────────────────────────────────
// Templates
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Template)]
#[template(path = "dotnet/opaque.raw.cs.jinja", escape = "none")]
struct OpaqueRawTemplate<'ctx> {
    /// C#-side name after `#[diplomat::rename]` + keyword escaping.
    name: String,
    methods: Vec<MethodInfo<'ctx>>,
    namespace: &'ctx str,
    dtor_abi_name: &'ctx IdentBuf,
}

#[derive(Template)]
#[template(path = "dotnet/opaque.impl.cs.jinja", escape = "none")]
struct OpaqueImplTemplate<'ctx> {
    /// C#-side name after `#[diplomat::rename]` + keyword escaping.
    name: String,
    namespace: &'ctx str,
    methods: Vec<MethodInfo<'ctx>>,
    properties: Vec<PropertyInfo>,
    has_edges: bool,
}

// ─────────────────────────────────────────────────────────────────────────────
// Codegen entry points
// ─────────────────────────────────────────────────────────────────────────────

impl<'ctx, 'tcx> ItemGenContext<'ctx, 'tcx> {
    // `methods` are lowered once by `gen_opaque` and shared with both
    // templates, so an unsupported shape is reported once, not twice.
    pub(super) fn gen_opaque_raw(
        &self,
        display_name: String,
        opaque_def: &'tcx OpaqueDef,
        methods: Vec<MethodInfo<'tcx>>,
    ) -> String {
        OpaqueRawTemplate {
            // Declaration site name flows through the same formatter
            // as type references — `#[diplomat::rename]` applied,
            // C# reserved words escaped with `@`.
            name: display_name,
            namespace: self.namespace,
            methods,
            dtor_abi_name: &opaque_def.dtor_abi_name,
        }
        .render()
        .unwrap()
    }

    pub(super) fn gen_opaque_impl(
        &self,
        display_name: String,
        methods: Vec<MethodInfo<'tcx>>,
        has_edges: bool,
    ) -> String {
        let properties = method::collect_properties(&methods);

        OpaqueImplTemplate {
            name: display_name,
            namespace: self.namespace,
            methods,
            properties,
            has_edges,
        }
        .render()
        .unwrap()
    }
}
