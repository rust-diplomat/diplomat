//! .NET backend for Diplomat.
//!
//! Generates C# bindings that call into the Diplomat-generated C ABI via
//! P/Invoke (`[DllImport]` externs with the `Cdecl` calling convention).
//! Opaque Rust handles map to `IDisposable` partial classes backed by
//! `RustHandle<T>`, which records whether C# or Rust owns the pointer. Slices
//! and strings copy across the boundary; callbacks are pinned via `GCHandle`.
//!
//! This file is the entry point that the Diplomat CLI dispatches to. Codegen
//! itself lives in [`gen`] and naming/type-formatting concerns live in
//! [`formatter`].
//!
//! ## Borrowing / lifetime model
//!
//! The backend does not encode Rust lifetimes in C# types. It uses HIR
//! lifetime-edge analysis to root supported borrowed outputs and reject the
//! unsafe cases:
//!
//! * `&[u8]` / `&[u32]` / `&DiplomatStr` params are pinned with `fixed (...)`
//!   (or copied into a pinned `byte[]`) for the call and unpinned immediately
//!   after. When an owned opaque success return borrows a `&[u8]` / `&[u32]`
//!   param, that param instead surfaces as `ReadOnlyMemory` and is pinned via
//!   `DiplomatPinnedMemory`, rooted as a keep-alive edge and unpinned after the
//!   Rust destructor runs. String params and other borrow positions (borrowed
//!   errors, Option-wrapped or non-opaque returns) are still rejected with a
//!   diagnostic. Because `ReadOnlyMemory` / `MemoryHandle` need the
//!   `System.Memory` package on the netstandard2.0 / .NET Framework floor, the
//!   `DiplomatPinnedMemory` helper and its `Dispose` sweep are emitted only when
//!   a run actually pins (see `uses_pinned_memory`), so runs that never borrow a
//!   slice don't inherit the dependency.
//! * Borrowed opaque returns (`&T`, `&mut T`, `Option<&T>`) use non-owning
//!   handles plus keep-alive edges.
//! * Borrowed opaque errors (`Result<_, &E>`) are rejected; without a success
//!   arm to carry keep-alive edges, `Dispose` would call `Destroy` on a pointer
//!   Rust still owns (double-free).
//! * Lifetime-carrying owned returns (`Box<T<'a>>`) from opaque wrappers get
//!   XML lifetime remarks.

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

