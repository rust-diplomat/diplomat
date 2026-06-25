//! .NET backend for Diplomat.
//!
//! Generates C# bindings that call into the Diplomat-generated C ABI via
//! P/Invoke (`[DllImport]` externs with the `Cdecl` calling convention).
//! Opaque Rust handles map to an `IDisposable` partial class holding the
//! raw `Raw.T*` pointer (freed via the generated `Destroy` extern in
//! `Dispose`/the finalizer); slices and strings copy across the boundary;
//! callbacks are pinned via `GCHandle` on the managed side.
//!
//! This file is the entry point that the Diplomat CLI dispatches to. Codegen
//! itself lives in [`gen`] and naming/type-formatting concerns live in
//! [`formatter`].
//!
//! ## Borrowing / lifetime model
//!
//! The backend does not encode Rust lifetimes in C# types. It supports
//! call-scoped borrows — valid only for the duration of the single P/Invoke
//! call — and uses HIR lifetime-edge analysis to decide which borrowed outputs
//! can be documented and which must be rejected:
//!
//! * `&[u8]` / `&[u32]` / `&DiplomatStr` params are pinned with `fixed (...)`
//!   (or copied into a pinned `byte[]`) for the call and unpinned immediately
//!   after. If the Rust side stashes the pointer past the call, the C# GC may
//!   move or free the backing buffer, so any returned value borrowing from
//!   these temporary slice/string params is rejected with a diagnostic.
//! * Borrowed opaque **returns** and **errors** (`&T`, `&mut T`, `Option<&T>`,
//!   `Result<_, &E>`) are rejected outright with a diagnostic, because the
//!   generated `IDisposable` wrapper would `Destroy` a pointer Rust still
//!   owns (double-free). Return `Box<T>` / `Option<Box<T>>` instead.
//! * Lifetime-carrying owned returns (`Box<T<'a>>`) that borrow from `self` or
//!   another opaque wrapper are generated with XML lifetime remarks. C# cannot
//!   enforce the relationship, so the caller must keep the borrowed-from wrapper
//!   alive and undisposed while the returned value is used.

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

    let borrowed_return_targets = ctx.borrowed_return_targets();

    for (id, ty) in tcx.all_types() {
        if ty.attrs().disable {
            continue;
        }

        /*
         * Raw represents the layer of C# that directly manipulates the C ABI. It is expected to be unsafe and low-level, and is not intended for direct consumption by end-users.
         * The content layer represents the safe, idiomatic C# API that end-users will interact with.
         * It may wrap or compose multiple raw items, and should prioritize usability and safety.
         */
        // Compute the formatted name once: applies `#[diplomat::rename]`
        // and C# keyword escaping. The same name flows into the file
        // names, the type declaration sites (`public partial class T`),
        // and the type references (`Raw.T.Method`).
        let display_name = ctx.formatter.fmt_type_name(id).into_owned();
        // Attribute any diagnostic pushed while lowering this type (or its
        // methods) to `Type` / `Type::method`. Restored on scope exit.
        let _guard = ctx.errors.set_context_ty(display_name.clone().into());
        let lowered = match ty {
            diplomat_core::hir::TypeDef::Struct(struct_def) => {
                ctx.gen_struct(display_name.clone(), struct_def)
            }
            diplomat_core::hir::TypeDef::OutStruct(struct_def) => ctx.gen_out_struct(struct_def),
            diplomat_core::hir::TypeDef::Opaque(opaque_def) => {
                ctx.gen_opaque(display_name.clone(), opaque_def, &borrowed_return_targets)
            }
            diplomat_core::hir::TypeDef::Enum(enum_def) => {
                ctx.gen_enum(display_name.clone(), enum_def)
            }
            _ => {
                // No other type variants are expected to be emitted as top-level items, but
                // if we add any in the future, this will catch them and prevent silent
                // omissions.
                unreachable!("unexpected type variant: {id:?}");
            }
        };

        // A `None` means the type used an unsupported shape: the diagnostic
        // was already recorded, so skip emitting it. The end-gate in `lib.rs`
        // aborts the whole run (printing every collected diagnostic) before
        // any file is written, so a skipped type never ships partial output.
        let Some((raw, content)) = lowered else {
            continue;
        };

        let file_name = format!("{display_name}.cs");
        if let Some(raw) = raw {
            let raw_file_name = format!("Raw{display_name}.cs");
            add_cs_file(&files, raw_file_name, raw);
        }
        add_cs_file(&files, file_name, content);
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
            foo.contains("owned: false"),
            "borrowed return should build a non-owning wrapper:\n{foo}"
        );
        assert!(
            foo.contains("if (_owned)"),
            "a borrow-target wrapper should gate Destroy on _owned:\n{foo}"
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
}
