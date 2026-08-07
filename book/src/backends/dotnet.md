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
* `scaffold` - an optional binary value. If set to `true`, `diplomat-tool` will emit a
  `.csproj` scaffold next to the generated sources.

## Properties

`#[diplomat::attr(auto, getter)]` and `#[diplomat::attr(auto, setter = "name")]` render as
C# properties. The accessor *is* the property — its body is generated inline, so no
separate method is emitted:

```rust
#[diplomat::attr(auto, getter)]
pub fn width(&self) -> u32 { self.0.width }
```
```csharp
public uint Width
{
    get
    {
        unsafe
        {
            /* ... */
        }
    }
}
```

A getter and a setter that resolve to the same property name are merged into one property
with both accessors. C# cannot hold two members of the same name, so this merging is
required — unlike Dart or JS, where a getter and setter are separate members.

Two cases do not merge:

* A setter with no matching getter becomes a **write-only** property.
* A getter and setter whose types disagree are rejected with a diagnostic naming both
  types, both Rust methods, and the marshal each side chose. A C# property has one type,
  and silently dropping the setter would hide the mistake — make the two agree. A
  byte-slice pair is the case you are most likely to hit: a `&'a [u8]` getter hands out a
  `DiplomatBorrowedSpan<byte>` view over Rust-owned memory, while a `&[u8]` setter takes a
  managed `byte[]` Rust only reads during the call. No single C# type serves both.

A getter must take `&self`. A `&mut self` getter is rejected, because reading it could
change the value: a property is read more than once — by a debugger watch, a serializer,
or just twice in a row — and a one-shot `self.field.take()` behind a property would drain
to null on the second read. Setters keep `&mut self`; assigning is the point.

Names must not collide either. A property that would share its name with a method, a
struct field, the type that contains it, or one of the members Diplomat always generates
(`AsFFI`, `FromFFI`, and opt-in `Dispose` on opaques) is rejected, because C# would not
compile the result.

A getter that returns an owned `Box<[u8]>` (`RustVec`) hands back a value you own, so
dispose it — `using var x = thing.Data;`. A getter returning an owned opaque can be used
the same way only when that opaque is opted into
`#[diplomat::attr(dotnet, manually_disposable)]`.

In accessor position a string-shaped parameter is always `string`, even for
`&DiplomatStr` (which is `byte[]` everywhere else, zero-copy and unvalidated). A property
cannot have one type for reading and another for writing, and every other backend already
maps `&DiplomatStr` to its string type — `std::string_view` in C++/Nanobind, `String` in
Dart, `string` in JS. Parameters outside accessors keep the zero-copy `byte[]` shape.

`#[diplomat::rename]` is applied after case conversion, so its value is used verbatim
(`#[rename = "UTCTime"]` stays `UTCTime`, not `UtcTime`) — matching how this backend names
methods and types. Static accessors are not supported.

## Ownership and memory safety

Every opaque type is backed by a `RustHandle<T>` rather than a bare pointer. A handle
remembers who owns the underlying memory: an **owned** handle carries the Rust destructor
and runs it on release; a **borrowed** handle carries none, so releasing it is a no-op
because Rust still owns (and will free) that memory. This means methods returning `&T` or
`Option<&T>` are safe to wrap without risking a double-free.

A handle that borrows from another wrapper (a method returning `&T`/`Option<&T>`, or an
owned-but-borrowing dependent) doesn't just root the source object for the GC — it retains
the source's shared native resource state (`RustHandleState<T>`) for as long as the
borrowing wrapper is reachable, and releases that retained reference from its own cleanup.
This is what lets the source wrapper be disposed (or finalized) *before* every dependent
has let go of it without either use-after-free or an early native destructor call: the
source's physical native destruction is deferred until its own wrapper reference **and**
every dependent's retained reference have all been released, however many dependents there
are or in whatever order their managed lifetimes end. `RustHandleState<T>` also holds any
of the wrapper's own pinned input buffers, unpinning them only after that wrapper's own
Rust destructor has actually run (which may itself be deferred by an outstanding dependent).

