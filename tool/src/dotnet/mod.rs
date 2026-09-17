//! .NET backend for Diplomat.
//!
//! Generates C# bindings that call into the Diplomat-generated C ABI via
//! P/Invoke (`[DllImport]` externs with the `Cdecl` calling convention).
//! Every opaque Rust handle maps to a partial class backed by the same
//! `RustHandle<T>` (`tool/templates/dotnet/RustHandle.cs.jinja`) — a resource
//! handle holding one native pointer, its destructor, dependency edges, and
//! borrow state.
//! Opaques are finalizer-only by default; `#[diplomat::attr(dotnet, manually_disposable)]`
//! opts a type into a public `IDisposable` surface.
//! Slices, `&DiplomatStr` (unvalidated UTF-8) and `&DiplomatStr16` pin
//! zero-copy; a validated `&str` still copies, since only a transcode from a
//! real `System.String` can guarantee well-formed UTF-8. Callbacks are
//! pinned via `GCHandle`.
//!
//! This file is the entry point that the Diplomat CLI dispatches to. Codegen
//! itself lives in [`gen`] and naming/type-formatting concerns live in
//! [`formatter`].
//!
//! ## Borrowing / lifetime model
//!
//! The backend does not encode Rust lifetimes in C# types. It uses HIR
//! lifetime-edge analysis (`hir::borrowing_param`) to root supported borrowed
//! outputs and reject the unsafe cases:
//!
//! * `&[u8]` / `&[u32]` / `&DiplomatStr` (`byte[]`) and `&DiplomatStr16`
//!   (`string`) params are pinned with `fixed (...)` for the call and
//!   unpinned immediately after. A validated `&str` is transcoded first
//!   (`Diplomat.Utf8.Clone`, an explicit, separately-named copy — see
//!   `gen::method`'s `Slice::Str` lowering) and then pinned the same way.
//!   When an owned opaque success return borrows a `&[u8]` / `&[u32]` /
//!   `&DiplomatStr` / `&DiplomatStr16` param, that param instead surfaces as
//!   `ReadOnlyMemory` and is pinned via `DiplomatPinnedMemory`, rooted as a
//!   pin holder on the returned handle's edges and unpinned after
//!   the Rust destructor runs. A validated `&str` can't take this path — the
//!   transcode-copy only lives for the call, so this borrow position (and
//!   other borrow positions: borrowed errors, Option-wrapped or non-opaque
//!   returns) are still rejected with a diagnostic. Because `ReadOnlyMemory` /
//!   `MemoryHandle` need the `System.Memory` package on the netstandard2.0 /
//!   .NET Framework floor, the `DiplomatPinnedMemory` helper is emitted only
//!   when a run actually pins (see `uses_pinned_memory`).
//! * Borrowed opaque returns (`&T`, `&mut T`, `Option<&T>`) use a non-owning
//!   handle and keep a managed source-handle edge. Shared views check the
//!   source mutation version before each access; exclusive views hold their
//!   source borrow until cleanup. A retained edge from a
//!   `manually_disposable` source is rejected during generation.
//! * Borrowed string/slice returns (`&'a str` / `&'a [u8]` / `&'a [u32]`) wrap
//!   the same `(ptr, len)` shape as an input slice in `DiplomatBorrowedSpan<T>`,
//!   rooted with the same managed source-handle edges a borrowed opaque return
//!   uses. It exposes `WithSpan(...)` (scoped, zero-copy, read-only access)
//!   and `Clone()` (an explicit, independent `T[]`) — never a bare
//!   `Span`-returning property, since nothing would keep the view's
//!   dependencies valid once the span escaped it. Disposal during a span
//!   callback is rejected before edges are released.
//! * An owned `Box<[u8]>` return wraps the `DiplomatOwnedSliceU8` `(ptr, len)`
//!   struct in `RustVec`, which owns the native allocation and is
//!   `IDisposable`. It offers the same `WithSpan(...)` / `Clone()` shape as
//!   `DiplomatBorrowedSpan<T>`, for the same reason: `MemoryManager<T>` would
//!   force a `GetSpan()` whose result doesn't keep the owner alive.
//! * Borrowed opaque errors (`Result<_, &E>`) are rejected; without a success
//!   arm to carry a keep-alive dependency, `Dispose` would call `Destroy` on
//!   a pointer Rust still owns (double-free).
//! * Lifetime-carrying owned returns (`Box<T<'a>>`) from opaque wrappers get
//!   XML lifetime remarks.
//!
//! ## Resource ownership
//!
//! Every opaque wrapper has one `RustHandle<Raw.T>? _inner`. The resource handle
//! owns one native pointer and runs its destructor at most once. A dependent
//! owns an ordinary managed source-handle edge plus borrow bookkeeping; that
//! edge keeps the source handle reachable without adding native ownership.
//! Methods that would retain such an edge from a `manually_disposable` source
//! fail generation. Application code must synchronize calls and disposal across
//! threads; this backend does not promise that race to be safe. Finalizer
//! cleanup still uses atomic state so exactly-once bookkeeping remains valid on
//! the finalizer thread.

use askama::Template;
use diplomat_core::hir::{BackendAttrSupport, DocsUrlGenerator, TypeContext};
use heck::ToUpperCamelCase;
use serde::{Deserialize, Serialize};

use crate::{dotnet::formatter::DotnetFormatter, Config, ErrorStore, FileMap};

mod formatter;
mod gen;

// ─────────────────────────────────────────────────────────────────────────────
// Runtime helpers — emitted once per generation run, independent of HIR.
// ─────────────────────────────────────────────────────────────────────────────

/// `DiplomatSliceU8` — the `repr(C)` fat pointer that crosses the FFI
/// boundary for every `&DiplomatStr` / `&[u8]` param. Namespace is
/// project-specific, so this is templated rather than `include_str!`'d.
#[derive(Template)]
#[template(path = "dotnet/DiplomatSliceU8.cs.jinja", escape = "none")]
struct DiplomatSliceU8Template<'a> {
    namespace: &'a str,
}

/// `DiplomatSliceMutU8` — the mutable counterpart, used for `&mut [u8]`
/// params. Same layout as `DiplomatSliceU8`; the distinct C# type keeps
/// the binding's intent (read-only vs writeable) clear at the call site.
#[derive(Template)]
#[template(path = "dotnet/DiplomatSliceMutU8.cs.jinja", escape = "none")]
struct DiplomatSliceMutU8Template<'a> {
    namespace: &'a str,
}

/// `DiplomatSliceU16` — the `repr(C)` fat pointer that crosses the FFI
/// boundary for every `&DiplomatStr16` param. A C# `char` is a UTF-16 code
/// unit, the same width as Rust's `u16`, so a C# `string` pins directly into
/// this shape with no transcoding.
#[derive(Template)]
#[template(path = "dotnet/DiplomatSliceU16.cs.jinja", escape = "none")]
struct DiplomatSliceU16Template<'a> {
    namespace: &'a str,
}

#[derive(Template)]
#[template(path = "dotnet/DiplomatSliceU32.cs.jinja", escape = "none")]
struct DiplomatSliceU32Template<'a> {
    namespace: &'a str,
}

#[derive(Template)]
#[template(path = "dotnet/DiplomatSliceMutU32.cs.jinja", escape = "none")]
struct DiplomatSliceMutU32Template<'a> {
    namespace: &'a str,
}

/// `DiplomatWrite` — caller-provided buffer Rust appends UTF-8 bytes
/// into. Carries function pointers for `flush` and `grow` callbacks so
/// Rust can ask C# to enlarge the buffer when it runs out. Used for
/// every "return string" API on the Rust side (`fn foo(&self, write: &mut DiplomatWrite)`).
#[derive(Template)]
#[template(path = "dotnet/DiplomatWrite.cs.jinja", escape = "none")]
struct DiplomatWriteTemplate<'a> {
    namespace: &'a str,
}

/// `DiplomatNativeLib` — the single shared `[DllImport]` library-name
/// constant referenced by every raw extern. Emitted once so the
/// iOS-vs-other `#if __IOS__` block isn't duplicated per type.
#[derive(Template)]
#[template(path = "dotnet/NativeLib.cs.jinja", escape = "none")]
struct NativeLibTemplate<'a> {
    namespace: &'a str,
    dylib_name: &'a str,
}

/// `RustHandle<T>` — a pointer that carries its own free decision (owned
/// runs the destructor, borrowed doesn't), so a borrow-returning wrapper
/// doesn't need an ownership flag field.
#[derive(Template)]
#[template(path = "dotnet/RustHandle.cs.jinja", escape = "none")]
struct RustHandleTemplate<'a> {
    namespace: &'a str,
}

#[derive(Template)]
#[template(path = "dotnet/BorrowLease.cs.jinja", escape = "none")]
struct BorrowLeaseTemplate<'a> {
    namespace: &'a str,
}

/// `DiplomatBool` — the blittable one-byte `bool` stand-in for Result/Option
/// tags and struct bool fields (the template explains why).
#[derive(Template)]
#[template(path = "dotnet/DiplomatBool.cs.jinja", escape = "none")]
struct DiplomatBoolTemplate<'a> {
    namespace: &'a str,
}

/// `DiplomatBorrowedSpan<T>` — a zero-copy view over a borrowed slice/string
/// return (Rust still owns the memory). Needs the `System.Memory` package on
/// the netstandard2.0 / .NET Framework floor (`ReadOnlySpan<T>`), so it's
/// emitted only when a run actually returns one (see `uses_borrowed_span`).
#[derive(Template)]
#[template(path = "dotnet/DiplomatBorrowedSpan.cs.jinja", escape = "none")]
struct DiplomatBorrowedSpanTemplate<'a> {
    namespace: &'a str,
}

/// `Diplomat.Utf8` — explicit, named UTF-16 `string` -> UTF-8 `byte[]`
/// clone, used wherever a validated `&str` parameter forces a transcode
/// that can't be avoided (see `gen::method`'s `Slice::Str` lowering).
#[derive(Template)]
#[template(path = "dotnet/Utf8.cs.jinja", escape = "none")]
struct Utf8Template<'a> {
    namespace: &'a str,
}

/// `DiplomatPinnedMemory` — pins a caller `ReadOnlyMemory` buffer while a
/// Rust value borrows it, and unpins when the borrowing wrapper is disposed.
#[derive(Template)]
#[template(path = "dotnet/DiplomatPinnedMemory.cs.jinja", escape = "none")]
struct DiplomatPinnedMemoryTemplate<'a> {
    namespace: &'a str,
}

/// `DiplomatOwnedSliceU8` — the `repr(C)` `(ptr, len)` pair an owned
/// `Box<[u8]>` return crosses the FFI boundary as, by value. Same layout as
/// `DiplomatSliceU8`; kept as a distinct type so the raw layer still shows
/// which structs are owned-returns vs. borrowed-params.
#[derive(Template)]
#[template(path = "dotnet/DiplomatOwnedSliceU8.cs.jinja", escape = "none")]
struct DiplomatOwnedSliceU8Template<'a> {
    namespace: &'a str,
}

#[derive(Template)]
#[template(path = "dotnet/RawRustVec.cs.jinja", escape = "none")]
struct RawRustVecTemplate<'a> {
    namespace: &'a str,
}

/// `RustVec` — GC-owned wrapper over an owned `Box<[u8]>` Rust handed back
/// across FFI. It allows scoped zero-copy access and explicit cloning.
#[derive(Template)]
#[template(path = "dotnet/RustVec.cs.jinja", escape = "none")]
struct RustVecTemplate<'a> {
    namespace: &'a str,
}

