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
owns one native pointer and, for an owned value, its destructor. A borrowed handle carries no
destructor, so cleaning it cannot free Rust-owned memory. This makes `&T`, `&mut T`, and
`Option<&T>` wrappers non-owning.

A dependent stores an ordinary managed reference to each source handle. That reference keeps
the handle reachable for garbage collection without adding native ownership. The generator
rejects a method whose returned value retains a borrow from an opaque marked
`#[diplomat::attr(dotnet, manually_disposable)]`. This covers borrowed views, borrowed slices,
owned lifetime-carrying children, and `Result`/`Option` success and error paths. The diagnostic
names the method and source and suggests an independent result or removing the attribute.

Temporary `&self`, `&mut self`, and opaque parameter borrows remain valid when the return does
not retain them. A manually disposable result may borrow from a non-disposable source. Cleaning
up a legal dependent releases its borrow bookkeeping and source-handle references; it never
disposes a source.

Borrow rules are separate from native ownership. Every value that borrows from a shared
source is a read view, whether Rust returned `&'a T`, `&'a [u8]`, or an owned `Box<T<'a>>`. It
lets go of the source's borrow when the creating call returns and remembers the source's
mutation version instead. A later `&mut self` call on the source succeeds and invalidates the
view; the view's next native call throws `InvalidOperationException`. This stands in for Rust's
rule that a borrow ends at its last use, which a garbage-collected runtime cannot observe.

A value born from an exclusive borrow (`&'a mut T`, or an owned `Box<T<'a>>` returned from
`&'a mut self` or an `&'a mut` parameter) is its source's only writer, so it keeps that borrow
until it is released. Every call on the source throws until then. The generator requires such a
returned type to be `manually_disposable`, because `Dispose()` is the only deterministic way to
end the borrow:

```rust
#[diplomat::opaque_mut]
pub struct Source(View);

#[diplomat::opaque_mut]
#[diplomat::attr(dotnet, manually_disposable)]
pub struct View;

impl Source {
    pub fn view_mut(&mut self) -> &mut View { &mut self.0 } // `View` must be disposable
}
```

A mutation attempt while an operation or `WithSpan` callback exposes a native pointer throws
instead of invalidating that pointer. Every handle is a `SafeHandle`; a scoped operation guard
protects the directly invoked value during its native call. Source edges validate borrow state
without recursively taking `SafeHandle` claims.

The generated API does not synchronize calls, mutation, or disposal across threads. Callers
must serialize access to every value in a dependency chain; racing these operations is outside
the supported contract and may cause undefined native behavior.

An owned return that borrows a managed slice or string parameter keeps that buffer pinned until
its handle is disposed or finalized. A type that also hands out borrows cannot be
`manually_disposable`, so its pin lasts until finalization.

A Rust destructor, including any custom `Drop`, must not read memory borrowed from another
opaque: the runtime may finalize the parent first, and this backend no longer keeps a parent
alive for a dependent's destructor. The generator cannot check this; keep it in review.

By default, generated opaques are **finalizer-only**: no public `Dispose()`, cleanup runs
through a private idempotent path invoked by the handle finalizer. Add
`#[diplomat::attr(dotnet, manually_disposable)]` on an opaque type declaration to generate
`: IDisposable` plus a public `Dispose()` that runs the same cleanup and
`GC.SuppressFinalize(this)`. Callers must not race `Dispose()` with calls from another thread.
After release, calls on the wrapper throw `ObjectDisposedException`. Methods that would expose
a retained borrow from this source are rejected during generation, so `Dispose()` is not a
parent-invalidation API. Native operations use scoped leases so the directly invoked handle
stays alive and its source edges remain reachable and validated through P/Invoke.

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
memory Rust still owns, rooted with managed source-handle edges. It validates those sources
before every raw access and implements `IDisposable` to release its edges. It intentionally
does not expose a `Span`-returning property; call `WithSpan(...)` for scoped, zero-copy,
read-only access instead. Producing an independent `T[]` is a separate, explicit step: call
`Clone()`.

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
