# .NET Backend

The .NET backend wraps Diplomat's C ABI in a C# library, generating two layers per type:
a `Raw` layer of `[LibraryImport]` P/Invoke declarations and unsafe pointer types, and an
idiomatic layer of safe, GC-friendly classes built on top of it. Consumers only interact
with the idiomatic layer.

To run the .NET backend you need to provide some configuration:
```sh
diplomat-tool -e {PATH_TO_LIB.RS} -c {CONFIG_FILE} --config {CONFIG_OVERRIDE_1} dotnet {OUTPUT_PATH}
```
The configuration consists of these options:
* `namespace` - the root .NET namespace for the generated bindings (e.g. `Icu4x`). Defaults
  to the crate's `lib_name`, upper-camel-cased.
* `dylib_name` (or `native_lib`) - the native library name passed to `LibraryImport`.
  Defaults to the crate's `lib_name`.
* `exception_trim_suffix` (or `exceptions.trim_suffix`) - suffix trimmed when deriving
  exception class names from error types, e.g. trimming `Error` so `FooError` becomes
  `FooException`.
* `exception_message_method` (or `exceptions.error_message_method`) - the method on an
  error type used to populate the generated exception's message, e.g. `ToDisplay`.
* `getters_prefix` (or `properties.getters_prefix`) - prefix identifying property getters,
  e.g. `get_`.
* `setters_prefix` (or `properties.setters_prefix`) - prefix identifying property setters,
  e.g. `set_`.
* `scaffold` - an optional binary value. If set to `true`, `diplomat-tool` will emit a
  `.csproj` scaffold next to the generated sources.

## Ownership and memory safety

Every opaque type is backed by a `RustHandle<T>` rather than a bare pointer. A handle
remembers who owns the underlying memory: an **owned** handle carries the Rust destructor
and runs it on release; a **borrowed** handle carries none, so releasing it is a no-op
because Rust still owns (and will free) that memory. This means methods returning `&T` or
`Option<&T>` are safe to wrap without risking a double-free.

Borrowed returns also carry a `_edges` array on the object rooting whatever it borrowed
from, so the GC can't collect the source object out from under a still-live borrowed
reference. `object[] _edges` is `Array.Empty<object>()` (no allocation) when a type has
no lifetime-carrying returns.

Consumers are not required to call `Dispose()`; a finalizer is the documented last-resort
cleanup path for owned handles. Native calls are followed by `GC.KeepAlive(this)` to
prevent the finalizer from running mid-call and freeing memory a P/Invoke is still using.

## Examples
The best way to learn to use the .NET backend is to first understand Diplomat generally
by reading this [book](../SUMMARY.md). Then look at the `example` and `feature_tests`
directories in the Diplomat project.
* Feature tests: [rust source](https://github.com/rust-diplomat/diplomat/tree/main/feature_tests/src/), [.NET usage](https://github.com/rust-diplomat/diplomat/tree/main/feature_tests/dotnet/Tests)
* Example: [rust source](https://github.com/rust-diplomat/diplomat/tree/main/example/src/), [.NET generated bindings](https://github.com/rust-diplomat/diplomat/tree/main/example/dotnet/Generated)

{{supports("dotnet")}}