/// `DiplomatWriteable` — caller-provided buffer Rust appends UTF-8 bytes
/// into. Carries function pointers for `flush` and `grow` callbacks so
/// Rust can ask C# to enlarge the buffer when it runs out. Used for
/// every "return string" API on the Rust side (`fn foo(&self, write: &mut DiplomatWrite)`).
#[derive(Template)]
#[template(path = "dotnet/DiplomatWriteable.cs.jinja", escape = "none")]
struct DiplomatWriteableTemplate<'a> {
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
    a.utf16_strings = false;
    // `option` and `mutable_slices` are advertised but coverage is
    // narrower than the flag suggests:
    //   - `mutable_slices`: only `&mut [DiplomatByte]`, `&mut [u8]`, and
    //     `&mut [u32]` lower today; other primitive element types report a
    //     diagnostic in the slice-primitive arm of `gen::method::lower_input`.
    //   - `option`: works for primitive / enum / struct success values;
    //     unsupported non-primitive struct fields report a diagnostic during
    //     struct codegen.
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
    a.accessors = false;
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
    /// Prefix identifying property getters, e.g. `get_`.
    pub getters_prefix: Option<String>,
    /// Prefix identifying property setters, e.g. `set_`.
    pub setters_prefix: Option<String>,
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
            "getters_prefix" | "properties.getters_prefix" if value.is_str() => {
                self.getters_prefix = value.as_str().map(str::to_string);
            }
            "setters_prefix" | "properties.setters_prefix" if value.is_str() => {
                self.setters_prefix = value.as_str().map(str::to_string);
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
        getters_prefix: config.dotnet_config.getters_prefix.as_deref(),
        setters_prefix: config.dotnet_config.setters_prefix.as_deref(),
        result_struct_registry: std::cell::RefCell::new(std::collections::HashMap::new()),
        option_struct_registry: std::cell::RefCell::new(std::collections::HashMap::new()),
        callback_struct_registry: std::cell::RefCell::new(std::collections::HashMap::new()),
    };

    /*
     * Raw represents the layer of C# that directly manipulates the C ABI. It is expected to be unsafe and low-level, and is not intended for direct consumption by end-users.
     * The content layer represents the safe, idiomatic C# API that end-users will interact with.
     * It may wrap or compose multiple raw items, and should prioritize usability and safety.
     */
    let (uses_pinned_memory, uses_owned_byte_slice_return, rendered_types) = ctx.render_all_types();
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
        "DiplomatWriteable.cs".to_string(),
        DiplomatWriteableTemplate {
            namespace: &namespace,
        }
        .render()
        .expect("DiplomatWriteable template render failed"),
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
        "RustHandle.cs".to_string(),
        RustHandleTemplate {
            namespace: &namespace,
        }
        .render()
        .expect("RustHandle template render failed"),
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

        let mut attr_validator = BasicAttributeValidator::new("dotnet_test");
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
                for (_cx, err) in e {
                    eprintln!("Lowering error: {err}");
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

        let mut attr_validator = BasicAttributeValidator::new("dotnet_test");
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
            Err(e) => e.into_iter().map(|(_cx, err)| err.to_string()).collect(),
        }
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
            foo.contains(".Borrowed("),
            "borrowed return should build the wrapper via the non-owning Borrowed factory:\n{foo}"
        );
        assert!(
            foo.contains("RustHandle<Raw.Foo>") && foo.contains("_inner.Release()"),
            "a borrow-target wrapper should carry ownership in the handle and free via Release:\n{foo}"
        );
        assert!(
            !foo.contains("_owned"),
            "the ownership flag field should be gone — ownership lives in the handle:\n{foo}"
        );
    }

    #[test]
    fn lifetime_carrying_owned_return_borrowing_slice_input_is_rejected() {
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
            list.contains("new Parsed(result.Ok, new object[] { dataPin })"),
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

    // Rust's Drop may still read the buffer, so the unpin lives in the
    // wrapper's Dispose after Release() — never in a holder finalizer.
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
            foo.contains("new Foo(result, new object[] { dataPin })"),
            "infallible owned return should root the pin holder as an edge:\n{foo}"
        );
        let release_at = foo
            .find("_inner.Release();")
            .expect("Dispose should release the Rust handle");
        let unpin_at = foo
            .find("(edge as DiplomatPinnedMemory)?.Dispose();")
            .expect("Dispose should unpin holder edges");
        assert!(
            release_at < unpin_at,
            "unpin must run AFTER Release() so Rust's Drop can still read the buffer:\n{foo}"
        );
    }

    // The pin edge lands on the RETURNED type's wrapper, so the Dispose sweep
    // must exist on every opaque, not just those with pinning methods (#1194).
    #[test]
    fn cross_type_pinned_return_unpins_on_dispose() {
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
            product.contains("(edge as DiplomatPinnedMemory)?.Dispose();"),
            "a type returned pinned from another type's method must sweep pin edges on Dispose:\n{product}"
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
            pair.contains("new Pair(result, new object[] { aPin, bPin })"),
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
            builder.contains("stays pinned until the returned value is disposed; do not mutate it"),
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
    // dependent pin helper, nor the Dispose sweep that references it — the
    // netstandard2.0 floor would fail to compile it.
    #[test]
    fn run_without_pinning_omits_pin_helper_and_sweep() {
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
        let plain = files.get("Plain.cs").expect("expected Plain.cs output");
        assert!(
            !plain.contains("(edge as DiplomatPinnedMemory)"),
            "no pinned return means the Dispose sweep must be absent:\n{plain}"
        );
    }

    // A run WITH a pinned-slice return ships the helper and the sweep on every
    // opaque (cross-type: the sweep must be present regardless of which type
    // declares the pinning method).
    #[test]
    fn run_with_pinning_emits_pin_helper_and_sweep() {
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
            viewer.contains("(edge as DiplomatPinnedMemory)?.Dispose();"),
            "a pinned return should emit the Dispose sweep:\n{viewer}"
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
                "throw new BorrowingErrorException(new BorrowingError(result.Err, new object[] { this }), this);"
            ),
            "error path should pass the receiver edge to the inner error and exception:\n{owner}"
        );

        let exc = files
            .get("BorrowingErrorException.cs")
            .expect("expected BorrowingErrorException.cs output");
        assert!(
            exc.contains("params object[] edges"),
            "exception class should accept keep-alive edges in its constructor:\n{exc}"
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
                "throw new BorrowingErrorException(new BorrowingError(result.Err, new object[] { this }), this);"
            ),
            "error path should pass the receiver edge to the inner error and exception:\n{owner}"
        );

        let exc = files
            .get("BorrowingErrorException.cs")
            .expect("expected BorrowingErrorException.cs output");
        assert!(
            exc.contains("params object[] edges"),
            "exception class should accept keep-alive edges in its constructor:\n{exc}"
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
            errors[0].contains("except for top-level `Box<[u8]>` method returns"),
            "unexpected diagnostic: {}",
            errors[0]
        );
    }

    // `Option<Box<[u8]>>` is rejected at HIR-lowering time: the new arm in
    // `core::hir::lowering` requires `!in_result_option`, so an optioned
    // owned slice falls through to the pre-existing rejection.
    #[test]
    fn optional_owned_byte_slice_return_is_rejected() {
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

        let errors = lowering_errors(tk_stream, true);
        assert_eq!(errors.len(), 1, "unexpected diagnostics: {errors:?}");
        assert!(
            errors[0].contains("except for top-level `Box<[u8]>` method returns"),
            "unexpected diagnostic: {}",
            errors[0]
        );
    }

    // `Result<Box<[u8]>, E>` must stay rejected: the macro leaves the ok arm
    // as a raw `Box<[u8]>` fat pointer inside `DiplomatResult` (it only
    // converts to the repr(C) `DiplomatOwnedSlice<u8>` for a plain top-level
    // return), so the result union's layout would not be FFI-stable.
    #[test]
    fn fallible_owned_byte_slice_return_is_rejected() {
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

        let errors = lowering_errors(tk_stream, true);
        assert_eq!(errors.len(), 1, "unexpected diagnostics: {errors:?}");
        assert!(
            errors[0].contains("except for top-level `Box<[u8]>` method returns"),
            "unexpected diagnostic: {}",
            errors[0]
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
                    .any(|e| e.contains("except for top-level `Box<[u8]>` method returns")),
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
}