This reference count is a plain, lock-guarded `int`, not `SafeHandle` and not an atomic
`Interlocked` counter, and generated wrappers add zero synchronization to hot per-call code
(P/Invoke calls, property getters, etc.) — calling ordinary instance methods on the same
wrapper from two threads at once is still undefined behavior, exactly as for any other
non-thread-safe .NET type. The one thing that *is* synchronized is the small set of
lifecycle edges — a dependent retaining its source on construction, a wrapper releasing its
own reference (`Dispose()` or finalizer), and a dependent releasing its retained reference —
because those are the only points where two different threads can genuinely be doing
lifecycle work on the very same shared state at once: finalizers run concurrently with the
application on a dedicated finalizer thread regardless of how single-threaded the user's own
code is, so a dependent's finalizer and an explicit `source.Dispose()` call can race even in
otherwise single-threaded user code. See `RustHandle.cs.jinja`'s `RustHandleState<T>` doc
comments for the full synchronization contract.

By default, generated opaques are **finalizer-only**: no public `Dispose()`, cleanup runs
through a private idempotent path invoked by the finalizer. Add
`#[diplomat::attr(dotnet, manually_disposable)]` on an opaque type declaration to generate
`: IDisposable` plus a public `Dispose()` that runs the same cleanup and
`GC.SuppressFinalize(this)`. `Dispose()` requests/releases *this wrapper's own* ownership
reference exactly once, however many times or from however many threads it (or the
finalizer) ends up running — it does not necessarily trigger physical native destruction
immediately, since that's deferred while borrowers remain, and existing borrowers stay
valid after a source is disposed. What `Dispose()` *does* do immediately is make the
disposed wrapper itself reject further use: subsequent method calls or attempts to retain
a new dependent from it throw `ObjectDisposedException`, regardless of whether the
underlying native resource has actually been freed yet. In both modes, native calls are
followed by `GC.KeepAlive(this)` to prevent finalization while P/Invoke is still using the
pointer.

## String encoding

The backend supports both UTF-8 and UTF-16 strings, zero-copy wherever the C# and Rust
representations line up:

* `&DiplomatStr16` params and returns: a C# `string` is already a flat UTF-16 buffer, so
  these are always zero-copy — pinned directly with `fixed` (or, if the return value
  borrows it, via `ReadOnlyMemory<char>` + the same pinning holder slices use).
* `&DiplomatStr` params and returns (unvalidated UTF-8 — Rust places no validity
  requirement on the caller): treated exactly like `&[u8]`, so these are also zero-copy —
  `byte[]` / `ReadOnlyMemory<byte>` pinned directly, no transcoding.
* `&str` params (validated UTF-8 — Rust requires the caller to guarantee well-formed
  UTF-8, undefined behavior otherwise): a transcode from the UTF-16 `string` is
  unavoidable here. That copy is always routed through the explicitly-named
  `Diplomat.Utf8.Clone(...)` helper rather than inlined, so it stays visible in the
  generated source instead of hiding inside generic marshalling.

A borrowed string or slice return (`&'a str` / `&'a DiplomatStr` / `&'a DiplomatStr16` /
`&'a [u8]` / `&'a [u32]`) surfaces as `DiplomatBorrowedSpan<T>` — a zero-copy view over
memory Rust still owns, rooted with the same keep-alive-edge mechanism as a borrowed
opaque return. It intentionally does not expose a `Span`-returning property (nothing
would keep the view rooted once the span escaped it); call `WithSpan(...)` for scoped,
zero-copy, read-only access instead — the same pattern `RustVec` uses for owned returns
(see below). Producing an independent `T[]` is a separate, explicit step: call `Clone()`.

An owned `Box<[u8]>` return surfaces as `RustVec` — it owns the native allocation, is
`IDisposable`, and offers the same `WithSpan(...)` / `Clone()` shape as
`DiplomatBorrowedSpan<T>` (it deliberately avoids `MemoryManager<T>` for the same reason:
`GetSpan()`'s result wouldn't keep the owner alive). Other owned string/slice returns
(`Box<str>`, `Box<[T]>` for `T` other than `u8`) and `&[&str]` (`&[DiplomatStrSlice]`)
parameters aren't supported yet.

## Examples
The best way to learn to use the .NET backend is to first understand Diplomat generally
by reading this [book](../SUMMARY.md). Then look at the `example` and `feature_tests`
directories in the Diplomat project.
* Feature tests: [rust source](https://github.com/rust-diplomat/diplomat/tree/main/feature_tests/src/), [.NET usage](https://github.com/rust-diplomat/diplomat/tree/main/feature_tests/dotnet/Tests)
* Example: [rust source](https://github.com/rust-diplomat/diplomat/tree/main/example/src/), [.NET generated bindings](https://github.com/rust-diplomat/diplomat/tree/main/example/dotnet/Generated)

{{supports("dotnet")}}
