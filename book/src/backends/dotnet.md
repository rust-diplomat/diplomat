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
(`AsFFI` and `FromFFI` on structs; `Lease`, `Cleanup`, and `Dispose` on opaques) is rejected,
because C# would not
compile the result.

A getter that returns an owned `Box<[u8]>` (`RustVec`) hands back a value you own, so
dispose it — `using var x = thing.Data;`. A getter returning an owned opaque supports the
same pattern because every generated opaque implements `IDisposable`.

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

Every `RustHandle<T>` is a small reference-counted class: pointer, destructor, edges,
lifetime references, and borrow state live in exactly one place. Construction starts the reference
count at 1 (the owning wrapper). Each `BorrowLease` adds one reference and a shared or exclusive
mode. A returned borrowed value takes over that lease: shared views convert it to a versioned
token (`TransferVersioned()`), exclusive views keep the exclusive lease until their bare
wrapper is disposed, and owned-borrowing returns keep the transferred lease until cleanup.
Physical native destruction waits until the last reference is gone, in whatever order managed
lifetimes end. Cleanup always runs the native destructor first, then disposes every edge
(pins and borrow tokens).

The reference count is updated with `Interlocked` so a user-thread retain can
race a finalizer-thread token release on the same handle without lost updates.
Teardown still runs only on the thread that drives the count to zero.

Every opaque keeps a finalizer as a fallback. Add
`#[diplomat::attr(dotnet, manually_disposable)]` to opt into `IDisposable`. The .NET
generator requires the attribute on a returned opaque in these cases:

1. A method returns a mutable borrow. Only `Dispose()` ends the exclusive borrow.

   ```rust
   #[diplomat::opaque_mut]
   #[diplomat::attr(dotnet, manually_disposable)]
   pub struct View;

   #[diplomat::opaque_mut]
   pub struct Source(View);

   impl Source {
       pub fn view_mut(&mut self) -> &mut View { &mut self.0 }
   }
   ```

2. A method returns an owned opaque that borrows from the receiver or another opaque parameter. `Dispose()` releases the returned wrapper's borrow and lifetime reference.

   ```rust
   #[diplomat::opaque]
   pub struct Parent;

   #[diplomat::opaque]
   #[diplomat::attr(dotnet, manually_disposable)]
   pub struct Child<'a>(&'a Parent);

   impl Parent {
       pub fn child<'a>(&'a self) -> Box<Child<'a>> { Box::new(Child(self)) }
   }
   ```

3. A returned view retains an opaque that is manually disposable or is required to be
   manually disposable. The rule is transitive, so every returned view in the retention
   chain must carry the attribute.

   ```rust
   #[diplomat::opaque]
   #[diplomat::attr(dotnet, manually_disposable)]
   pub struct Source(View);

   #[diplomat::opaque]
   #[diplomat::attr(dotnet, manually_disposable)]
   pub struct View;

   impl Source {
       pub fn view(&self) -> &View { &self.0 }
   }
   ```

4. An owned opaque return borrows a managed slice or string input. The wrapper keeps that
   managed memory pinned until `Dispose()` unpins it.

   ```rust
   #[diplomat::opaque]
   #[diplomat::attr(dotnet, manually_disposable)]
   pub struct Parser<'a>(&'a [u8]);

   impl Parser<'_> {
       pub fn parse<'a>(data: &'a [u8]) -> Box<Parser<'a>> {
           Box::new(Parser(data))
       }
   }
   ```

A plain owned opaque with no borrowed data does not require the attribute. A shared view of
a source that is not manually disposable does not require it either. You may still opt in
when no rule requires deterministic cleanup.

`Dispose()` runs the same private idempotent cleanup path as the finalizer and calls
`GC.SuppressFinalize(this)`. It releases this wrapper's ownership reference but does not
necessarily destroy the native value immediately: existing borrowers keep it alive and
remain valid. The disposed wrapper itself rejects further use with
`ObjectDisposedException`. Native calls are followed by `GC.KeepAlive(this)` to prevent
finalization while P/Invoke is still using the pointer.

Shared versioned views do not keep their source borrowed between calls: a mutable source
call can proceed and invalidates the old view. Exclusive views are bare wrappers, and owned
values that borrow from a source keep a real source borrow until disposed. A conflicting
source mutation throws `InvalidOperationException` until that view or dependent is disposed.
Dispose these values deterministically rather than waiting for the GC if the source needs to
be mutated again. In particular, dispose owned-borrowing returns before mutating their source.

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
memory Rust still owns. Like a shared opaque view, it holds a versioned lifetime reference
and implements `IDisposable`; `Dispose()` releases that reference without waiting for GC. A later
mutable call on the source invalidates the view: the next `WithSpan(...)` or `Clone()`
throws `InvalidOperationException`. It intentionally does not expose a `Span`-returning
property (nothing would keep the view rooted once the span escaped it); call
`WithSpan(...)` for scoped, zero-copy, read-only access instead — the same pattern
`RustVec` uses for owned returns (see below). Producing an independent `T[]` is a
separate, explicit step: call `Clone()`.

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