pub(crate) fn attr_support() -> BackendAttrSupport {
    let mut a = BackendAttrSupport::default();

    // Conservative defaults — flip to `true` as features land in `gen`.
    //
    // `namespacing = false` because every generated file lands in the
    // single configured root namespace (`{namespace}.Raw` for the raw
    // layer, `{namespace}` for the idiomatic layer). HIR
    // `#[diplomat::attr(*, namespace = "...")]` annotations would flatten
    // silently otherwise, producing wrong APIs / type collisions.
    a.namespacing = false;
    a.memory_sharing = false;
    a.non_exhaustive_structs = true;
    a.method_overloading = true;
    a.utf8_strings = true;
    a.utf16_strings = true;
    // `option`, `mutable_slices`, and `utf16_strings` are advertised but
    // coverage is narrower than the flag suggests:
    //   - `mutable_slices`: only `&mut [DiplomatByte]`, `&mut [u8]`, and
    //     `&mut [u32]` lower today; other primitive element types report a
    //     diagnostic in the slice-primitive arm of `gen::method::lower_input`.
    //   - `option`: works for primitive / enum / struct success values;
    //     unsupported non-primitive struct fields report a diagnostic during
    //     struct codegen.
    //   - `utf16_strings`: `&DiplomatStr16` inputs are zero-copy (call-scoped
    //     and borrowed-by-return); borrowed string *returns* (`&'a str` /
    //     `&'a DiplomatStr` / `&'a DiplomatStr16`) work bare when they borrow
    //     an opaque owner, but not when they borrow a slice/string parameter
    //     (no Dispose path to unpin) and not wrapped in `Result`/`Option`
    //     yet; `'static` string/slice returns are rejected; owned string
    //     returns (`Box<str>`) aren't supported (see the separately-decided
    //     owned-slice-return design in DECISIONS.md); `&[&str]`
    //     (`Slice::Strs`) is rejected regardless of encoding.
    // The granularity needed to express this in `attr_support` doesn't
    // exist (no per-primitive flag), so we keep the broad flags `true`
    // and document the gaps here + via the diagnostics themselves.
    a.option = true;
    a.mutable_slices = true;
    // `static_slices` and `owned_slices` would advertise support for
    // `&'static [T]` / `Box<[T]>` style inputs and outputs. The backend
    // reports diagnostics on those paths today (see `method.rs` slice arms),
    // so we tell the HIR validator to reject them at lowering.
    a.static_slices = false;
    a.owned_slices = false;
    // Only the return position: `Box<[u8]>` returns lower to a zero-copy
    // `RustVec` (see `gen::method::lower_return`). Owned slice *parameters*
    // stay rejected via `owned_slices` above — this is a separate flag on
    // purpose, so input and return position can be toggled independently.
    a.owned_byte_slice_returns = true;

    a.constructors = false;
    a.named_constructors = false;
    a.fallible_constructors = false;
    // Getters and setters become C# properties. Static ones stay off, matching
    // the Dart and JS backends.
    a.accessors = true;
    a.static_accessors = false;
    a.stringifiers = false;
    a.comparators = false;
    a.iterators = false;
    a.iterables = false;
    a.indexing = false;
    a.callbacks = false;
    a.traits = false;
    a.custom_errors = false;
    a.traits_are_send = false;
    a.traits_are_sync = false;
    a.generate_mocking_interface = false;
    a.manually_disposable = true;

    a
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DotnetConfig {
    /// Root .NET namespace for the generated bindings (e.g. `Icu4x`).
    pub namespace: Option<String>,
    /// The native library name passed to `LibraryImport`. Defaults to the
    /// crate's `lib_name`.
    pub dylib_name: Option<String>,
    /// Suffix trimmed when generating exception names from error types,
    /// e.g. trimming `Error` so `FooError` -> `FooException`.
    pub exception_trim_suffix: Option<String>,
    /// Error method used for exception messages, e.g. `ToDisplay`.
    pub exception_message_method: Option<String>,
    /// If `true`, emit a `.csproj` scaffold next to the generated sources.
    pub scaffold: Option<bool>,
}

impl DotnetConfig {
    pub fn set(&mut self, key: &str, value: toml::Value) {
        match key {
            "namespace" if value.is_str() => {
                self.namespace = value.as_str().map(str::to_string);
            }
            "dylib_name" | "native_lib" if value.is_str() => {
                self.dylib_name = value.as_str().map(str::to_string);
            }
            "exception_trim_suffix" | "exceptions.trim_suffix" if value.is_str() => {
                self.exception_trim_suffix = value.as_str().map(str::to_string);
            }
            "exception_message_method" | "exceptions.error_message_method" if value.is_str() => {
                self.exception_message_method = value.as_str().map(str::to_string);
            }
            "scaffold" => {
                self.scaffold = value
                    .as_bool()
                    .or_else(|| value.as_str().map(|v| v == "true"));
            }
            _ => {}
        }
    }
}

/// Normalize a rendered template into the on-disk shape we check in:
/// LF-only line endings (templates may be checked out as CRLF on
/// Windows via `core.autocrlf`), no trailing whitespace per line
/// (Jinja indent-then-include patterns leave stray spaces on
/// otherwise-blank lines). Preserves a trailing newline if the
/// rendered string had one. Keeps `git diff --check` clean across
/// platforms.
fn normalize_output(rendered: String) -> String {
    let trailing_newline = rendered.ends_with('\n');
    let mut out: String = rendered
        .split('\n')
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n");
    if !trailing_newline && out.ends_with('\n') {
        out.pop();
    }
    out
}

fn add_cs_file(files: &FileMap, name: String, rendered: String) {
    files.add_file(name, normalize_output(rendered));
}

pub(crate) fn run<'tcx>(
    tcx: &'tcx TypeContext,
    config: &'tcx Config,
    docs_url_gen: &'tcx DocsUrlGenerator,
) -> (FileMap, ErrorStore<'tcx, String>) {
    let files = FileMap::default();
    let errors: ErrorStore<'tcx, String> = ErrorStore::default();
    let formatter = DotnetFormatter::new(tcx, config, docs_url_gen);

    let lib_name = config
        .shared_config
        .lib_name
        .clone()
        .or_else(|| config.dotnet_config.dylib_name.clone())
        .expect("Missing required field `lib_name` in [shared] or `native_lib`/`dylib_name` in .NET config");

    let dylib_name = config
        .dotnet_config
        .dylib_name
        .clone()
        .unwrap_or_else(|| lib_name.clone());

    let namespace = config
        .dotnet_config
        .namespace
        .clone()
        .unwrap_or_else(|| lib_name.to_upper_camel_case());

    let ctx = gen::ItemGenContext {
        tcx,
        formatter: &formatter,
        errors: &errors,
        docs_url_gen,
        lib_name: &lib_name,
        dylib_name: &dylib_name,
        namespace: &namespace,
        exception_trim_suffix: config.dotnet_config.exception_trim_suffix.as_deref(),
        exception_message_method: config.dotnet_config.exception_message_method.as_deref(),
        result_struct_registry: std::cell::RefCell::new(std::collections::HashMap::new()),
        option_struct_registry: std::cell::RefCell::new(std::collections::HashMap::new()),
        callback_struct_registry: std::cell::RefCell::new(std::collections::HashMap::new()),
    };

    /*
     * Raw represents the layer of C# that directly manipulates the C ABI. It is expected to be unsafe and low-level, and is not intended for direct consumption by end-users.
     * The content layer represents the safe, idiomatic C# API that end-users will interact with.
     * It may wrap or compose multiple raw items, and should prioritize usability and safety.
     */
    let (uses_pinned_memory, uses_owned_byte_slice_return, uses_borrowed_span, rendered_types) =
        ctx.render_all_types();
    for rendered in rendered_types {
        let file_name = format!("{}.cs", rendered.display_name);
        if let Some(raw) = rendered.raw {
            add_cs_file(&files, format!("Raw{}.cs", rendered.display_name), raw);
        }
        add_cs_file(&files, file_name, rendered.content);
    }

    // Emit result structs + their exception classes. One exception per
    // unique error type, dedup'd via a HashSet on the way through. The
    // key uses `dedup_key()` (variant-tag prefixed) rather than the
    // bare display name, so an opaque `Foo` and a struct `Foo` are not
    // collapsed into one exception class.
    let mut emitted_exceptions: std::collections::HashSet<String> =
        std::collections::HashSet::new();
    for result_struct in ctx.result_struct_registry.into_inner().into_values() {
        let error_key = result_struct.error.dedup_key();
        // Skip exception emission for the unit-error case: it maps to the
        // built-in BCL `InvalidOperationException`, no per-method class.
        if !result_struct.error.is_unit() && emitted_exceptions.insert(error_key) {
            let exception = gen::fillable::DotnetException {
                namespace: namespace.clone(),
                error: result_struct.error.clone(),
                exception_name: result_struct.exception_name.clone(),
                message_method: config.dotnet_config.exception_message_method.clone(),
            };
            add_cs_file(
                &files,
                format!("{}.cs", result_struct.exception_name),
                exception
                    .render()
                    .expect("DotnetException template render failed"),
            );
        }

        let file_name = format!("{}.cs", result_struct.result_struct_name);
        add_cs_file(&files, file_name, result_struct.render().unwrap());
    }

    // Emit option structs — one per unique inner type encountered in any
    // Option<value-type> return. Pointer-nullable Options (Option<Box<T>>)
    // don't register anything; the inner opaque pointer carries null
    // natively and needs no wrapper.
    for option_struct in ctx.option_struct_registry.into_inner().into_values() {
        let file_name = format!("{}.cs", option_struct.option_struct_name);
        add_cs_file(
            &files,
            file_name,
            option_struct
                .render()
                .expect("DotnetOption template render failed"),
        );
    }

    // Runtime helpers — emit once, independent of which types exist.
    add_cs_file(
        &files,
        "DiplomatSliceU8.cs".to_string(),
        DiplomatSliceU8Template {
            namespace: &namespace,
        }
        .render()
        .expect("DiplomatSliceU8 template render failed"),
    );
    add_cs_file(
        &files,
        "DiplomatSliceMutU8.cs".to_string(),
        DiplomatSliceMutU8Template {
            namespace: &namespace,
        }
        .render()
        .expect("DiplomatSliceMutU8 template render failed"),
    );
    add_cs_file(
        &files,
        "DiplomatSliceU16.cs".to_string(),
        DiplomatSliceU16Template {
            namespace: &namespace,
        }
        .render()
        .expect("DiplomatSliceU16 template render failed"),
    );
    add_cs_file(
        &files,
        "DiplomatSliceU32.cs".to_string(),
        DiplomatSliceU32Template {
            namespace: &namespace,
        }
        .render()
        .expect("DiplomatSliceU32 template render failed"),
    );
    add_cs_file(
        &files,
        "DiplomatSliceMutU32.cs".to_string(),
        DiplomatSliceMutU32Template {
            namespace: &namespace,
        }
        .render()
        .expect("DiplomatSliceMutU32 template render failed"),
    );
    add_cs_file(
        &files,
        "DiplomatWrite.cs".to_string(),
        DiplomatWriteTemplate {
            namespace: &namespace,
        }
        .render()
        .expect("DiplomatWrite template render failed"),
    );
    add_cs_file(
        &files,
        "NativeLib.cs".to_string(),
        NativeLibTemplate {
            namespace: &namespace,
            dylib_name: &dylib_name,
        }
        .render()
        .expect("NativeLib template render failed"),
    );
    add_cs_file(
        &files,
        "BorrowLease.cs".to_string(),
        BorrowLeaseTemplate {
            namespace: &namespace,
        }
        .render()
        .expect("BorrowLease template render failed"),
    );
    add_cs_file(
        &files,
        "RustHandle.cs".to_string(),
        RustHandleTemplate {
            namespace: &namespace,
        }
        .render()
        .expect("RustHandle template render failed"),
    );
    add_cs_file(
        &files,
        "DiplomatBool.cs".to_string(),
        DiplomatBoolTemplate {
            namespace: &namespace,
        }
        .render()
        .expect("DiplomatBool template render failed"),
    );
    add_cs_file(
        &files,
        "Utf8.cs".to_string(),
        Utf8Template {
            namespace: &namespace,
        }
        .render()
        .expect("Utf8 template render failed"),
    );
    // The helper pulls in System.Memory, which the netstandard2.0 floor lacks
    // by default — so only ship it when the run actually pins a slice.
    if uses_pinned_memory {
        add_cs_file(
            &files,
            "DiplomatPinnedMemory.cs".to_string(),
            DiplomatPinnedMemoryTemplate {
                namespace: &namespace,
            }
            .render()
            .expect("DiplomatPinnedMemory template render failed"),
        );
    }
    // Also needs System.Memory (`ReadOnlySpan<T>`) — same reasoning, gated
    // independently since a method can return a borrowed span without
    // pinning any input (e.g. borrowing only from `self`).
    if uses_borrowed_span {
        add_cs_file(
            &files,
            "DiplomatBorrowedSpan.cs".to_string(),
            DiplomatBorrowedSpanTemplate {
                namespace: &namespace,
            }
            .render()
            .expect("DiplomatBorrowedSpan template render failed"),
        );
    }

    // The helper owns native memory, so only ship it when a method returns an
    // owned `Box<[u8]>`.
    if uses_owned_byte_slice_return {
        add_cs_file(
            &files,
            "DiplomatOwnedSliceU8.cs".to_string(),
            DiplomatOwnedSliceU8Template {
                namespace: &namespace,
            }
            .render()
            .expect("DiplomatOwnedSliceU8 template render failed"),
        );
        add_cs_file(
            &files,
            "RawRustVec.cs".to_string(),
            RawRustVecTemplate {
                namespace: &namespace,
            }
            .render()
            .expect("RawRustVec template render failed"),
        );
        add_cs_file(
            &files,
            "RustVec.cs".to_string(),
            RustVecTemplate {
                namespace: &namespace,
            }
            .render()
            .expect("RustVec template render failed"),
        );
    }

    (files, errors)
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use diplomat_core::hir::{BasicAttributeValidator, DocsUrlGenerator, TypeContext};
    use quote::quote;

    use crate::Config;

    fn new_tcx(tk_stream: proc_macro2::TokenStream) -> TypeContext {
        let file = syn::parse2::<syn::File>(tk_stream).expect("failed to parse test module");

        let mut attr_validator = BasicAttributeValidator::new("dotnet");
        attr_validator.support = super::attr_support();

        match TypeContext::from_syn(
            &file,
            Default::default(),
            attr_validator,
            None,
            &diplomat_core::ast::SpanLocation::None,
        ) {
            Ok(context) => context,
            Err(e) => {
                for err in e {
                    eprintln!("{err}");
                }
                panic!("Failed to create context")
            }
        }
    }

    /// For shapes rejected before a `TypeContext` even exists (HIR-lowering-time
    /// errors, e.g. an owned slice used in parameter/field position) — `new_tcx`
    /// panics on these, since every other test here expects a valid context.
    fn lowering_errors(
        tk_stream: proc_macro2::TokenStream,
        owned_byte_slice_returns: bool,
    ) -> Vec<String> {
        let file = syn::parse2::<syn::File>(tk_stream).expect("failed to parse test module");

        let mut attr_validator = BasicAttributeValidator::new("dotnet");
        attr_validator.support = super::attr_support();
        attr_validator.support.owned_byte_slice_returns = owned_byte_slice_returns;

        match TypeContext::from_syn(
            &file,
            Default::default(),
            attr_validator,
            None,
            &diplomat_core::ast::SpanLocation::None,
        ) {
            Ok(_) => Vec::new(),
            Err(e) => e.into_iter().map(|err| err.to_string()).collect(),
        }
    }

    // Property accessors and the internal `Handle` getter share one indentation,
    // so count getters without the handle's.
    fn property_getter_count(config: &str) -> usize {
        config.matches("        get").count() - config.matches("> Handle").count()
    }

    fn run_dotnet(tk_stream: proc_macro2::TokenStream) -> (HashMap<String, String>, Vec<String>) {
        let tcx = new_tcx(tk_stream);
        let mut config = Config::default();
        config.shared_config.lib_name = Some("somelib".to_string());
        let docs_url_gen = DocsUrlGenerator::with_base_urls(None, HashMap::new());

        let (files, errors) = super::run(&tcx, &config, &docs_url_gen);
        let errors = errors
            .take_all()
            .into_iter()
            .map(|e| format!("{}: {}", e.0, e.1))
            .collect();
        (files.take_files(), errors)
    }

    #[test]
    fn retained_borrows_from_manually_disposable_sources_are_rejected() {
        let cases = vec![
            (
                "borrowed receiver view",
                quote! {
                    #[diplomat::bridge]
                    mod ffi {
                        #[diplomat::opaque]
                        #[diplomat::attr(dotnet, manually_disposable)]
                        pub struct Source;

                        impl Source {
                            pub fn view<'a>(&'a self) -> &'a Self {
                                unimplemented!()
                            }
                        }
                    }
                },
                "source `this` of type `Source`",
            ),
            (
                "borrowed mutable receiver view",
                quote! {
                    #[diplomat::bridge]
                    mod ffi {
                        #[diplomat::opaque_mut]
                        #[diplomat::attr(dotnet, manually_disposable)]
                        pub struct Source;

                        impl Source {
                            pub fn view_mut<'a>(&'a mut self) -> &'a mut Self {
                                unimplemented!()
                            }
                        }
                    }
                },
                "source `this` of type `Source`",
            ),
            (
                "borrowed getter from receiver",
                quote! {
                    #[diplomat::bridge]
                    mod ffi {
                        #[diplomat::opaque]
                        #[diplomat::attr(dotnet, manually_disposable)]
                        pub struct Source;

                        impl Source {
                            #[diplomat::attr(auto, getter)]
                            pub fn view<'a>(&'a self) -> &'a Self {
                                unimplemented!()
                            }
                        }
                    }
                },
                "source `this` of type `Source`",
            ),
            (
                "optional named source and owned optional child",
                quote! {
                    #[diplomat::bridge]
                    mod ffi {
                        #[diplomat::opaque]
                        #[diplomat::attr(dotnet, manually_disposable)]
                        pub struct Source;

                        #[diplomat::opaque]
                        pub struct Child<'a>(&'a Source);

                        #[diplomat::opaque]
                        pub struct Factory;

                        impl Factory {
                            pub fn make<'a>(source: Option<&'a Source>) -> Option<Box<Child<'a>>> {
                                unimplemented!()
                            }
                        }
                    }
                },
                "source `source` of type `Source`",
            ),
            (
                "keyword named source with a lifetime bound",
                quote! {
                    #[diplomat::bridge]
                    mod ffi {
                        #[diplomat::opaque]
                        #[diplomat::attr(dotnet, manually_disposable)]
                        pub struct Source;

                        #[diplomat::opaque]
                        pub struct Other;

                        #[diplomat::opaque]
                        pub struct Child<'a>(&'a Source);

                        #[diplomat::opaque]
                        pub struct Factory;

                        impl Factory {
                            pub fn make<'short, 'long: 'short>(
                                string: &'long Source,
                                other: &'short Other,
                            ) -> Box<Child<'short>> {
                                let _ = other;
                                unimplemented!()
                            }
                        }
                    }
                },
                "source `@string` of type `Source`",
            ),
            (
                "borrowed slice from receiver",
                quote! {
                    #[diplomat::bridge]
                    mod ffi {
                        #[diplomat::opaque]
                        #[diplomat::attr(dotnet, manually_disposable)]
                        pub struct Source;

                        impl Source {
                            pub fn bytes<'a>(&'a self) -> &'a [u8] {
                                unimplemented!()
                            }
                        }
                    }
                },
                "source `this` of type `Source`",
            ),
            (
                "owned borrowing error",
                quote! {
                    #[diplomat::bridge]
                    mod ffi {
                        #[diplomat::opaque]
                        #[diplomat::attr(dotnet, manually_disposable)]
                        pub struct Source;

                        #[diplomat::opaque]
                        pub struct BorrowingError<'a>(&'a Source);

                        #[diplomat::opaque]
                        pub struct Factory;

                        impl Factory {
                            pub fn check<'a>(source: &'a Source) -> Result<(), Box<BorrowingError<'a>>> {
                                unimplemented!()
                            }
                        }
                    }
                },
                "source `source` of type `Source`",
            ),
            (
                "owned borrowing success",
                quote! {
                    #[diplomat::bridge]
                    mod ffi {
                        #[diplomat::opaque]
                        #[diplomat::attr(dotnet, manually_disposable)]
                        pub struct Source;

                        #[diplomat::opaque]
                        pub struct Child<'a>(&'a Source);

                        #[diplomat::opaque]
                        pub struct Error;

                        #[diplomat::opaque]
                        pub struct Factory;

                        impl Factory {
                            pub fn check<'a>(
                                source: &'a Source,
                            ) -> Result<Box<Child<'a>>, Box<Error>> {
                                unimplemented!()
                            }
                        }
                    }
                },
                "source `source` of type `Source`",
            ),
        ];

        for (case, tk_stream, expected) in cases {
            let (_files, errors) = run_dotnet(tk_stream);
            assert_eq!(errors.len(), 1, "{case}: {errors:?}");
            assert!(
                errors[0].contains("retains a borrow")
                    && errors[0].contains("return an independent value")
                    && errors[0].contains(expected),
                "{case}: unexpected diagnostics: {}",
                errors.join("\n")
            );
        }
    }

    #[test]
    fn multiple_manually_disposable_sources_are_each_reported() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                #[diplomat::attr(dotnet, manually_disposable)]
                pub struct Source;

                #[diplomat::opaque]
                #[diplomat::attr(dotnet, manually_disposable)]
                pub struct Other;

                #[diplomat::opaque]
                pub struct Child<'a>(&'a Source, &'a Other);

                #[diplomat::opaque]
                pub struct Factory;

                impl Factory {
                    pub fn make<'a>(first: &'a Source, second: &'a Other) -> Box<Child<'a>> {
                        let _ = (first, second);
                        unimplemented!()
                    }
                }
            }
        };

        let (_files, errors) = run_dotnet(tk_stream);
        assert_eq!(errors.len(), 2, "unexpected diagnostics: {errors:?}");
        assert!(errors
            .iter()
            .any(|error| { error.contains("source `first` of type `Source`") }));
        assert!(errors
            .iter()
            .any(|error| { error.contains("source `second` of type `Other`") }));
    }

    #[test]
    fn temporary_and_independent_borrows_from_manually_disposable_sources_are_allowed() {
        let cases = vec![
            (
                "shared receiver",
                quote! {
                    #[diplomat::bridge]
                    mod ffi {
                        #[diplomat::opaque]
                        #[diplomat::attr(dotnet, manually_disposable)]
                        pub struct Source;

                        impl Source {
                            pub fn id(&self) -> u64 {
                                0
                            }
                        }
                    }
                },
                "Source.cs",
                "Id(",
            ),
            (
                "named shared parameter",
                quote! {
                    #[diplomat::bridge]
                    mod ffi {
                        #[diplomat::opaque]
                        #[diplomat::attr(dotnet, manually_disposable)]
                        pub struct Source;

                        #[diplomat::opaque]
                        pub struct Factory;

                        impl Factory {
                            pub fn id(source: &Source) -> u64 {
                                let _ = source;
                                0
                            }
                        }
                    }
                },
                "Factory.cs",
                "Id(",
            ),
            (
                "mutable receiver",
                quote! {
                    #[diplomat::bridge]
                    mod ffi {
                        #[diplomat::opaque_mut]
                        #[diplomat::attr(dotnet, manually_disposable)]
                        pub struct Source;

                        impl Source {
                            pub fn id_mut(&mut self) -> u64 {
                                0
                            }
                        }
                    }
                },
                "Source.cs",
                "IdMut(",
            ),
            (
                "temporary mutable parameter",
                quote! {
                    #[diplomat::bridge]
                    mod ffi {
                        #[diplomat::opaque_mut]
                        #[diplomat::attr(dotnet, manually_disposable)]
                        pub struct Source;

                        #[diplomat::opaque]
                        pub struct Factory;

                        impl Factory {
                            pub fn id_mut(source: &mut Source) -> u64 {
                                let _ = source;
                                0
                            }
                        }
                    }
                },
                "Factory.cs",
                "IdMut(",
            ),
            (
                "independent owned return",
                quote! {
                    #[diplomat::bridge]
                    mod ffi {
                        #[diplomat::opaque]
                        #[diplomat::attr(dotnet, manually_disposable)]
                        pub struct Source;

                        #[diplomat::opaque]
                        pub struct Independent;

                        impl Source {
                            pub fn independent(&self) -> Box<Independent> {
                                unimplemented!()
                            }
                        }
                    }
                },
                "Source.cs",
                "Independent(",
            ),
            (
                "borrow from independent source",
                quote! {
                    #[diplomat::bridge]
                    mod ffi {
                        #[diplomat::opaque]
                        pub struct Source;

                        #[diplomat::opaque]
                        #[diplomat::attr(dotnet, manually_disposable)]
                        pub struct Child<'a>(&'a Source);

                        impl Source {
                            pub fn child<'a>(&'a self) -> Box<Child<'a>> {
                                unimplemented!()
                            }
                        }
                    }
                },
                "Source.cs",
                "Child(",
            ),
            (
                "factory borrow from independent source",
                quote! {
                    #[diplomat::bridge]
                    mod ffi {
                        #[diplomat::opaque]
                        #[diplomat::attr(dotnet, manually_disposable)]
                        pub struct Factory;

                        #[diplomat::opaque]
                        pub struct Source;

                        #[diplomat::opaque]
                        pub struct Child<'a>(&'a Source);

                        impl Factory {
                            pub fn child<'a>(&self, source: &'a Source) -> Box<Child<'a>> {
                                let _ = source;
                                unimplemented!()
                            }
                        }
                    }
                },
                "Factory.cs",
                "Child(",
            ),
        ];

        for (case, tk_stream, output_file, method) in cases {
            let (files, errors) = run_dotnet(tk_stream);
            assert!(errors.is_empty(), "case {case}: {}", errors.join("\n"));
            let output = files
                .get(output_file)
                .unwrap_or_else(|| panic!("case {case}: expected {output_file} output"));
            assert!(
                output.contains(method),
                "case {case}: expected generated method {method} in {output_file}:\n{output}"
            );
        }
    }

    #[test]
    fn native_lib_and_dylib_name_config_aliases_are_supported() {
        let mut native_lib_config = super::DotnetConfig::default();
        native_lib_config.set(
            "native_lib",
            toml::Value::String("diplomat_example".to_string()),
        );
        assert_eq!(
            native_lib_config.dylib_name.as_deref(),
            Some("diplomat_example")
        );

        let mut dylib_name_config = super::DotnetConfig::default();
        dylib_name_config.set(
            "dylib_name",
            toml::Value::String("diplomat_example".to_string()),
        );
        assert_eq!(
            dylib_name_config.dylib_name.as_deref(),
            Some("diplomat_example")
        );
    }

    #[test]
    fn borrowed_opaque_error_is_rejected() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct ResultOpaque(i32);

                impl ResultOpaque {
                    pub fn borrowed_error<'a>(&'a self) -> Result<(), &'a Self> {
                        unimplemented!()
                    }
                }
            }
        };

        let (_files, errors) = run_dotnet(tk_stream);
        assert_eq!(errors.len(), 1);
        let error_str = errors.join("\n");
        assert!(
            errors[0].contains("borrowed opaque error"),
            "unexpected diagnostics: {error_str}"
        );
    }

    #[test]
    fn borrowing_struct_error_is_rejected() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Owner;

                pub struct BorrowingStructError<'a> {
                    owner: &'a Owner,
                }

                impl Owner {
                    pub fn borrowed_struct_error<'a>(
                        &'a self,
                    ) -> Result<i32, BorrowingStructError<'a>> {
                        unimplemented!()
                    }
                }
            }
        };

        let (_files, errors) = run_dotnet(tk_stream);
        let error_str = errors.join("\n");
        assert!(
            error_str.contains("error value of type `BorrowingStructError` borrows"),
            "unexpected diagnostics: {error_str}"
        );
    }

    #[test]
    fn borrowed_opaque_return_generates_non_owning() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Foo;

                impl Foo {
                    pub fn borrowed_return<'a>(&'a self) -> &'a Self {
                        unimplemented!()
                    }
                }
            }
        };

        let (files, errors) = run_dotnet(tk_stream);
        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );

        let foo = files.get("Foo.cs").expect("expected Foo.cs output");
        assert!(
            foo.contains("WrapperKind.SharedView"),
            "borrowed return should build a shared non-owning view:\n{foo}"
        );
        assert!(
            foo.contains("RustHandle<Raw.Foo>") && !foo.contains("private void Cleanup()"),
            "a borrowed wrapper should leave finalization to its non-owning handle:\n{foo}"
        );
        assert!(
            !foo.contains("_owned"),
            "the ownership flag field should be gone — ownership lives in the handle:\n{foo}"
        );
    }

    // A borrowed opaque return moves its scoped receiver lease into a managed
    // source-handle edge without adding native ownership.
    #[test]
    fn borrowed_opaque_return_moves_receiver_lease_into_view() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Foo;

                impl Foo {
                    pub fn borrowed_return<'a>(&'a self) -> &'a Self {
                        unimplemented!()
                    }
                }
            }
        };

        let (files, errors) = run_dotnet(tk_stream);
        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );

        let foo = files.get("Foo.cs").expect("expected Foo.cs output");
        assert!(
            foo.contains("new Foo(result, WrapperKind.SharedView")
                && foo.contains("LifetimeEdge.Move(ref selfLease)"),
            "a borrowed opaque return should move its receiver lease into the view:\n{foo}"
        );
        assert!(
            !foo.contains("DiplomatRetainDependency") && !foo.contains("_refCount"),
            "the generated view must not use the removed native retain API:\n{foo}"
        );
    }

    // An owned-but-borrowing return moves its source lease into the owned
    // handle. The source handle remains reachable without adding native
    // ownership to the returned value.
    #[test]
    fn owned_borrowing_return_moves_receiver_lease_into_handle() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Owner;

                #[diplomat::opaque]
                pub struct Dependent<'a>(&'a Owner);

                impl Owner {
                    pub fn make_dependent<'a>(&'a self) -> Box<Dependent<'a>> {
                        unimplemented!()
                    }
                }
            }
        };

        let (files, errors) = run_dotnet(tk_stream);
        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );

        let owner = files.get("Owner.cs").expect("expected Owner.cs output");
        assert!(
            owner.contains("new Dependent(result, LifetimeEdge.Move(ref selfLease))"),
            "an owned-borrowing return should move the receiver lease at construction:\n{owner}"
        );

        let dependent = files
            .get("Dependent.cs")
            .expect("expected Dependent.cs output");
        assert!(
            dependent.contains(
                "internal unsafe Dependent(Raw.Dependent* handle, params ILifetimeEdge?[] edges)"
            ),
            "the owned-borrowing wrapper should accept managed lifetime edges:\n{dependent}"
        );
        assert!(
            dependent.contains("_inner = RustHandle<Raw.Dependent>.Owned(handle, _destroy);")
                && dependent.contains("Owned(handle, _destroy, edges)"),
            "Dependent threads edges into its single native owner:\n{dependent}"
        );
    }

    // The shared runtime owns one pointer and keeps active-operation guards
    // separate from managed source edges.
    #[test]
    fn owned_child_from_shared_borrow_is_a_versioned_read_view() {
        let (files, errors) = run_dotnet(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque_mut]
                pub struct Owner;

                #[diplomat::opaque]
                pub struct Dependent<'a>(&'a Owner);

                impl Owner {
                    pub fn make_dependent<'a>(&'a self) -> Box<Dependent<'a>> {
                        unimplemented!()
                    }

                    pub fn touch(&mut self) {}
                }
            }
        });
        assert!(
            errors.is_empty(),
            "an owned child of a shared borrow needs no attribute: {}",
            errors.join("\n")
        );

        let rust_handle = files
            .get("RustHandle.cs")
            .expect("expected RustHandle.cs output");
        assert!(
            rust_handle.contains("kind == WrapperKind.Owned && lease.Kind == BorrowKind.Shared"),
            "an owned wrapper must turn a shared source lease into a versioned edge:\n{rust_handle}"
        );
    }

    #[test]
    fn exclusive_view_return_requires_manually_disposable() {
        let (_, errors) = run_dotnet(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque_mut]
                pub struct Source(View);

                #[diplomat::opaque_mut]
                pub struct View;

                impl Source {
                    pub fn view_mut<'a>(&'a mut self) -> &'a mut View {
                        unimplemented!()
                    }
                }
            }
        });
        assert!(
            errors.iter().any(|e| {
                e.contains("method `ViewMut`")
                    && e.contains("is `View`")
                    && e.contains("exclusive borrow of `this` of type `Source`")
                    && e.contains("manually_disposable")
            }),
            "an `&mut` view must be manually_disposable: {errors:?}"
        );
    }

    #[test]
    fn manually_disposable_exclusive_view_return_is_accepted() {
        let (files, errors) = run_dotnet(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque_mut]
                pub struct Source(View);

                #[diplomat::opaque_mut]
                #[diplomat::attr(dotnet, manually_disposable)]
                pub struct View;

                impl Source {
                    pub fn view_mut<'a>(&'a mut self) -> &'a mut View {
                        unimplemented!()
                    }
                }
            }
        });
        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );
        let source = files.get("Source.cs").expect("expected Source.cs output");
        assert!(
            source.contains("WrapperKind.ExclusiveView"),
            "the `&mut` view keeps its exclusive borrow until disposed:\n{source}"
        );
    }

    #[test]
    fn owned_child_from_exclusive_receiver_requires_manually_disposable() {
        let (_, errors) = run_dotnet(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque_mut]
                pub struct Source;

                #[diplomat::opaque]
                pub struct Writer<'a>(&'a mut Source);

                impl Source {
                    pub fn writer<'a>(&'a mut self) -> Box<Writer<'a>> {
                        unimplemented!()
                    }
                }
            }
        });
        assert!(
            errors.iter().any(|e| {
                e.contains("method `Writer`")
                    && e.contains("is `Writer`")
                    && e.contains("exclusive borrow of `this` of type `Source`")
            }),
            "an owned child of `&mut self` must be manually_disposable: {errors:?}"
        );
    }

    #[test]
    fn owned_child_from_exclusive_parameter_requires_manually_disposable() {
        let (_, errors) = run_dotnet(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque_mut]
                pub struct Source;

                #[diplomat::opaque]
                pub struct Writer<'a>(&'a mut Source);

                #[diplomat::opaque]
                pub struct Factory;

                impl Factory {
                    pub fn attach<'a>(target: &'a mut Source) -> Box<Writer<'a>> {
                        unimplemented!()
                    }
                }
            }
        });
        assert!(
            errors.iter().any(|e| {
                e.contains("method `Attach`")
                    && e.contains("exclusive borrow of `target` of type `Source`")
            }),
            "an owned child of an `&mut` parameter must be manually_disposable: {errors:?}"
        );
    }

    #[test]
    fn manually_disposable_owned_child_from_exclusive_receiver_is_accepted() {
        let (files, errors) = run_dotnet(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque_mut]
                pub struct Source;

                #[diplomat::opaque]
                #[diplomat::attr(dotnet, manually_disposable)]
                pub struct Writer<'a>(&'a mut Source);

                impl Source {
                    pub fn writer<'a>(&'a mut self) -> Box<Writer<'a>> {
                        unimplemented!()
                    }
                }
            }
        });
        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );
        let source = files.get("Source.cs").expect("expected Source.cs output");
        assert!(
            source.contains("new Writer(result, LifetimeEdge.Move(ref selfLease))"),
            "the exclusive child takes over the receiver lease:\n{source}"
        );
    }

    #[test]
    fn generated_runtime_has_one_owner_and_no_native_retain_counter() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Owner;

                #[diplomat::opaque]
                pub struct Dependent<'a>(&'a Owner);

                impl Owner {
                    pub fn make_dependent<'a>(&'a self) -> Box<Dependent<'a>> {
                        unimplemented!()
                    }
                }
            }
        };

        let (files, errors) = run_dotnet(tk_stream);
        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );

        let rust_handle = files
            .get("RustHandle.cs")
            .expect("expected RustHandle.cs output");
        let borrow_lease = files
            .get("BorrowLease.cs")
            .expect("expected BorrowLease.cs output");

        assert!(
            rust_handle.contains("class RustHandle<T> : SafeHandle")
                && rust_handle.contains("private LifetimeEdges _edges"),
            "the runtime should own the pointer through one SafeHandle with explicit source edges:\n{rust_handle}"
        );
        assert!(
            rust_handle.contains("DangerousAddRef(ref success)")
                && rust_handle.contains("internal void ExitOperation() => DangerousRelease();")
                && rust_handle.contains("HoldForCall()")
                && borrow_lease.contains("return owner.HoldForCall();")
                && !borrow_lease.contains("return owner.StartCall();"),
            "only the directly invoked handle should take a SafeHandle operation claim:\n\
             RustHandle.cs:\n{rust_handle}\nBorrowLease.cs:\n{borrow_lease}"
        );
        assert!(!rust_handle.contains("_refCount") && !rust_handle.contains("_activeOperations"));

        // No opaque wrapper should do its own separate pin-disposal — that
        // responsibility lives entirely in RustHandle.
        for (name, contents) in &files {
            if name == "RustHandle.cs" || name == "DiplomatPinnedMemory.cs" {
                continue;
            }
            assert!(
                !contents.contains("as DiplomatPinnedMemory"),
                "generated file {name} should not manually sweep pins — \
                 pin release lives entirely in the shared RustHandle.cs \
                 runtime:\n{contents}"
            );
        }
    }

    // `&DiplomatStr` (`UnvalidatedUtf8`) carries no caller-side validity
    // contract, so it's lowered exactly like `&[u8]` (see
    // `lower_immutable_element_slice`) — including this case, which used to
    // be rejected before that reshape: an owned opaque return borrowing a
    // `&DiplomatStr` parameter now pins it and roots the pin as a
    // keep-alive edge, identical to `owned_return_borrowing_byte_slice_unpins_on_dispose`.
    #[test]
    fn owned_return_borrowing_diplomat_str_input_pins_and_unpins_on_dispose() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                use diplomat_runtime::DiplomatStr;

                #[diplomat::opaque]
                pub struct Foo<'a>(&'a DiplomatStr);

                impl<'a> Foo<'a> {
                    pub fn new(x: &'a DiplomatStr) -> Box<Self> {
                        unimplemented!()
                    }
                }
            }
        };

        let (files, errors) = run_dotnet(tk_stream);
        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );

        let foo = files.get("Foo.cs").expect("expected Foo.cs output");
        assert!(
            foo.contains("public static Foo New(ReadOnlyMemory<byte> x)"),
            "borrowed &DiplomatStr param should surface as ReadOnlyMemory<byte>:\n{foo}"
        );
        assert!(
            foo.contains("new Foo(result, xPin)"),
            "infallible owned return should root the pin holder as an edge:\n{foo}"
        );
        assert!(
            foo.contains("_inner = RustHandle<Raw.Foo>.Owned(handle, _destroy);")
                && foo.contains("Owned(handle, _destroy, edges)"),
            "Foo threads edges into RustHandle.Owned like any other \
             opaque; its own pin is unpinned right after its own Rust \
             destructor runs in Cleanup():\n{foo}"
        );
        assert!(
            !foo.contains("as DiplomatPinnedMemory"),
            "Cleanup() should not manually sweep pins itself — that lives \
             entirely in the shared RustHandle.cs runtime:\n{foo}"
        );

        // The actual destructor-then-unpin ordering is enforced once, in the
        // shared runtime, not per generated type — see
        // `generated_rc_runtime_unpins_only_after_its_own_destructor_runs`.
    }

    // A validated `&'a str` still forces a transcode-copy (`Utf8.Clone`),
    // so it can't be pinned past the call — this borrow position stays
    // rejected, unlike the `&DiplomatStr` case above.
    #[test]
    fn lifetime_carrying_owned_return_borrowing_validated_str_input_is_rejected() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Foo<'a>(&'a str);

                impl<'a> Foo<'a> {
                    pub fn new(x: &'a str) -> Box<Self> {
                        unimplemented!()
                    }
                }
            }
        };

        let (_files, errors) = run_dotnet(tk_stream);
        assert_eq!(errors.len(), 1);
        let error_str = errors.join("\n");
        assert!(
            errors[0].contains("return value borrows from slice/string parameter"),
            "unexpected diagnostics: {error_str}"
        );
    }

    #[test]
    fn lifetime_carrying_owned_return_borrowing_opaque_gets_warning() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Parent;

                #[diplomat::opaque]
                pub struct Child<'a>(&'a Parent);

                impl Parent {
                    pub fn child<'a>(&'a self) -> Box<Child<'a>> {
                        unimplemented!()
                    }
                }

                #[diplomat::opaque]
                pub struct OwnedFoo;

                impl OwnedFoo {
                    pub fn new() -> Box<Self> {
                        unimplemented!()
                    }
                }
            }
        };

        let (files, errors) = run_dotnet(tk_stream);
        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );

        let parent = files.get("Parent.cs").expect("expected Parent.cs output");
        assert!(
            parent.contains("Lifetime: the returned native-backed value may borrow"),
            "expected lifetime warning in Parent.cs:\n{parent}"
        );

        let owned_foo = files
            .get("OwnedFoo.cs")
            .expect("expected OwnedFoo.cs output");
        assert!(
            !owned_foo.contains("Lifetime: the returned native-backed value may borrow"),
            "unexpected lifetime warning in OwnedFoo.cs:\n{owned_foo}"
        );
    }

    // An owned opaque return borrowing `&[u8]` must pin the input for the
    // wrapper's whole lifetime: ReadOnlyMemory -> pinned holder -> edge.
    #[test]
    fn fallible_owned_return_borrowing_byte_slice_pins_input() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Parsed<'a>(&'a [u8]);

                #[diplomat::opaque]
                pub struct ParseError;

                impl<'a> Parsed<'a> {
                    pub fn parse(data: &'a [u8]) -> Result<Box<Parsed<'a>>, Box<ParseError>> {
                        unimplemented!()
                    }
                }
            }
        };

        let (files, errors) = run_dotnet(tk_stream);
        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );

        let list = files.get("Parsed.cs").expect("expected Parsed.cs output");
        assert!(
            list.contains("public static Parsed Parse(ReadOnlyMemory<byte> data)"),
            "borrowed slice param should surface as ReadOnlyMemory<byte>:\n{list}"
        );
        assert!(
            list.contains("DiplomatPinnedMemory? dataPin = null;")
                && list.contains("dataPin = DiplomatPinnedMemory.Pin(data);"),
            "borrowed slice should be pinned into a holder before the raw call:\n{list}"
        );
        assert!(
            list.contains("Ptr = (byte*)dataPin.Pointer"),
            "raw call should pass the pinned pointer:\n{list}"
        );
        assert!(
            list.contains("new Parsed(result.Ok, dataPin)"),
            "the returned wrapper should root the pin holder as an edge:\n{list}"
        );
        assert!(
            list.contains(
                "            catch\n            {\n                dataPin?.Dispose();\n                throw;\n            }"
            ),
            "any exception before the wrapper owns the pin (P/Invoke failure, error-arm throw) \
             must dispose the pin and rethrow:\n{list}"
        );
        assert!(
            files.contains_key("DiplomatPinnedMemory.cs"),
            "the DiplomatPinnedMemory runtime helper should be emitted"
        );
    }

    // The returned handle owns the pin edge and releases it after its native
    // destructor, while source-handle edges remain ordinary managed references.
    #[test]
    fn owned_return_borrowing_byte_slice_unpins_on_dispose() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Foo<'a>(&'a [u8]);

                impl<'a> Foo<'a> {
                    pub fn new(data: &'a [u8]) -> Box<Self> {
                        unimplemented!()
                    }
                }
            }
        };

        let (files, errors) = run_dotnet(tk_stream);
        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );

        let foo = files.get("Foo.cs").expect("expected Foo.cs output");
        assert!(
            foo.contains("new Foo(result, dataPin)"),
            "infallible owned return should root the pin holder as an edge:\n{foo}"
        );
        assert!(
            foo.contains("_inner = RustHandle<Raw.Foo>.Owned(handle, _destroy);")
                && foo.contains("Owned(handle, _destroy, edges)"),
            "Foo threads edges into RustHandle.Owned like any other \
             opaque; its own pin is unpinned right after Cleanup() runs its \
             own Rust destructor:\n{foo}"
        );
        assert!(
            !foo.contains("as DiplomatPinnedMemory"),
            "Cleanup() should not manually sweep pins itself — that lives \
             entirely in the shared RustHandle.cs runtime:\n{foo}"
        );
    }

    // The pin edge lands on the RETURNED type's wrapper, so its constructor
    // must accept edges even on a type with no pinning methods of its own
    // (#1194) — but the actual unpin sweep lives once in the shared
    // RustHandle.cs runtime, not duplicated per opaque.
    #[test]
    fn cross_type_pinned_return_threads_pins_into_rust_handle_state() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Factory;

                #[diplomat::opaque]
                pub struct Product<'a>(&'a [u8]);

                impl Factory {
                    pub fn build<'a>(data: &'a [u8]) -> Box<Product<'a>> {
                        unimplemented!()
                    }
                }
            }
        };

        let (files, errors) = run_dotnet(tk_stream);
        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );

        let product = files.get("Product.cs").expect("expected Product.cs output");
        assert!(
            product.contains("_inner = RustHandle<Raw.Product>.Owned(handle, _destroy);")
                && product.contains("Owned(handle, _destroy, edges)"),
            "a type returned pinned from another type's method must still \
             thread its pin into its own wrapper's `_edges` field, same as \
             any other opaque:\n{product}"
        );
    }

    // Two slice params borrowed by the same output lifetime must each get their
    // own pin local, disposed independently on throw and rooted together.
    #[test]
    fn multiple_pinned_slices_each_get_a_distinct_pin() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Pair<'a>(&'a [u8], &'a [u8]);

                impl<'a> Pair<'a> {
                    pub fn combine(a: &'a [u8], b: &'a [u8]) -> Box<Pair<'a>> {
                        unimplemented!()
                    }
                }
            }
        };

        let (files, errors) = run_dotnet(tk_stream);
        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );

        let pair = files.get("Pair.cs").expect("expected Pair.cs output");
        assert!(
            pair.contains("DiplomatPinnedMemory? aPin = null;")
                && pair.contains("DiplomatPinnedMemory? bPin = null;"),
            "both pins should be declared nullable before the try:\n{pair}"
        );
        assert!(
            pair.contains("aPin = DiplomatPinnedMemory.Pin(a);")
                && pair.contains("bPin = DiplomatPinnedMemory.Pin(b);"),
            "both pins should be assigned inside the try:\n{pair}"
        );
        assert!(
            pair.contains("aPin?.Dispose();") && pair.contains("bPin?.Dispose();"),
            "the catch should dispose both pins independently:\n{pair}"
        );
        assert!(
            pair.contains("new Pair(result, aPin, bPin)"),
            "both distinct pin locals should be rooted on the returned wrapper:\n{pair}"
        );
    }

    // The `&[u32]` element type surfaces as ReadOnlyMemory<uint> with a `uint*`
    // pinned pointer — the whole contract, not just the `&[u8]` case.
    #[test]
    fn pinned_u32_slice_uses_readonly_memory_uint() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct View<'a>(&'a [u32]);

                impl<'a> View<'a> {
                    pub fn parse(data: &'a [u32]) -> Box<View<'a>> {
                        unimplemented!()
                    }
                }
            }
        };

        let (files, errors) = run_dotnet(tk_stream);
        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );

        let view = files.get("View.cs").expect("expected View.cs output");
        assert!(
            view.contains("public static View Parse(ReadOnlyMemory<uint> data)"),
            "a &[u32] borrowed param should surface as ReadOnlyMemory<uint>:\n{view}"
        );
        assert!(
            view.contains("Ptr = (uint*)dataPin.Pointer"),
            "the raw call should pass the pinned pointer as uint*:\n{view}"
        );
    }

    // Struct methods share the pin lowering, so their docs need the same
    // "stays pinned until disposed" remark the opaque template emits.
    #[test]
    fn struct_method_pinned_return_gets_pin_remark() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                pub struct BuilderOptions {
                    pub flag: bool,
                }

                #[diplomat::opaque]
                pub struct Built<'a>(&'a [u8]);

                impl BuilderOptions {
                    pub fn make<'a>(data: &'a [u8]) -> Box<Built<'a>> {
                        unimplemented!()
                    }
                }
            }
        };

        let (files, errors) = run_dotnet(tk_stream);
        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );

        let builder = files
            .get("BuilderOptions.cs")
            .expect("expected BuilderOptions.cs output");
        assert!(
            builder.contains("Warning: this call pins the buffer passed via <c>ReadOnlyMemory</c>"),
            "struct methods with pinned inputs should carry the pin remark:\n{builder}"
        );
    }

    // A slice whose lifetime is NOT used by the output keeps the cheap
    // call-scoped `fixed` pinning — no ReadOnlyMemory, no holder.
    #[test]
    fn temporary_byte_slice_keeps_fixed_pinning() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Hasher;

                impl Hasher {
                    pub fn hash(data: &[u8]) -> u32 {
                        unimplemented!()
                    }
                }
            }
        };

        let (files, errors) = run_dotnet(tk_stream);
        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );

        let hasher = files.get("Hasher.cs").expect("expected Hasher.cs output");
        assert!(
            hasher.contains("public static uint Hash(byte[] data)")
                && hasher.contains("fixed (byte* dataPtr = data)"),
            "temporary slice should keep the byte[] + fixed lowering:\n{hasher}"
        );
        assert!(
            !hasher.contains("DiplomatPinnedMemory.Pin("),
            "temporary slice method should not pin the input:\n{hasher}"
        );
    }

    // `&DiplomatStr` carries no caller-side validity contract, so it's
    // lowered exactly like `&[u8]` — zero-copy `byte[]` + `fixed`, no
    // transcode, ever.
    #[test]
    fn temporary_diplomat_str_keeps_byte_array_fixed_pinning() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                use diplomat_runtime::DiplomatStr;

                #[diplomat::opaque]
                pub struct Hasher;

                impl Hasher {
                    pub fn hash(data: &DiplomatStr) -> u32 {
                        unimplemented!()
                    }
                }
            }
        };

        let (files, errors) = run_dotnet(tk_stream);
        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );

        let hasher = files.get("Hasher.cs").expect("expected Hasher.cs output");
        assert!(
            hasher.contains("public static uint Hash(byte[] data)")
                && hasher.contains("fixed (byte* dataPtr = data)"),
            "&DiplomatStr should get the same byte[] + fixed lowering as &[u8]:\n{hasher}"
        );
        assert!(
            !hasher.contains("Encoding.UTF8") && !hasher.contains("Utf8.Clone"),
            "&DiplomatStr must never transcode — it's already raw bytes:\n{hasher}"
        );
    }

    // A validated `&str` still needs the caller to guarantee well-formed
    // UTF-8, so the idiomatic surface stays `string` and the unavoidable
    // transcode is routed through the explicitly-named `Diplomat.Utf8.Clone`.
    #[test]
    fn validated_str_input_uses_utf8_clone() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Hasher;

                impl Hasher {
                    pub fn hash(data: &str) -> u32 {
                        unimplemented!()
                    }
                }
            }
        };

        let (files, errors) = run_dotnet(tk_stream);
        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );

        let hasher = files.get("Hasher.cs").expect("expected Hasher.cs output");
        assert!(
            hasher.contains("public static uint Hash(string data)"),
            "validated &str should keep the string idiomatic param:\n{hasher}"
        );
        assert!(
            hasher.contains("byte[] dataBytes = Diplomat.Utf8.Clone(data);")
                && hasher.contains("fixed (byte* dataPtr = dataBytes)"),
            "the unavoidable transcode should be named Utf8.Clone, not inlined:\n{hasher}"
        );
    }

    // A C# `string` is already a flat UTF-16 buffer — `&DiplomatStr16` pins
    // it directly with no allocation.
    #[test]
    fn temporary_diplomat_str16_keeps_string_fixed_pinning() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                use diplomat_runtime::DiplomatStr16;

                #[diplomat::opaque]
                pub struct Hasher;

                impl Hasher {
                    pub fn hash(data: &DiplomatStr16) -> u32 {
                        unimplemented!()
                    }
                }
            }
        };

        let (files, errors) = run_dotnet(tk_stream);
        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );

        let hasher = files.get("Hasher.cs").expect("expected Hasher.cs output");
        assert!(
            hasher.contains("public static uint Hash(string data)")
                && hasher.contains("fixed (char* dataPtr = data)"),
            "&DiplomatStr16 should pin the C# string directly, zero-copy:\n{hasher}"
        );
        assert!(
            !hasher.contains("Encoding.UTF8") && !hasher.contains("Utf8.Clone"),
            "&DiplomatStr16 must never transcode:\n{hasher}"
        );
    }

    // A `&DiplomatStr16` borrowed by an owned opaque return pins via
    // `ReadOnlyMemory<char>` — same keep-alive-edge mechanism as `&[u8]`.
    #[test]
    fn owned_return_borrowing_diplomat_str16_input_pins_via_readonly_memory_char() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                use diplomat_runtime::DiplomatStr16;

                #[diplomat::opaque]
                pub struct Foo<'a>(&'a DiplomatStr16);

                impl<'a> Foo<'a> {
                    pub fn new(x: &'a DiplomatStr16) -> Box<Self> {
                        unimplemented!()
                    }
                }
            }
        };

        let (files, errors) = run_dotnet(tk_stream);
        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );

        let foo = files.get("Foo.cs").expect("expected Foo.cs output");
        assert!(
            foo.contains("public static Foo New(ReadOnlyMemory<char> x)"),
            "borrowed &DiplomatStr16 param should surface as ReadOnlyMemory<char>:\n{foo}"
        );
        assert!(
            foo.contains("DiplomatPinnedMemory.Pin(x)") && foo.contains("(char*)xPin.Pointer"),
            "borrowed &DiplomatStr16 should be pinned via DiplomatPinnedMemory:\n{foo}"
        );
        assert!(
            foo.contains("new Foo(result, xPin)"),
            "infallible owned return should root the pin holder as an edge:\n{foo}"
        );
    }

    // If only the ERROR borrows the slice, the thrown exception would have to
    // own the pin, but nothing ever disposes an exception — reject.
    #[test]
    fn error_borrowing_byte_slice_is_rejected() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Validator;

                #[diplomat::opaque]
                pub struct BadData<'a>(&'a [u8]);

                impl Validator {
                    pub fn check<'a>(data: &'a [u8]) -> Result<(), Box<BadData<'a>>> {
                        unimplemented!()
                    }
                }
            }
        };

        let (_files, errors) = run_dotnet(tk_stream);
        let error_str = errors.join("\n");
        assert!(
            error_str.contains("error return borrows from slice/string parameter"),
            "unexpected diagnostics: {error_str}"
        );
    }

    // A `null` success would leave the pin holder with no owner to unpin it.
    #[test]
    fn optional_owned_return_borrowing_byte_slice_is_rejected() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Finder<'a>(&'a [u8]);

                impl<'a> Finder<'a> {
                    pub fn find(data: &'a [u8]) -> Option<Box<Finder<'a>>> {
                        unimplemented!()
                    }
                }
            }
        };

        let (_files, errors) = run_dotnet(tk_stream);
        let error_str = errors.join("\n");
        assert!(
            error_str.contains("Option-wrapped return borrowing from a slice parameter"),
            "unexpected diagnostics: {error_str}"
        );
    }

    // ReadOnlyMemory can't hand Rust a `&mut [u8]` view; a borrowed mutable
    // slice would need Memory<T> plumbing that doesn't exist yet.
    #[test]
    fn mutable_borrowed_byte_slice_is_rejected() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Wrapper<'a>(&'a mut [u8]);

                impl<'a> Wrapper<'a> {
                    pub fn wrap(data: &'a mut [u8]) -> Box<Wrapper<'a>> {
                        unimplemented!()
                    }
                }
            }
        };

        let (_files, errors) = run_dotnet(tk_stream);
        let error_str = errors.join("\n");
        assert!(
            error_str.contains("mutable slice parameter")
                && error_str.contains("borrowed by the output"),
            "unexpected diagnostics: {error_str}"
        );
    }

    // A run with NO pinned-slice return must not ship the System.Memory-
    // dependent pin helper at all — the netstandard2.0 floor would fail to
    // compile it.
    #[test]
    fn run_without_pinning_omits_pin_helper() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Plain;

                impl Plain {
                    pub fn make() -> Box<Plain> {
                        unimplemented!()
                    }
                }
            }
        };

        let (files, errors) = run_dotnet(tk_stream);
        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );
        assert!(
            !files.contains_key("DiplomatPinnedMemory.cs"),
            "no pinned return means the pin helper must not be emitted"
        );
    }

    // A run WITH a pinned-slice return ships the helper, and the returned
    // type's own wrapper threads its pin(s) straight into its own `_edges`
    // field (cross-type: this must hold regardless of which type declares
    // the pinning method).
    #[test]
    fn run_with_pinning_emits_pin_helper_and_threads_pins_into_rust_handle_state() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Viewer<'a>(&'a [u8]);

                impl<'a> Viewer<'a> {
                    pub fn open(data: &'a [u8]) -> Box<Viewer<'a>> {
                        unimplemented!()
                    }
                }
            }
        };

        let (files, errors) = run_dotnet(tk_stream);
        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );
        assert!(
            files.contains_key("DiplomatPinnedMemory.cs"),
            "a pinned return should emit the pin helper"
        );
        let viewer = files.get("Viewer.cs").expect("expected Viewer.cs output");
        assert!(
            viewer.contains("_inner = RustHandle<Raw.Viewer>.Owned(handle, _destroy);")
                && viewer.contains("Owned(handle, _destroy, edges)"),
            "a pinned return threads its pin into RustHandle.Owned \
             every opaque uses:\n{viewer}"
        );
        assert!(
            !viewer.contains("as DiplomatPinnedMemory"),
            "the wrapper itself should not manually sweep pins:\n{viewer}"
        );
    }

    // A borrowed opaque return builds a non-owning wrapper whose Dispose never
    // runs the Rust destructor, so it must not root a pin — unpinning there
    // would free the buffer while Rust still holds the slice. Reject it.
    #[test]
    fn borrowed_opaque_return_borrowing_slice_is_rejected() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct SliceView<'a>(&'a [u8]);

                impl<'a> SliceView<'a> {
                    pub fn peek(&'a self, data: &'a [u8]) -> &'a SliceView<'a> {
                        unimplemented!()
                    }
                }
            }
        };

        let (_files, errors) = run_dotnet(tk_stream);
        let error_str = errors.join("\n");
        assert!(
            error_str.contains("only owned opaque success returns borrowing from"),
            "unexpected diagnostics: {error_str}"
        );
    }

    // A borrowing error (`Box<BorrowingError<'a>>`) must thread the receiver edge onto
    // both the Ok wrapper and the thrown exception, or the owner can be finalized while
    // either is still live.
    #[test]
    fn fallible_borrowed_return_with_borrowing_error_threads_edges_to_exception() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Owner;

                #[diplomat::opaque]
                pub struct BorrowingError<'a>(&'a Owner);

                impl Owner {
                    pub fn try_borrow<'a>(
                        &'a self,
                        fail: bool,
                    ) -> Result<&'a Self, Box<BorrowingError<'a>>> {
                        unimplemented!()
                    }
                }
            }
        };

        let (files, errors) = run_dotnet(tk_stream);
        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );

        let owner = files.get("Owner.cs").expect("expected Owner.cs output");
        assert!(
            owner.contains(".Borrowed("),
            "Ok path should use the non-owning Borrowed factory:\n{owner}"
        );
        assert!(
            owner.contains(
                "throw new BorrowingErrorException(new BorrowingError(result.Err, new ILifetimeEdge?[] { LifetimeEdge.Move(ref selfLease) }));"
            ),
            "error path should retain the receiver directly, at the inner error's construction:\n{owner}"
        );

        let exc = files
            .get("BorrowingErrorException.cs")
            .expect("expected BorrowingErrorException.cs output");
        assert!(
            exc.contains("public BorrowingErrorException(BorrowingError inner)"),
            "exception class no longer needs its own keep-alive edges — the inner \
             error opaque's own RC state already retains the dependency:\n{exc}"
        );
    }

    #[test]
    fn fallible_owned_return_with_borrowing_error_threads_edges_to_exception() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Owner;

                #[diplomat::opaque]
                pub struct BorrowingError<'a>(&'a Owner);

                impl Owner {
                    // Ok is i32 (owned, no edges), but the error borrows 'a from self.
                    pub fn try_get<'a>(&'a self) -> Result<i32, Box<BorrowingError<'a>>> {
                        unimplemented!()
                    }
                }
            }
        };

        let (files, errors) = run_dotnet(tk_stream);
        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );

        let owner = files.get("Owner.cs").expect("expected Owner.cs output");
        assert!(
            owner.contains(
                "throw new BorrowingErrorException(new BorrowingError(result.Err, new ILifetimeEdge?[] { LifetimeEdge.Move(ref selfLease) }));"
            ),
            "error path should retain the receiver directly, at the inner error's construction:\n{owner}"
        );

        let exc = files
            .get("BorrowingErrorException.cs")
            .expect("expected BorrowingErrorException.cs output");
        assert!(
            exc.contains("public BorrowingErrorException(BorrowingError inner)"),
            "exception class no longer needs its own keep-alive edges — the inner \
             error opaque's own RC state already retains the dependency:\n{exc}"
        );
    }

    // ─────────────────────────────────────────────────────────────────────
    // Owned `Box<[u8]>` return -> `RustVec` (owned_byte_slice_returns)
    // ─────────────────────────────────────────────────────────────────────

    #[test]
    fn owned_byte_slice_return_lowers_to_rustvec() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Buf;

                impl Buf {
                    pub fn make(len: u32) -> Box<[u8]> {
                        unimplemented!()
                    }
                }
            }
        };

        let (files, errors) = run_dotnet(tk_stream);
        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );

        let buf = files.get("Buf.cs").expect("expected Buf.cs output");
        assert!(
            buf.contains("public static RustVec Make(uint len)"),
            "idiomatic signature should return RustVec:\n{buf}"
        );
        assert!(
            buf.contains("new RustVec(result.Ptr, result.Len)"),
            "idiomatic body should wrap the raw (ptr, len) pair in RustVec:\n{buf}"
        );

        let raw_buf = files.get("RawBuf.cs").expect("expected RawBuf.cs output");
        assert!(
            raw_buf.contains("internal static unsafe extern DiplomatOwnedSliceU8 Make(uint len);"),
            "raw extern should return the DiplomatOwnedSliceU8 (ptr, len) struct by value:\n{raw_buf}"
        );

        let rust_vec = files
            .get("RustVec.cs")
            .expect("an owned byte-slice return should emit the RustVec runtime helper");
        assert!(
            rust_vec.contains("public sealed class RustVec : IDisposable")
                && rust_vec.contains("public void WithSpan(RustVecSpanAction action)")
                && rust_vec.contains("public byte[] Clone()")
                && rust_vec.contains("~RustVec()"),
            "RustVec should provide scoped access, explicit cloning, and GC fallback:\n{rust_vec}"
        );
        assert!(
            !rust_vec.contains("public sealed unsafe class RustVec")
                && !rust_vec.contains("MemoryManager<byte>")
                && !rust_vec.contains("public Span<byte> GetSpan")
                && !rust_vec.contains("DllImport"),
            "RustVec must not expose an escaping memory view:\n{rust_vec}"
        );
        let raw_rust_vec = files
            .get("RawRustVec.cs")
            .expect("an owned byte-slice return should emit the raw RustVec helper");
        assert!(
            raw_rust_vec.contains("namespace Somelib.Raw;")
                && raw_rust_vec.contains("internal static extern void Destroy"),
            "raw RustVec should own the destroy import:\n{raw_rust_vec}"
        );
        assert!(
            files.contains_key("DiplomatOwnedSliceU8.cs"),
            "an owned byte-slice return should emit the DiplomatOwnedSliceU8 raw struct"
        );
    }

    // A run that never returns an owned byte slice must not ship RustVec or
    // its raw struct — same "only emit what's used" discipline as
    // `DiplomatPinnedMemory` (see `run_without_pinning_omits_pin_helper_and_sweep`).
    #[test]
    fn run_without_owned_byte_slice_return_omits_rustvec_helper() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Plain;

                impl Plain {
                    pub fn make() -> Box<Plain> {
                        unimplemented!()
                    }
                }
            }
        };

        let (files, errors) = run_dotnet(tk_stream);
        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );
        assert!(
            !files.contains_key("RustVec.cs"),
            "no owned byte-slice return means RustVec must not be emitted"
        );
        assert!(
            !files.contains_key("RawRustVec.cs"),
            "no owned byte-slice return means raw RustVec must not be emitted"
        );
        assert!(
            !files.contains_key("DiplomatOwnedSliceU8.cs"),
            "no owned byte-slice return means its raw struct must not be emitted"
        );
    }

    #[test]
    fn owned_byte_slice_return_reports_unsupported_backend() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Buf;

                impl Buf {
                    pub fn make(len: u32) -> Box<[u8]> {
                        unimplemented!()
                    }
                }
            }
        };

        let errors = lowering_errors(tk_stream, false);
        assert_eq!(errors.len(), 1, "unexpected diagnostics: {errors:?}");
        assert!(
            errors[0].contains("#[diplomat::cfg(supports = owned_byte_slice_returns)]"),
            "unexpected diagnostic: {}",
            errors[0]
        );
    }

    #[test]
    fn owned_slice_return_of_non_u8_primitive_is_rejected() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Buf;

                impl Buf {
                    pub fn make(len: u32) -> Box<[u32]> {
                        unimplemented!()
                    }
                }
            }
        };

        let errors = lowering_errors(tk_stream, true);
        assert_eq!(errors.len(), 1, "unexpected diagnostics: {errors:?}");
        assert!(
            errors[0].contains("except for top-level method-return `Box<[u8]>`"),
            "unexpected diagnostic: {}",
            errors[0]
        );
    }

    // `Option<Box<[u8]>>` lowers to tagged DiplomatOption + idiomatic `RustVec?`.
    #[test]
    fn optional_owned_byte_slice_return_lowers_to_nullable_rustvec() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Buf;

                impl Buf {
                    pub fn make(len: u32) -> Option<Box<[u8]>> {
                        unimplemented!()
                    }
                }
            }
        };

        let (files, errors) = run_dotnet(tk_stream);
        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );

        let buf = files.get("Buf.cs").expect("expected Buf.cs output");
        assert!(
            buf.contains("public static RustVec? Make(uint len)"),
            "idiomatic signature should return RustVec?:\n{buf}"
        );
        assert!(
            buf.contains(
                "return result.IsSome ? new RustVec(result.Value.Ptr, result.Value.Len) : null;"
            ),
            "None should be null and Some should wrap as RustVec:\n{buf}"
        );

        assert!(
            files.contains_key("DiplomatOptionDiplomatOwnedSliceU8.cs"),
            "optional owned-byte-slice return should emit the DiplomatOption helper"
        );
        assert!(
            files.contains_key("RustVec.cs"),
            "optional owned-byte-slice return should still emit RustVec"
        );
    }

    // `Result<Box<[u8]>, E>` is supported when `owned_byte_slice_returns` is on:
    // the macro lowers the `Ok` payload to `DiplomatOwnedSlice<u8>`, and the
    // idiomatic .NET surface wraps it as `RustVec`.
    #[test]
    fn fallible_owned_byte_slice_return_lowers_to_rustvec() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Buf;

                pub enum MyError {
                    A,
                }

                impl Buf {
                    pub fn make(len: u32) -> Result<Box<[u8]>, MyError> {
                        unimplemented!()
                    }
                }
            }
        };

        let (files, errors) = run_dotnet(tk_stream);
        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );

        let buf = files.get("Buf.cs").expect("expected Buf.cs output");
        assert!(
            buf.contains("public static RustVec Make(uint len)"),
            "idiomatic signature should return RustVec:\n{buf}"
        );
        assert!(
            buf.contains("if (!result.IsOk)")
                && buf.contains("throw new MyErrorException(result.Err);")
                && buf.contains("return new RustVec(result.Ok.Ptr, result.Ok.Len);"),
            "fallible owned-byte-slice path should throw on Err and wrap Ok as RustVec:\n{buf}"
        );

        let raw_result = files
            .get("DiplomatResultDiplomatOwnedSliceU8MyError.cs")
            .expect("expected Result helper struct for fallible owned-byte-slice return");
        assert!(
            raw_result.contains("internal DiplomatOwnedSliceU8 ok;")
                && raw_result.contains("public DiplomatOwnedSliceU8 Ok => IsOk ? _inner.ok")
                && raw_result.contains("internal MyError err;"),
            "result helper should carry owned-byte-slice Ok payload and MyError Err payload:\n{raw_result}"
        );
    }

    #[test]
    fn fallible_owned_byte_slice_return_reports_unsupported_backend() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Buf;

                pub enum MyError {
                    A,
                }

                impl Buf {
                    pub fn make(len: u32) -> Result<Box<[u8]>, MyError> {
                        unimplemented!()
                    }
                }
            }
        };

        let errors = lowering_errors(tk_stream, false);
        assert_eq!(errors.len(), 1, "unexpected diagnostics: {errors:?}");
        assert!(
            errors[0].contains("#[diplomat::cfg(supports = owned_byte_slice_returns)]"),
            "unexpected diagnostic: {}",
            errors[0]
        );
    }

    #[test]
    fn fallible_owned_byte_slice_return_with_opaque_error_is_supported() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Buf;

                #[diplomat::opaque]
                pub struct MyError;

                impl Buf {
                    pub fn make(len: u32) -> Result<Box<[u8]>, Box<MyError>> {
                        unimplemented!()
                    }
                }
            }
        };

        let (files, errors) = run_dotnet(tk_stream);
        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );

        let buf = files.get("Buf.cs").expect("expected Buf.cs output");
        assert!(
            buf.contains("public static RustVec Make(uint len)")
                && buf.contains("if (!result.IsOk)")
                && buf.contains("return new RustVec(result.Ok.Ptr, result.Ok.Len);"),
            "fallible owned-byte-slice return with opaque error should lower cleanly:\n{buf}"
        );
    }

    // The new lowering arm is scoped to method returns: an owned slice in an
    // out-struct field must keep the old rejection even with
    // `owned_byte_slice_returns` enabled.
    #[test]
    fn owned_byte_slice_out_struct_field_is_rejected() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::out]
                pub struct Out {
                    pub bytes: Box<[u8]>,
                }

                #[diplomat::opaque]
                pub struct Buf;

                impl Buf {
                    pub fn make(len: u32) -> Out {
                        unimplemented!()
                    }
                }
            }
        };

        let errors = lowering_errors(tk_stream, true);
        assert!(
            !errors.is_empty()
                && errors
                    .iter()
                    .any(|e| e.contains("except for top-level method-return `Box<[u8]>`")),
            "unexpected diagnostics: {errors:?}"
        );
    }

    // This guards the return-only capability from enabling owned slice parameters.
    #[test]
    fn owned_byte_slice_parameter_is_still_rejected() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Buf;

                impl Buf {
                    pub fn take(v: Box<[u8]>) {
                        unimplemented!()
                    }
                }
            }
        };

        let errors = lowering_errors(tk_stream, true);
        assert_eq!(errors.len(), 1, "unexpected diagnostics: {errors:?}");
        assert!(
            errors[0].contains("Owned slices are not supported in this backend"),
            "an owned slice parameter must still be rejected now that \
             owned_byte_slice_returns is enabled for the return position; got: {}",
            errors[0]
        );
    }

    // A borrowed string return (`&'a str` family) is a zero-copy
    // `DiplomatBorrowedSpan<byte>` that retains its opaque source until the
    // span wrapper is finalized.
    #[test]
    fn borrowed_string_return_generates_diplomat_borrowed_span() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                use diplomat_runtime::DiplomatStrSlice;

                #[diplomat::opaque]
                pub struct MyString;

                impl MyString {
                    pub fn borrow<'a>(&'a self) -> DiplomatStrSlice<'a> {
                        unimplemented!()
                    }
                }
            }
        };

        let (files, errors) = run_dotnet(tk_stream);
        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );

        let my_string = files
            .get("MyString.cs")
            .expect("expected MyString.cs output");
        assert!(
            my_string.contains("public DiplomatBorrowedSpan<byte> Borrow()"),
            "borrowed string return should surface as DiplomatBorrowedSpan<byte>:\n{my_string}"
        );
        assert!(
            my_string.contains(
                "new DiplomatBorrowedSpan<byte>(result.Ptr, result.Len, new ILifetimeEdge?[] { LifetimeEdge.Move(ref selfLease) })"
            ),
            "the returned view should retain `this` as a managed lifetime edge:\n{my_string}"
        );

        let span = files
            .get("DiplomatBorrowedSpan.cs")
            .expect("DiplomatBorrowedSpan.cs should be emitted when a run returns one");
        assert!(
            span.contains("public sealed unsafe class DiplomatBorrowedSpan<T>"),
            "the view must be a sealed reference type so aliases share one finalizer:\n{span}"
        );
        assert!(
            span.contains("public void WithSpan(DiplomatBorrowedSpanAction<T> action)"),
            "the view should expose zero-copy access scoped to a callback, mirroring \
             RustVec's WithSpan — never a bare Span-returning property:\n{span}"
        );
        assert!(
            span.contains("public T[] Clone()"),
            "an independent copy should be a separate, explicitly-named operation:\n{span}"
        );
        assert!(
            span.contains("public sealed unsafe class DiplomatBorrowedSpan<T> : IDisposable"),
            "the view should release its managed source edges explicitly:\n{span}"
        );
        assert!(
            span.contains("Cannot dispose DiplomatBorrowedSpan during a WithSpan callback"),
            "disposing a borrowed span from its active callback must fail before releasing edges:\n{span}"
        );
        assert!(
            span.contains("LifetimeEdges.ReleaseNoThrow(edges)"),
            "the view cleanup must release source edges from both Dispose and finalization:\n{span}"
        );
        assert!(
            span.contains("if (len > (nuint)int.MaxValue)")
                && span.contains("Borrowed Rust slice is too large for a .NET Span/Memory"),
            "oversize construction must release the edges it was handed and then throw:\n{span}"
        );
    }

    // The same view type covers a borrowed primitive slice return, not just
    // strings — `lower_return` previously had no `Type::Slice` arm at all.
    #[test]
    fn borrowed_u32_slice_return_generates_diplomat_borrowed_span() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Buffer;

                impl Buffer {
                    pub fn borrow<'a>(&'a self) -> &'a [u32] {
                        unimplemented!()
                    }
                }
            }
        };

        let (files, errors) = run_dotnet(tk_stream);
        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );

        let buffer = files.get("Buffer.cs").expect("expected Buffer.cs output");
        assert!(
            buffer.contains("public DiplomatBorrowedSpan<uint> Borrow()"),
            "borrowed &[u32] return should surface as DiplomatBorrowedSpan<uint>:\n{buffer}"
        );
        assert!(
            buffer.contains(
                "new DiplomatBorrowedSpan<uint>(result.Ptr, result.Len, new ILifetimeEdge?[] { LifetimeEdge.Move(ref selfLease) })"
            ),
            "the returned view should retain `this` as a managed lifetime edge:\n{buffer}"
        );
    }

    // A run that never returns a borrowed span shouldn't pay for the
    // System.Memory-dependent helper — mirrors
    // `run_without_pinning_omits_pin_helper_and_sweep`.
    #[test]
    fn run_without_borrowed_span_omits_the_helper() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Foo;

                impl Foo {
                    pub fn value(&self) -> u32 {
                        unimplemented!()
                    }
                }
            }
        };

        let (files, errors) = run_dotnet(tk_stream);
        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );
        assert!(
            !files.contains_key("DiplomatBorrowedSpan.cs"),
            "a run that never returns a borrowed span shouldn't emit the helper"
        );
    }

    // Wrapping a borrowed span return in Result/Option hasn't been exercised
    // end-to-end (the result/option helper structs' bridging was only ever
    // built for opaque/primitive/struct/enum arms) — reject rather than risk
    // generating broken code, instead of silently mis-lowering it.
    #[test]
    fn fallible_borrowed_string_return_is_rejected() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                use diplomat_runtime::DiplomatStrSlice;

                #[diplomat::opaque]
                pub struct MyString;

                #[diplomat::opaque]
                pub struct MyError;

                impl MyString {
                    pub fn try_borrow<'a>(&'a self) -> Result<DiplomatStrSlice<'a>, Box<MyError>> {
                        unimplemented!()
                    }
                }
            }
        };

        let (_files, errors) = run_dotnet(tk_stream);
        assert_eq!(errors.len(), 1);
        assert!(
            errors[0].contains("wrapping a borrowed span return"),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );
    }

    // A borrowed span that borrows a slice/string param would pin that
    // buffer onto DiplomatBorrowedSpan._edges, but the span has no Dispose
    // and the pin holder has no finalizer — permanent pin. Reject rather
    // than generate a leaky binding. (Owned opaque success returns that
    // borrow a slice param remain supported: their Dispose unpins.)
    #[test]
    fn borrowed_span_return_borrowing_slice_param_is_rejected() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                use diplomat_runtime::DiplomatStr;

                #[diplomat::opaque]
                pub struct MyString;

                impl MyString {
                    pub fn echo_str<'a>(&'a self, s: &'a DiplomatStr) -> &'a DiplomatStr {
                        unimplemented!()
                    }
                }
            }
        };

        let (_files, errors) = run_dotnet(tk_stream);
        assert_eq!(errors.len(), 1, "unexpected diagnostics: {errors:?}");
        assert!(
            errors[0].contains("borrows from slice/string parameter"),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );
    }

    // `'static` string returns have no managed owner and no dispose path on
    // the span — reject until there's an explicit static-slice design.
    #[test]
    fn static_string_return_is_rejected() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct MyString;

                impl MyString {
                    pub fn get_static_str() -> &'static str {
                        unimplemented!()
                    }
                }
            }
        };

        // HIR may still lower `'static` returns; the .NET backend rejects them
        // in lower_return (defense in depth on top of static_slices=false).
        let (_files, errors) = run_dotnet(tk_stream);
        assert_eq!(errors.len(), 1, "unexpected diagnostics: {errors:?}");
        assert!(
            errors[0].contains("'static") || errors[0].contains("static"),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );
    }

    fn property_test_module_with_type_attrs(
        type_attrs: proc_macro2::TokenStream,
        methods: proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        quote! {
            #[diplomat::bridge]
            mod ffi {
                #type_attrs
                #[diplomat::opaque]
                pub struct Config;

                impl Config {
                    #methods
                }
            }
        }
    }

    fn property_test_module(methods: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        property_test_module_with_type_attrs(quote! {}, methods)
    }

    #[test]
    fn opaque_defaults_to_finalizer_only_cleanup() {
        let (files, errors) = run_dotnet(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Plain;

                impl Plain {
                    #[diplomat::attr(auto, constructor)]
                    pub fn new() -> Box<Self> {
                        unimplemented!()
                    }
                }
            }
        });

        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );
        let plain = files.get("Plain.cs").expect("expected Plain.cs output");
        assert!(
            plain.contains("public partial class Plain") && !plain.contains(": IDisposable"),
            "default opaque should not implement IDisposable:\n{plain}"
        );
        assert!(
            !plain.contains("public void Dispose()"),
            "default opaque should not expose public Dispose:\n{plain}"
        );
        assert!(
            !plain.contains("private void Cleanup()") && !plain.contains("~Plain()"),
            "default opaque leaves all cleanup to its RustHandle:\n{plain}"
        );
    }

    #[test]
    fn opaque_manually_disposable_opt_in_emits_public_dispose() {
        // Generate both shapes from one bridge so the attribute is what flips
        // the public Dispose surface, not some ambient backend default.
        let (files, errors) = run_dotnet(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct FinalizerOnly;

                impl FinalizerOnly {
                    #[diplomat::attr(auto, constructor)]
                    pub fn new() -> Box<Self> {
                        unimplemented!()
                    }

                    pub fn ping(&self) {}
                }

                #[diplomat::attr(dotnet, manually_disposable)]
                #[diplomat::opaque]
                pub struct Manual;

                impl Manual {
                    #[diplomat::attr(auto, constructor)]
                    pub fn new() -> Box<Self> {
                        unimplemented!()
                    }

                    pub fn ping(&self) {}
                }
            }
        });

        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );

        let finalizer_only = files
            .get("FinalizerOnly.cs")
            .expect("expected FinalizerOnly.cs output");
        assert!(
            finalizer_only.contains("public partial class FinalizerOnly")
                && !finalizer_only.contains(": IDisposable")
                && !finalizer_only.contains("public void Dispose()"),
            "unmarked opaque must stay finalizer-only:\n{finalizer_only}"
        );
        assert!(
            !finalizer_only.contains("private void Cleanup()")
                && !finalizer_only.contains("~FinalizerOnly()"),
            "unmarked opaque leaves all cleanup to its RustHandle:\n{finalizer_only}"
        );

        let manual = files.get("Manual.cs").expect("expected Manual.cs output");
        assert!(
            manual.contains("public partial class Manual : IDisposable"),
            "`manually_disposable` must generate `: IDisposable`:\n{manual}"
        );
        assert!(
            manual.contains("public void Dispose()")
                && manual.contains("inner.ReleaseWrapper();")
                && !manual.contains("GC.SuppressFinalize(this);"),
            "`manually_disposable` must release its RustHandle directly:\n{manual}"
        );
        assert!(
            !manual.contains("~Manual()"),
            "opted-in opaque leaves finalization to its RustHandle:\n{manual}"
        );
        assert!(
            manual.contains("public void Ping()")
                && manual.contains("ObjectDisposedException(\"Manual\")"),
            "instance methods on a manually_disposable opaque must reject use-after-dispose:\n{manual}"
        );
    }

    #[test]
    fn manually_disposable_attr_requires_simple_path() {
        let errors = lowering_errors(
            quote! {
                #[diplomat::bridge]
                mod ffi {
                    #[diplomat::attr(dotnet, manually_disposable = true)]
                    #[diplomat::opaque]
                    pub struct Bad;
                }
            },
            true,
        );

        assert!(
            errors
                .iter()
                .any(|e| e.contains("`manually_disposable` must be a simple path")),
            "expected simple-path validation error, got: {errors:?}"
        );
    }

    #[test]
    fn manually_disposable_attr_invalid_contexts_are_rejected() {
        let errors = lowering_errors(
            quote! {
                #[diplomat::bridge]
                mod ffi {
                    #[diplomat::attr(dotnet, manually_disposable)]
                    pub struct NotOpaque {
                        value: u8,
                    }

                    #[diplomat::attr(dotnet, manually_disposable)]
                    pub enum NotOpaqueEnum {
                        A,
                    }

                    #[diplomat::opaque]
                    pub struct GoodOpaque;

                    impl GoodOpaque {
                        #[diplomat::attr(dotnet, manually_disposable)]
                        pub fn bad(&self) {
                        }
                    }
                }
            },
            true,
        );

        let wrong_context_count = errors
            .iter()
            .filter(|e| e.contains("`manually_disposable` can only be used on opaque types"))
            .count();
        assert_eq!(
            wrong_context_count, 3,
            "expected 3 context errors (struct, enum, method), got: {errors:?}"
        );
    }

    #[test]
    fn manually_disposable_attr_duplicate_is_rejected() {
        let errors = lowering_errors(
            quote! {
                #[diplomat::bridge]
                mod ffi {
                    #[diplomat::attr(dotnet, manually_disposable)]
                    #[diplomat::attr(dotnet, manually_disposable)]
                    #[diplomat::opaque]
                    pub struct Duplicate;
                }
            },
            true,
        );

        assert!(
            errors
                .iter()
                .any(|e| e.contains("Duplicate `manually_disposable` attribute")),
            "expected duplicate-manually_disposable error, got: {errors:?}"
        );
    }

    #[test]
    fn non_dotnet_gated_manually_disposable_does_not_affect_dotnet_codegen() {
        let (files, errors) = run_dotnet(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::attr(not(dotnet), manually_disposable)]
                #[diplomat::opaque]
                pub struct Gated;

                impl Gated {
                    #[diplomat::attr(auto, constructor)]
                    pub fn new() -> Box<Self> {
                        unimplemented!()
                    }
                }
            }
        });

        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );
        let gated = files.get("Gated.cs").expect("expected Gated.cs output");
        assert!(
            !gated.contains(": IDisposable") && !gated.contains("public void Dispose()"),
            "dotnet-disabled manually_disposable attr must not change dotnet output:\n{gated}"
        );
    }

    #[test]
    fn getter_and_setter_pair_share_one_property() {
        let (files, _errors) = run_dotnet(property_test_module(quote! {
            #[diplomat::attr(auto, getter)]
            pub fn size(&self) -> usize {
                unimplemented!()
            }

            #[diplomat::attr(auto, setter = "size")]
            pub fn set_size(&self, size: usize) {
                unimplemented!()
            }
        }));

        let config = files.get("Config.cs").expect("expected Config.cs output");
        assert!(
            config.contains("public nuint Size"),
            "expected a Size property, got:
{config}"
        );
        assert!(
            property_getter_count(config) == 1 && config.contains("        set"),
            "expected one property carrying both accessors, got:
{config}"
        );
        // The accessor *is* the property: its body is inline and there is no
        // second member to collide with, as in the Dart and JS backends.
        assert!(
            config.contains("Raw.Config.Size(selfLease!.Ptr)"),
            "expected the getter body inline in the property, got:
{config}"
        );
        assert!(
            !config.contains("nuint Size()") && !config.contains("void SetSize("),
            "an accessor must not also emit a method, got:
{config}"
        );
    }

    // A setter with no getter to pair with still deserves a property — write-only
    // properties are legal C#, and a Rust config object often only has setters.
    #[test]
    fn setter_without_a_getter_renders_a_write_only_property() {
        let (files, _errors) = run_dotnet(property_test_module(quote! {
            #[diplomat::attr(auto, setter = "size")]
            pub fn set_size(&self, size: usize) {
                unimplemented!()
            }
        }));

        let config = files.get("Config.cs").expect("expected Config.cs output");
        assert!(
            config.contains("public nuint Size"),
            "expected a write-only Size property, got:
{config}"
        );
        assert!(
            config.contains("        set"),
            "expected a write-only property, got:
{config}"
        );
        assert!(
            property_getter_count(config) == 0,
            "there is no getter to read through, got:
{config}"
        );
        assert!(
            !config.contains("void SetSize("),
            "the accessor must not also emit a method, got:
{config}"
        );
    }

    #[test]
    fn getter_without_a_setter_renders_a_read_only_property() {
        let (files, _errors) = run_dotnet(property_test_module(quote! {
            #[diplomat::attr(auto, getter)]
            pub fn size(&self) -> usize {
                unimplemented!()
            }
        }));

        let config = files.get("Config.cs").expect("expected Config.cs output");
        assert!(
            config.contains("public nuint Size"),
            "expected a Size property, got:
{config}"
        );
        assert!(
            !config.contains("        set"),
            "there is no setter to write through, got:
{config}"
        );
        assert!(
            !config.contains("nuint Size()"),
            "the accessor must not also emit a method, got:
{config}"
        );
    }

    // Every string encoding presents as `string` in a property, so a pair can
    // only disagree if the author declared two genuinely different types. C# has
    // no way to express that as one property, so the backend refuses rather than
    // silently dropping the setter.
    #[test]
    fn setter_whose_type_disagrees_with_the_getter_is_rejected() {
        let (_files, errors) = run_dotnet(property_test_module(quote! {
            #[diplomat::attr(auto, getter)]
            pub fn size(&self) -> usize {
                unimplemented!()
            }

            #[diplomat::attr(auto, setter = "size")]
            pub fn set_size(&self, size: i32) {
                unimplemented!()
            }
        }));

        assert_eq!(
            errors.len(),
            1,
            "expected exactly one diagnostic: {errors:?}"
        );
        let error = &errors[0];
        for expected in [
            "Config",
            "property `Size` would need two types",
            "`nuint`",
            "`size`",
            "`int`",
            "`set_size`",
        ] {
            assert!(
                error.contains(expected),
                "the diagnostic must name {expected}; got: {error}"
            );
        }
    }

    // The two sides marshal differently — the getter hands out a view into
    // Rust-owned memory the caller must not outlive, the setter takes a managed
    // array Rust only reads during the call — so no single C# type serves both.
    // The diagnostic has to say which marshal each side chose, because the C#
    // types alone don't explain why they can't meet.
    #[test]
    fn byte_slice_accessors_that_marshal_differently_are_rejected() {
        let (_files, errors) = run_dotnet(property_test_module(quote! {
            #[diplomat::attr(auto, getter)]
            pub fn data<'a>(&'a self) -> &'a [u8] {
                unimplemented!()
            }

            #[diplomat::attr(auto, setter = "data")]
            pub fn set_data(&self, data: &[u8]) {
                unimplemented!()
            }
        }));

        assert_eq!(
            errors.len(),
            1,
            "expected exactly one diagnostic: {errors:?}"
        );
        let error = &errors[0];
        for expected in [
            "DiplomatBorrowedSpan<byte>",
            "byte[]",
            "a borrowed view over Rust-owned memory",
            "a managed array Rust reads during the call",
        ] {
            assert!(
                error.contains(expected),
                "the diagnostic must name {expected}; got: {error}"
            );
        }
    }

    // Two methods claiming one slot would be two C# members with one name
    // (CS0102), which does not compile at all.
    #[test]
    fn two_getters_on_one_property_name_are_rejected() {
        let (files, errors) = run_dotnet(property_test_module(quote! {
            #[diplomat::attr(auto, getter = "size")]
            pub fn size(&self) -> usize {
                unimplemented!()
            }

            #[diplomat::attr(auto, getter = "size")]
            pub fn other_size(&self) -> usize {
                unimplemented!()
            }
        }));

        assert_eq!(
            errors.len(),
            1,
            "expected exactly one diagnostic: {errors:?}"
        );
        let error = &errors[0];
        for expected in ["`get` of property `Size`", "`size`", "`other_size`"] {
            assert!(
                error.contains(expected),
                "the diagnostic must name {expected}; got: {error}"
            );
        }
        let config = files.get("Config.cs").expect("expected Config.cs output");
        assert_eq!(
            config.matches("public nuint Size").count(),
            1,
            "the losing accessor must not be emitted, got:
{config}"
        );
    }

    // A property and a method can collide just as easily as two properties, and
    // the template's own members (`AsFFI`, `FromFFI`, plus opt-in `Dispose`) are
    // in the same namespace.
    #[test]
    fn a_property_colliding_with_a_method_is_rejected() {
        let (_files, errors) = run_dotnet(property_test_module(quote! {
            #[diplomat::attr(auto, getter = "size")]
            pub fn get_the_size(&self) -> usize {
                unimplemented!()
            }

            pub fn size(&self) -> usize {
                unimplemented!()
            }
        }));

        assert_eq!(
            errors.len(),
            1,
            "expected exactly one diagnostic: {errors:?}"
        );
        assert!(
            errors[0].contains("two members named `Size`"),
            "the collision must be reported; got: {}",
            errors[0]
        );
    }

    // C# rejects a member sharing its enclosing type's name outright, so this
    // needs its own message rather than the two-members one.
    #[test]
    fn a_property_named_after_its_own_type_is_rejected() {
        let (_files, errors) = run_dotnet(property_test_module(quote! {
            #[diplomat::attr(auto, getter)]
            pub fn config(&self) -> u32 {
                unimplemented!()
            }
        }));

        assert_eq!(
            errors.len(),
            1,
            "expected exactly one diagnostic: {errors:?}"
        );
        let error = &errors[0];
        for expected in [
            "property `Config` has the same name as `Config`",
            "the type that contains it",
        ] {
            assert!(
                error.contains(expected),
                "the diagnostic must say {expected}; got: {error}"
            );
        }
    }

    // A `&mut self` getter may change what it reports, and a property gets read
    // more than once — by a debugger watch, a serializer, or twice in a row. A
    // one-shot `self.x.take()` behind a property drains to null on the second
    // read, which is why the receiver has to be `&self`.
    #[test]
    fn a_mut_self_getter_is_rejected() {
        let (_files, errors) = run_dotnet(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque_mut]
                pub struct Config;

                impl Config {
                    #[diplomat::attr(auto, getter = "callback")]
                    pub fn take_callback(&mut self) -> u32 {
                        unimplemented!()
                    }
                }
            }
        });

        assert_eq!(
            errors.len(),
            1,
            "expected exactly one diagnostic: {errors:?}"
        );
        let error = &errors[0];
        for expected in ["`take_callback`", "`&mut self`", "look idempotent"] {
            assert!(
                error.contains(expected),
                "the diagnostic must mention {expected}; got: {error}"
            );
        }
    }

    // Assigning is the whole point of a setter, so it keeps `&mut self` —
    // `feature_tests` has eight that depend on this.
    #[test]
    fn a_mut_self_setter_is_still_accepted() {
        let (files, errors) = run_dotnet(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque_mut]
                pub struct Config;

                impl Config {
                    #[diplomat::attr(auto, setter = "size")]
                    pub fn set_size(&mut self, value: usize) {
                        unimplemented!()
                    }
                }
            }
        });

        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );
        let config = files.get("Config.cs").expect("expected Config.cs output");
        assert!(
            config.contains("public nuint Size") && config.contains("        set"),
            "expected a write-only property, got:
{config}"
        );
    }

    // `Dispose` is only a member of an opaque, so a struct may use the name.
    #[test]
    fn a_struct_property_named_dispose_is_accepted() {
        let (files, errors) = run_dotnet(quote! {
            #[diplomat::bridge]
            mod ffi {
                pub struct Config {
                    size: u8,
                }

                impl Config {
                    #[diplomat::attr(auto, getter = "dispose")]
                    pub fn is_disposed(self) -> bool {
                        unimplemented!()
                    }
                }
            }
        });

        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );
        let config = files.get("Config.cs").expect("expected Config.cs output");
        assert!(
            config.contains("public bool Dispose"),
            "a struct has no Dispose to collide with, got:
{config}"
        );
    }

    #[test]
    fn a_property_named_dispose_is_accepted_without_manually_disposable_opt_in() {
        let (files, errors) = run_dotnet(property_test_module(quote! {
            #[diplomat::attr(auto, getter = "dispose")]
            pub fn is_disposed(&self) -> bool {
                unimplemented!()
            }
        }));

        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );
        let config = files.get("Config.cs").expect("expected Config.cs output");
        assert!(
            config.contains("public bool Dispose"),
            "without dotnet manually_disposable opt-in, an opaque may expose a Dispose-named property:\n{config}"
        );
    }

    #[test]
    fn a_property_colliding_with_dispose_is_rejected_when_manually_disposable_opted_in() {
        let (_files, errors) = run_dotnet(property_test_module_with_type_attrs(
            quote! {
                #[diplomat::attr(dotnet, manually_disposable)]
            },
            quote! {
                #[diplomat::attr(auto, getter = "dispose")]
                pub fn is_disposed(&self) -> bool {
                    unimplemented!()
                }
            },
        ));

        assert_eq!(
            errors.len(),
            1,
            "expected exactly one diagnostic: {errors:?}"
        );
        assert!(
            errors[0].contains("two members named `Dispose`"),
            "the collision must be reported; got: {}",
            errors[0]
        );
    }

    #[test]
    fn an_opaque_property_named_cleanup_is_accepted() {
        let (files, errors) = run_dotnet(property_test_module(quote! {
            #[diplomat::attr(auto, getter = "cleanup")]
            pub fn cleanup_state(&self) -> bool {
                unimplemented!()
            }
        }));

        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );
        let config = files.get("Config.cs").expect("expected Config.cs output");
        assert!(
            config.contains("public bool Cleanup"),
            "opaque wrappers no longer reserve a private Cleanup member:\n{config}"
        );
    }

    #[test]
    fn an_opaque_method_named_cleanup_is_accepted() {
        let (files, errors) = run_dotnet(property_test_module(quote! {
            pub fn cleanup(&self) {
                unimplemented!()
            }
        }));

        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );
        let config = files.get("Config.cs").expect("expected Config.cs output");
        assert!(
            config.contains("public void Cleanup()"),
            "opaque wrappers no longer reserve a private Cleanup member:\n{config}"
        );
    }

    #[test]
    fn a_struct_property_named_cleanup_is_accepted() {
        let (files, errors) = run_dotnet(quote! {
            #[diplomat::bridge]
            mod ffi {
                pub struct Config {
                    size: u8,
                }

                impl Config {
                    #[diplomat::attr(auto, getter = "cleanup")]
                    pub fn cleanup_state(self) -> bool {
                        unimplemented!()
                    }
                }
            }
        });

        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );
        let config = files.get("Config.cs").expect("expected Config.cs output");
        assert!(
            config.contains("public bool Cleanup"),
            "a struct has no generated Cleanup to collide with, got:\n{config}"
        );
    }

    // Accessors leave the method list before the run-level helper flags are
    // folded, so a getter that needs a helper type must still pull it in.
    #[test]
    fn a_getter_returning_an_owned_byte_slice_still_emits_the_rustvec_helper() {
        let (files, errors) = run_dotnet(property_test_module(quote! {
            #[diplomat::attr(auto, getter)]
            pub fn data(&self) -> Box<[u8]> {
                unimplemented!()
            }
        }));

        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join(
                "
"
            )
        );
        assert!(
            files.contains_key("RustVec.cs"),
            "an owned byte-slice getter needs the RustVec helper"
        );
        let config = files.get("Config.cs").expect("expected Config.cs output");
        assert!(
            config.contains("public RustVec Data"),
            "expected a RustVec property, got:
{config}"
        );
    }

    #[test]
    fn a_getter_returning_a_borrowed_span_still_emits_the_span_helper() {
        let (files, errors) = run_dotnet(property_test_module(quote! {
            #[diplomat::attr(auto, getter)]
            pub fn data<'a>(&'a self) -> &'a [u8] {
                unimplemented!()
            }
        }));

        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join(
                "
"
            )
        );
        assert!(
            files.contains_key("DiplomatBorrowedSpan.cs"),
            "a borrowed-span getter needs the span helper"
        );
        let config = files.get("Config.cs").expect("expected Config.cs output");
        assert!(
            config.contains("public DiplomatBorrowedSpan<byte> Data"),
            "expected a borrowed-span property, got:
{config}"
        );
    }

    // Every text encoding presents as `string`, which is the one place the
    // marshal-to-property-type mapping collapses. Each has to arrive there by its
    // own route, so each is checked for the marshalling that route implies.
    #[test]
    fn every_text_marshal_presents_as_a_string_property() {
        // (setter param type, the marshalling only that encoding produces)
        let cases = [
            // `&str`: Rust may assume valid UTF-8, so a real string is transcoded.
            (quote!(&str), "Diplomat.Utf8.Clone(value)"),
            // `&DiplomatStr`: opaque bytes, but a property cannot read as
            // `string` and write as `byte[]`, so it transcodes here too.
            (quote!(&DiplomatStr), "Diplomat.Utf8.Clone(value)"),
            // `&DiplomatStr16`: a C# string is already UTF-16, so it is pinned
            // where it lies and nothing is allocated.
            (quote!(&DiplomatStr16), "fixed (char* valuePtr = value)"),
        ];

        for (param, marshalling) in cases {
            let (files, errors) = run_dotnet(property_test_module(quote! {
                #[diplomat::attr(auto, getter)]
                pub fn text(&self, w: &mut DiplomatWrite) {
                    unimplemented!()
                }

                #[diplomat::attr(auto, setter = "text")]
                pub fn set_text(&self, value: #param) {
                    unimplemented!()
                }
            }));

            assert!(
                errors.is_empty(),
                "{param} should pair with a written-UTF-8 getter: {}",
                errors.join("\n")
            );
            let config = files.get("Config.cs").expect("expected Config.cs output");
            assert!(
                config.contains("public string Text"),
                "{param} must present as a string property, got:
{config}"
            );
            assert_eq!(
                property_getter_count(config),
                1,
                "expected one getter for {param}, got:
{config}"
            );
            assert_eq!(
                config.matches("        set").count(),
                1,
                "expected one setter for {param}, got:
{config}"
            );
            assert!(
                config.contains(marshalling),
                "{param} must marshal via `{marshalling}`, got:
{config}"
            );
        }
    }

    // Each non-text marshal has to survive the round trip through
    // `AccessorMarshal` -> `PropertyType` -> C# type, and a pair that agrees has
    // to end up as one member with both accessors rather than two members.
    #[test]
    fn each_marshal_pairs_into_one_property() {
        // (getter return, setter param, the C# property type)
        let cases = [
            (quote!(u32), quote!(u32), "public uint Value"),
            (quote!(MyEnum), quote!(MyEnum), "public MyEnum Value"),
            (quote!(MyStruct), quote!(MyStruct), "public MyStruct Value"),
            // Owned out, borrowed in: ownership differs, the C# type does not.
            (quote!(Box<Config>), quote!(&Config), "public Config Value"),
        ];

        for (returns, param, declaration) in cases {
            let (files, errors) = run_dotnet(quote! {
                #[diplomat::bridge]
                mod ffi {
                    pub enum MyEnum { A, B }

                    pub struct MyStruct { a: u8 }

                    #[diplomat::opaque]
                    pub struct Config;

                    impl Config {
                        #[diplomat::attr(auto, getter = "value")]
                        pub fn get_value(&self) -> #returns {
                            unimplemented!()
                        }

                        #[diplomat::attr(auto, setter = "value")]
                        pub fn set_value(&self, value: #param) {
                            unimplemented!()
                        }
                    }
                }
            });

            assert!(
                errors.is_empty(),
                "{returns} / {param} should pair: {}",
                errors.join("\n")
            );
            let config = files.get("Config.cs").expect("expected Config.cs output");
            assert!(
                config.contains(declaration),
                "expected `{declaration}`, got:
{config}"
            );
            assert_eq!(
                property_getter_count(config),
                1,
                "expected exactly one getter, got:
{config}"
            );
            assert_eq!(
                config.matches("        set").count(),
                1,
                "expected exactly one setter, got:
{config}"
            );
        }
    }

    // Nullability is part of the property type, not decoration on it: `Config?`
    // and `Config` are different C# types and cannot be one property.
    #[test]
    fn a_nullable_getter_and_a_non_nullable_setter_are_rejected() {
        let (_files, errors) = run_dotnet(property_test_module(quote! {
            #[diplomat::attr(auto, getter = "value")]
            pub fn get_value(&self) -> Option<Box<Config>> {
                unimplemented!()
            }

            #[diplomat::attr(auto, setter = "value")]
            pub fn set_value(&self, value: &Config) {
                unimplemented!()
            }
        }));

        assert_eq!(
            errors.len(),
            1,
            "expected exactly one diagnostic: {errors:?}"
        );
        let error = &errors[0];
        for expected in ["`Config?`", "`Config`", "`get_value`", "`set_value`"] {
            assert!(
                error.contains(expected),
                "the diagnostic must name {expected}; got: {error}"
            );
        }
    }

    #[test]
    fn a_nullable_pair_agrees_and_renders_one_nullable_property() {
        let (files, errors) = run_dotnet(property_test_module(quote! {
            #[diplomat::attr(auto, getter = "value")]
            pub fn get_value(&self) -> Option<Box<Config>> {
                unimplemented!()
            }

            #[diplomat::attr(auto, setter = "value")]
            pub fn set_value(&self, value: Option<&Config>) {
                unimplemented!()
            }
        }));

        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );
        let config = files.get("Config.cs").expect("expected Config.cs output");
        assert!(
            config.contains("public Config? Value"),
            "expected a nullable property, got:
{config}"
        );
    }

    // A property is one member, so anything both accessors would document has to
    // be said once. Two methods sharing an error type is the case that used to
    // emit the tag twice.
    #[test]
    fn accessors_sharing_an_error_type_document_the_exception_once() {
        let (files, errors) = run_dotnet(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Config;

                #[diplomat::opaque]
                pub struct MyError;

                impl Config {
                    #[diplomat::attr(auto, getter = "value")]
                    pub fn get_value(&self) -> Result<u32, Box<MyError>> {
                        unimplemented!()
                    }

                    #[diplomat::attr(auto, setter = "value")]
                    pub fn set_value(&self, value: u32) -> Result<(), Box<MyError>> {
                        unimplemented!()
                    }
                }
            }
        });

        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );
        let config = files.get("Config.cs").expect("expected Config.cs output");
        assert_eq!(
            config
                .matches("/// <exception cref=\"MyErrorException\">")
                .count(),
            1,
            "both accessors throw the same exception, so document it once, got:
{config}"
        );
    }

    // The `<returns>` tag went missing from properties when the block was
    // copy-pasted into the two impl templates; it comes from the shared one now.
    #[test]
    fn an_opaque_property_documents_its_rust_allocated_return_once() {
        let (files, errors) = run_dotnet(property_test_module(quote! {
            #[diplomat::attr(auto, getter = "value")]
            pub fn get_value(&self) -> Box<Config> {
                unimplemented!()
            }

            #[diplomat::attr(auto, setter = "value")]
            pub fn set_value(&self, value: &Config) {
                unimplemented!()
            }
        }));

        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );
        let config = files.get("Config.cs").expect("expected Config.cs output");
        assert_eq!(
            config
                .matches("/// A <c>Config</c> allocated on Rust side.")
                .count(),
            1,
            "the property must carry its <returns> tag exactly once, got:
{config}"
        );
    }

    #[test]
    fn two_setters_on_one_property_name_are_rejected() {
        let (_files, errors) = run_dotnet(property_test_module(quote! {
            #[diplomat::attr(auto, setter = "size")]
            pub fn set_size(&self, value: usize) {
                unimplemented!()
            }

            #[diplomat::attr(auto, setter = "size")]
            pub fn set_size_again(&self, value: usize) {
                unimplemented!()
            }
        }));

        assert_eq!(
            errors.len(),
            1,
            "expected exactly one diagnostic: {errors:?}"
        );
        let error = &errors[0];
        for expected in ["`set` of property `Size`", "`set_size`", "`set_size_again`"] {
            assert!(
                error.contains(expected),
                "the diagnostic must name {expected}; got: {error}"
            );
        }
    }

    // A struct's fields are members too, and a property named after one is the
    // same CS0102 as two methods with one name.
    #[test]
    fn a_property_colliding_with_a_struct_field_is_rejected() {
        let (_files, errors) = run_dotnet(quote! {
            #[diplomat::bridge]
            mod ffi {
                pub struct Config {
                    size: u8,
                }

                impl Config {
                    #[diplomat::attr(auto, getter = "size")]
                    pub fn get_size(self) -> u8 {
                        unimplemented!()
                    }
                }
            }
        });

        assert_eq!(
            errors.len(),
            1,
            "expected exactly one diagnostic: {errors:?}"
        );
        assert!(
            errors[0].contains("two members named `Size`") && errors[0].contains("a struct field"),
            "the collision must name the field; got: {}",
            errors[0]
        );
    }

    // `#[diplomat::rename]` lands verbatim, after case conversion, so it is the
    // one thing that can produce a name case-folding never would.
    #[test]
    fn an_opaque_property_named_as_ffi_is_accepted() {
        let (files, errors) = run_dotnet(property_test_module(quote! {
            #[diplomat::attr(dotnet, rename = "AsFFI")]
            #[diplomat::attr(auto, getter)]
            pub fn handle(&self) -> u8 {
                unimplemented!()
            }
        }));

        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );
        let config = files.get("Config.cs").expect("expected Config.cs output");
        assert!(
            config.contains("public byte AsFFI"),
            "opaque wrappers do not generate AsFFI:\n{config}"
        );
    }

    #[test]
    fn a_struct_property_colliding_with_as_ffi_is_rejected() {
        let (_files, errors) = run_dotnet(quote! {
            #[diplomat::bridge]
            mod ffi {
                pub struct Config {
                    value: u8,
                }

                impl Config {
                    #[diplomat::attr(dotnet, rename = "AsFFI")]
                    #[diplomat::attr(auto, getter)]
                    pub fn handle(self) -> u8 {
                        unimplemented!()
                    }
                }
            }
        });

        assert_eq!(
            errors.len(),
            1,
            "expected exactly one diagnostic: {errors:?}"
        );
        assert!(
            errors[0].contains("two members named `AsFFI`"),
            "structs still generate AsFFI; got: {}",
            errors[0]
        );
    }

    // Case conversion runs first and the rename is applied to the result, so an
    // all-caps rename survives instead of being mangled to `UtcTime`. This is a
    // deliberate divergence from Dart, which case-folds after renaming.
    #[test]
    fn a_renamed_property_keeps_its_name_verbatim() {
        let (files, errors) = run_dotnet(property_test_module(quote! {
            #[diplomat::attr(dotnet, rename = "UTCTime")]
            #[diplomat::attr(auto, getter)]
            pub fn utc_time(&self) -> u64 {
                unimplemented!()
            }
        }));

        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join(
                "
"
            )
        );
        let config = files.get("Config.cs").expect("expected Config.cs output");
        assert!(
            config.contains("public ulong UTCTime"),
            "the rename must survive verbatim, got:
{config}"
        );
    }

    // `static_accessors` is off, so a static accessor is not a property here. It
    // has to stay a plain static method rather than silently disappear.
    #[test]
    fn a_static_accessor_stays_a_method() {
        let (files, errors) = run_dotnet(property_test_module(quote! {
            #[diplomat::attr(supports = static_accessors, getter)]
            pub fn origin() -> Box<Config> {
                unimplemented!()
            }
        }));

        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join("\n")
        );
        let config = files.get("Config.cs").expect("expected Config.cs output");
        assert!(
            config.contains("public static Config Origin()"),
            "expected a static method, got:
{config}"
        );
        assert!(
            property_getter_count(config) == 0,
            "a static accessor must not become a property, got:
{config}"
        );
    }

    // HIR does not require a getter to return anything, so this reaches the
    // backend: `void` is not a legal C# property type, and a property that reads
    // as nothing is not what the author meant either.
    #[test]
    fn a_getter_returning_nothing_is_rejected() {
        let (_files, errors) = run_dotnet(property_test_module(quote! {
            #[diplomat::attr(auto, getter)]
            pub fn size(&self) {
                unimplemented!()
            }
        }));

        assert_eq!(
            errors.len(),
            1,
            "expected exactly one diagnostic: {errors:?}"
        );
        assert!(
            errors[0].contains("a getter has to return something"),
            "the empty getter must be reported; got: {}",
            errors[0]
        );
    }

    // The P/Invoke layer is a plain method however the idiomatic layer presents
    // it, so the setter's `value` alias must stop at the property.
    #[test]
    fn a_setters_raw_declaration_keeps_the_rust_parameter_name() {
        let (files, errors) = run_dotnet(property_test_module(quote! {
            #[diplomat::attr(auto, setter = "size")]
            pub fn set_size(&self, new_size: usize) {
                unimplemented!()
            }
        }));

        assert!(
            errors.is_empty(),
            "unexpected diagnostics: {}",
            errors.join(
                "
"
            )
        );
        let raw = files
            .get("RawConfig.cs")
            .expect("expected RawConfig.cs output");
        assert!(
            raw.contains("SetSize(Config* handle, nuint newSize)"),
            "the raw declaration must keep the Rust parameter name, got:
{raw}"
        );
        let config = files.get("Config.cs").expect("expected Config.cs output");
        assert!(
            config.contains("Raw.Config.SetSize(selfLease!.Ptr, value)"),
            "the property must pass the implicit `value`, got:
{config}"
        );
    }

    // Properties come from the HIR accessor attributes, the same mechanism the
    // Dart and JS backends use — not from how a method happens to be named.
    #[test]
    fn get_and_set_named_methods_without_attributes_stay_methods() {
        let (files, _errors) = run_dotnet(property_test_module(quote! {
            pub fn get_size(&self) -> usize {
                unimplemented!()
            }

            pub fn set_size(&self, size: usize) {
                unimplemented!()
            }
        }));

        let config = files.get("Config.cs").expect("expected Config.cs output");
        assert!(
            config.contains("public nuint GetSize()"),
            "expected a plain GetSize method, got:
{config}"
        );
        assert!(
            !config.contains("public nuint Size"),
            "an unannotated method must not become a property, got:
{config}"
        );
    }
}
