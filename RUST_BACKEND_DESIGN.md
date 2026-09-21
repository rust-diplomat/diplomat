# Experimental Safe Rust native-ABI backend

Status: implemented and exercised against Diplomat `main` at `84f57d78`, then rebased
onto `002db80c`.

This is specifically a **Safe Rust backend that consumes Diplomat's native
ABI**. It does not call the provider through the Rust ABI, depend on the
provider implementation crate, reconstruct provider allocations, or use C
headers/bindgen as a semantic intermediate.

```text
provider Rust implementation
        │ #[diplomat::bridge] native shims
        ▼
provider cdylib  ←──── private generated extern "C" declarations
                              ▲
                              │
                     public safe Rust API
                              ▲
                              │
                           consumer
```

## Current Diplomat pipeline

The provider and consumer sides are generated independently:

1. The `#[diplomat::bridge]` procedural macro in `macro/src/lib.rs` emits
   provider-side `extern "C"` functions and an owner-side destructor for every
   opaque type.
2. `diplomat-tool` parses and inlines the provider source, selects a
   `BackendAttrSupport`, lowers the source into `hir::TypeContext`, and
   dispatches a target in `tool/src/lib.rs`.
3. Each backend consumes HIR directly and returns `FileMap` plus
   `ErrorStore`. The top-level driver writes no files if any backend diagnostic
   exists.
4. HIR `Method` already carries the native `abi_name`, receiver, parameters,
   `ReturnType`, `LifetimeEnv`, attributes, and docs. `OpaqueDef` carries its
   `dtor_abi_name`. `TyPosition`, `MaybeOwn`, `Borrow`, `OpaquePath`, and
   `borrowing_param_visitor` preserve ownership, mutability, nullability, and
   lifetime-edge semantics.

The new target follows that architecture in `tool/src/rust/` and
`tool/templates/rust/`. `diplomat-tool rust` emits a complete Cargo package:

```text
Cargo.toml
build.rs
src/lib.rs                  # crate facade: module declarations and flat re-exports
src/ffi.rs                  # private raw layer: runtime ABI types and extern declarations
src/private.rs              # private: sealed capability traits + one unsafe UTF-8 rebuild helper
src/types.rs                # enums and value structs
src/opaques.rs              # `mod <type>;` + `pub use <type>::*;` for each opaque
src/opaques/<type>.rs       # one module per opaque: wrappers, sealed impls, Drop, methods
```

`[rust] crate-name` selects the generated package name. `[rust] dylib-name`
selects the `#[link]` name. `build.rs` has no dependencies and only turns
`DIPLOMAT_RUST_NATIVE_LIB_DIR` into a native link-search path.

## ABI types come from `diplomat-runtime`

The generated package declares one dependency:

```toml
diplomat-runtime = "0.16"
```

`DiplomatSlice`, `DiplomatSliceMut`, `DiplomatOwnedSlice`, and `DiplomatOption`
are the runtime's types, re-exported into the private `ffi` module. The backend
no longer transcribes them.

This keeps the property the experiment is about: the consumer reaches the
provider only through exported native symbols, never through the provider's
implementation crate. `diplomat-runtime` is ABI-support machinery, not the
provider, so consuming it does not weaken that boundary. The in-repo fixture
resolves the dependency against this checkout with a `[patch.crates-io]` entry in
`feature_tests/rust/Cargo.toml`; a real generated package resolves the published
crate, which is why the emitted dependency is a version requirement and not a
path.

Three consequences are deliberate:

- The hand-mirror is gone because a transcription with no layout check is a
  silent-UB hazard: nothing failed if the runtime's `repr(C)` layouts moved.
- A version skew between the generated package and the runtime it resolves is a
  live risk — but the mirror carried the same risk with none of the checking,
  since a hand-written copy has to be kept in step by hand.
- The old mirror constrained `DiplomatOption<T: Copy>`. The runtime type has no
  such bound, so the artificial `Copy` constraint that fork issue #11 is about
  no longer exists at the type level. What still limits `Option<T>` payloads is
  the backend's own value-type validation, not the ABI container.

One conversion cannot be expressed with the runtime's public API:
`DiplomatUtf8StrSlice`'s field is private, so a `DiplomatSlice<'a, u8>` cannot
become a `&'a str` by conversion alone. `src/private.rs` keeps a single
`unsafe fn utf8_str_from_slice` for that, carrying the assumption that the
provider sends valid UTF-8 — which the ABI type itself cannot express.

Because the runtime's slice types carry a lifetime, lifetimes have to be spelled
out where the local mirror needed none. A `repr(C)` mirror struct's field types
cannot elide them at all, and a borrowed return value cannot either (elision in
return position needs exactly one input lifetime, and a raw-pointer receiver
contributes none). So a borrowed slice return gets a named lifetime parameter on
its extern declaration, and a lifetime-bearing struct return names its own.

## What existing backends establish

The C backend is the closest description of the wire declarations, although
its output is not a safe consumer API. The C++ backend adds RAII and native
references on top of that ABI. Its type system can express much of ownership
and borrowing without a runtime object graph.

The .NET backend has the most explicit raw/safe split. Its public layer needs
`RustHandle<T>`, `BorrowLease<T>`, `BorrowKind`, lifetime-edge retention,
borrow versioning, pin holders, and `IDisposable` because the CLR type system
and GC do not enforce Rust's aliasing or deterministic lifetime rules. The
Rust backend retains HIR lifetime analysis but replaces that runtime machinery
with ordinary `&self`, `&mut self`, distinct borrowed wrapper types, and Rust
lifetimes. Allocation and destruction still cross only the native ABI.

## HIR mapping and safety argument

The “HIR/reuse” and “unsafe/invariant” columns explicitly record the four
questions required for each mapping: information already in HIR, reusable
Diplomat machinery, internal unsafe work, and the invariant making the public
API safe.

| Diplomat HIR semantic | Native ABI representation | .NET/C++ safe representation | Experimental Safe Rust representation | HIR/reuse | Internal unsafe operation and public invariant |
|---|---|---|---|---|---|
| `bool`, `DiplomatByte`, 8–64-bit integers, `isize`, `usize`, `f32`, `f64` | Corresponding C-compatible scalar | Native scalar conversions | Same Rust scalar (`DiplomatByte` becomes `u8`) | `Type::Primitive`, `PrimitiveType`; primitive formatters informed the mapping | Unsafe extern call only. Both sides use the exact scalar ABI type. Unsupported primitive kinds are rejected. |
| Owned opaque / `Box<T>` return | Non-null `T*` allocated by provider | .NET owning `RustHandle<T>`; C++ owning RAII wrapper | Non-cloneable `T { NonNull<ffi::T>, ... }` | Output `Type::Opaque`, `MaybeOwn::Own`, `OpaqueDef::dtor_abi_name` | Check null, construct private fields, later call provider destructor. Only one constructible owner exists and consumer never reconstructs `Box<T>`. |
| Nullable owned opaque | Nullable `T*` | Nullable managed/RAII wrapper | `Option<T>` | `OpaquePath::is_optional` plus owned `OpaqueOwner` | `NonNull::new(ptr).map(...)`. Null is `None`; non-null creates exactly one owner. |
| `&T` input | `const T*` | Handle/pointer extraction or C++ const reference | `&impl TSharedArg` | Input opaque `Borrow { mutability: Immutable, lifetime }`; reuse HIR lifetime | Private sealed trait extracts the pointer. Public capability trait has no methods and downstream crates cannot implement it. `T`, `TRef`, and `TRefMut` provide only shared capability. |
| `&mut T` input | `T*` | Exclusive lease or C++ mutable reference | `&mut impl TMutArg` | Input opaque mutable `Borrow`; `ParamSelf::get_mutability` | Private sealed trait extracts a mutable pointer. Only `T` and `TRefMut` implement it, and Rust enforces uniqueness of the argument borrow. |
| `&self` | `const T*` receiver | Managed shared borrow/lease; C++ const member | `&self` on `T`, `TRef<'_>`, and `TRefMut<'_>` | `ParamSelf`, `SelfType::Opaque`, immutable `Borrow` | Private `NonNull` is passed as const. No public mutable capability is created. |
| `&mut self` | `T*` receiver | Managed exclusive lease; C++ mutable member | `&mut self` on `T` and `TRefMut<'_>` only | Mutable `ParamSelf` | Private `NonNull` is passed as mutable. `TRef` does not receive mutable methods; borrow checking prevents aliases. |
| Shared borrowed opaque return | Non-owning non-null `const U*` | .NET non-owning handle with source lease/version; C++ reference | `URef<'a>` with `PhantomData<&'a ()>`, no `Drop` | Borrowed output `OpaqueOwner`, output lifetime, `Method::used_method_lifetimes`, `borrowing_param_visitor` | Check null then construct a non-owning view. Generated signature ties `'a` to the HIR-selected opaque input(s), so the view cannot outlive them. |
| Mutable borrowed opaque return | Non-owning non-null `U*` | Exclusive managed lease; C++ mutable reference | `URefMut<'a>` with `PhantomData<&'a mut ()>`, no `Drop`, no `Copy`/`Clone` | Same helpers plus mutable output `Borrow` | Construct non-owning exclusive view. The originating `&'a mut` remains borrowed for the view lifetime. |
| Nullable borrowed return | Nullable `const U*`/`U*` | Nullable non-owning handle | `Option<URef<'a>>` / `Option<URefMut<'a>>` | Output optionality, borrow owner/lifetime, lifetime edges | Null maps to `None`; a non-null view carries the real HIR lifetime, never `'static`. |
| Destructor | `T_destroy(T*)` exported by provider | `IDisposable`/finalizer or C++ destructor | `Drop for T` only | `OpaqueDef::dtor_abi_name` | One unsafe provider call. Borrowed wrappers have no `Drop`; provider allocation is never freed locally. |
| Simple enum | C-compatible enum value | Native target enum | `#[repr(C)]` Rust enum | `EnumDef`, variant discriminants, docs/renames; C layout informed mapping | Extern call passes by value. HIR guarantees provider-produced variants; the backend emits every variant and exact discriminant. |
| Simple struct | `repr(C)` value fields | Target value struct/class | `#[repr(C)]`, `Copy` struct with public primitive/enum fields | `StructDef`, field types/docs/renames | Passed/read by value. The backend rejects lifetimes, output-only structs, nested/owning fields, and anything outside the proven primitive/enum set. |
| Value `Option<T>` | `DiplomatResult<T, ()>`: `repr(C)` union plus bool tag | Target option/nullable abstraction over raw tagged union | Public `Option<T>`; `diplomat_runtime::DiplomatOption<T>` | `Type::DiplomatOption` and `ReturnType::Nullable`; reuse `diplomat-runtime`'s `DiplomatOption`/`DiplomatResult` | The runtime's `From<Option<T>>`/`From<DiplomatOption<T>>` conversions replace the earlier tag-checked read. Supported payloads are still value types, so no cross-library drop is introduced; the old mirror's `T: Copy` bound is gone. |
| Fallible return (`Result<T, E>`) | `DiplomatResult<T, E>`: `repr(C)` union plus bool tag | Backend-specific result or exception | `Result<SafeT, SafeE>` | `ReturnType::Fallible`; reuse `diplomat-runtime`'s `DiplomatResult<T, E>` | The runtime's `From<DiplomatResult<T, E>> for Result<T, E>` owns the tag check, so the generated code only converts each payload in its own arm. An owned opaque on either side becomes the owning wrapper, so its `Drop` frees the provider's allocation even when `?` discards the error. The success side is any shape a plain return accepts; `E` is `()`, a supported primitive, an owned opaque, or an enum/struct the provider marked with `#[diplomat::attr(auto, error)]`. |
| Writer (`SuccessType::Write`) | `DiplomatWrite*` out-parameter; ABI success is unit | .NET/`string`, C++/`std::string` | `String` / `Result<String, E>` / `Option<String>` | `SuccessType::Write`; `diplomat_runtime::rust_interop::RustWriteVec` | The wrapper constructs a `RustWriteVec`, passes `*mut DiplomatWrite` as the trailing ABI argument, and copies `as_bytes()` into a `String`. `DiplomatWrite` is not part of the public API. |
| Borrowed slices/strings (`&[T]`, `&mut [T]`, `&str`, `&DiplomatStr`, `&DiplomatStr16`) | `repr(C)` `{ptr, len}` pair (`DiplomatSlice`/`DiplomatSliceMut`) | Pins/leases in .NET; spans/views in C++ | `&'a [T]` / `&'a mut [T]` / `&'a str` / `&'a [u8]` / `&'a [u16]` | `Type::Slice(Slice::Primitive|Str)`, `Slice::lifetime`, `LifetimeEdgeKind::SliceParam`; C layout informed mapping | The runtime's `From<DiplomatSlice<T>>`/`From<DiplomatSliceMut<T>>` impls reconstruct the borrow after null/empty normalization (`nullptr` with `len == 0` maps to `&[]`). The return lifetime is taken from the HIR edge (receiver or slice parameter), never `'static`. |
| Slices of plain `repr(C)` value structs (`&[S]`, `&mut [S]`) | `DiplomatSlice<S>` / `DiplomatSliceMut<S>` | Backend-specific | `&'a [S]` / `&'a mut [S]` | `Type::Slice(Slice::Struct(MaybeOwn::Borrow(_), _))`, `abi_compatibles` | Both sides already emit the same `repr(C)` layout with all-`pub` fields, so the slice is a native `{ptr, len}` with no per-element call and no intermediate buffer. The element type must be a value struct (`abi_compatible`, no lifetimes); a lifetime-bearing struct carries a `PhantomData` field the provider does not have. `Box<[S]>` is rejected — it is an owned allocation and inherits the undecided allocator contract (#15). |
| Lifetime-bearing value struct (`BorrowedFields<'a>`, `BorrowedFieldsWithBounds<'a,'b:'a,'c:'b>`) | `repr(C)` struct of `{ptr,len}` fields | Pinned managed fields; C++ reference members | `pub struct Name<'a>` with `pub` reference fields (plus a private invariant `PhantomData`) | `StructDef::lifetimes`, per-field `Lifetime`, `LifetimeEdgeKind::StructLifetime`, `MaybeStatic`/`LifetimeEnv` bound graph | Converted field-by-field through the runtime slice conversions: `DiplomatSlice::from(slice)` on input, `.into()` on output. The `repr(C)` mirror struct carries the struct's lifetimes, so its field types name them explicitly. Each field's HIR lifetime index is mapped to the declaring struct's lifetime name; bounds are reproduced. Declaring and using such a struct is supported; **returning** one from a method is limited to a single output lifetime, because the return path accepts exactly one method lifetime. A method that borrows the struct it returns from two inputs (e.g. `fn both<'a, 'b>(&'a self, other: &'b Bar) -> Both<'a, 'b>`) is rejected — see the rejected list below. |
| Owned slice return (`Box<[u8]>`) | `DiplomatOwnedSlice<u8>` (ownership transferred) | Zero-copy `RustVec` in .NET | `Box<[u8]>` | `Slice::Primitive(MaybeOwn::Own, _)`, `owned_byte_slice_returns` capability gate | `Box::from(result)` over the runtime's `DiplomatOwnedSlice<u8>`, whose `From` impl reclaims the provider's allocation under Diplomat's owned-slice contract that provider and consumer share an allocator. Null+zero maps to an empty `Box`. |
| Slices of lifetime-bearing structs/strings/opaques (`&[Borrowed<'a>]`, `&[DiplomatStrSlice]`, `&[&Opaque]`) | Nested pointer/length shapes | Backend-specific | Rejected | `Slice::Struct` of a lifetime-bearing def, `Slice::Strs`, `Slice::Opaque` | Generated wrappers are not layout-compatible with the provider's type (a lifetime struct carries a `PhantomData` the provider does not), so an intermediate buffer or a different API shape is required. A plain `repr(C)` value struct is the exception: see the supported row above. |
| Callback/trait/free function/async | Function pointer, vtable, or standalone symbols | Backend-specific runtime/trampolines | Rejected | HIR variants and `TypeContext` iterators identify them | No guessed trampoline or lifetime behavior is generated. |

## Generated opaque capabilities

For each opaque `T`, the safe file generates:

- `T`: owned, private `NonNull`, non-cloneable, provider destructor in `Drop`;
- `TRef<'a>`: shared, non-owning, method set limited to shared methods;
- `TRefMut<'a>`: exclusive, non-owning, shared and mutable methods;
- public, methodless `TSharedArg` and `TMutArg` marker traits whose pointer
  operations live in a private sealing module;
- a hand-written `Debug` impl on all three wrappers that prints
  `TypeName(<addr>)`. A derive is not used: it would name the `pub(crate)`
  fields and lock the internal representation into the public format.
  `PartialEq` is not generated — identity comparison is already an explicit
  method (`Counter::same_identity`), and `==` would be easy to confuse with
  value equality.

Every wrapper carries `PhantomData<Rc<()>>`. Diplomat HIR currently has no
per-opaque thread-safety metadata, so the safe conservative answer is
`!Send + !Sync` for owned and borrowed wrappers. No `unsafe impl Send` or
`Sync` is generated.

Opaque types with type-level lifetimes become lifetime-parameterized wrappers:
`T<'a>`, `TRef<'view, 'a>`, and `TRefMut<'view, 'a>`. The type lifetimes are
threaded through the generated `Drop` impl, the sealed capability traits, and
every method, and an invariant `PhantomData<fn(&'a ()) -> &'a ()>` prevents
unsound variance. Method-level lifetimes come after the type-level ones in
HIR's `LifetimeEnv`, and only the lifetimes actually named in the generated
signature are declared on the method.

The generator reproduces the method lifetimes used by borrowed outputs and
their HIR bounds. It runs `borrowing_param_visitor`, accepts one simple
non-static output lifetime with direct opaque incoming edges, and rejects
slice/struct edges or larger output lifetime graphs. It never substitutes
`'static` and never erases a borrowed output lifetime.

## Exact implemented scope

Supported:

- `bool`, `DiplomatByte`, `i8`–`i64`, `u8`–`u64`, `isize`, `usize`, and `f32`/`f64`;
- opaque definitions with or without type-level lifetime parameters;
- static methods/constructors, `&self`, and `&mut self`;
- shared and mutable opaque parameters through sealed capabilities;
- owned and nullable-owned opaque returns;
- shared/mutable borrowed opaque returns, including nullable returns;
- borrowed slices and strings (`&[T]`, `&mut [T]`, `&str`, `&DiplomatStr`,
  `&DiplomatStr16`) as parameters and as returns, with the HIR lifetime edge
  re-materialized as a Rust lifetime;
- borrowed slices of plain `repr(C)` value structs (`&[S]`, `&mut [S]`) as
  parameters and as returns (`abi_compatibles`): one `DiplomatSlice<S>` across
  the ABI, then a native Rust loop, with no per-element call;
- owned `Box<[u8]>` returns (`owned_byte_slice_returns`), taking ownership of
  provider-allocated memory;
- `Result<T, E>` returns (`custom_errors`), where the success payload is any of the
  shapes above and `E` is one of `()`, a supported primitive, an **owned** opaque
  (`Box<T>`), or an enum/struct the provider marked with
  `#[diplomat::attr(auto, error)]`. Both payloads convert independently, so an owned
  opaque on either side becomes the owning wrapper — which is what frees the provider's
  allocation when a caller discards the error with `?`;
- simple enums;
- `repr(C)` value structs whose fields are supported primitives/enums;
- lifetime-bearing value structs whose fields are supported primitives/enums or
  borrowed slices (e.g. `BorrowedFields<'a>`), with per-field lifetime mapping;
- value `Option<T>` for supported value-type payloads (the old local mirror's `Copy` bound is gone);
- docs and applicable HIR renames;
- `Debug` on every opaque wrapper (`T`, `TRef`, `TRefMut`), printing
  `TypeName(<addr>)` without naming private fields, so `.expect` / `.unwrap_err`
  compile on a `Result` that names an opaque;
- invalid-name and generated-name collision diagnostics.

Explicitly rejected with contextual backend errors:

- unsupported primitives: `char` (it reaches the ABI as a `DiplomatChar`/`u32`, so
  accepting it would need a code-point validity decision), `Ordering` (no agreed ABI
  shape), and 128-bit integers (not FFI-safe on every target);
- nested/owning struct fields and output-only structs;
- owned slice *inputs* and owned slices of non-byte elements, including
  `Box<[S]>` for a value struct `S` (core rejects it at the AST with "Owned
  slices only support primitives"; the allocator contract is #15);
- slices of lifetime-bearing structs, strings-of-strings, and opaques, and
  `Option`/`Result` composition of owned slices;
- writers (`SuccessType::Write`), and every `Result` shape outside the subset above:
  a nullable owned opaque error (`Result<T, Option<Box<E>>>`, which has no single owner
  to destruct), a `#[diplomat::attr(auto, error)]`-marked struct that is not a plain
  value struct, and a custom enum or struct error that is not marked at all — the last
  gets a diagnostic naming the attribute, which is the same rule the other backends
  apply to a custom error type;
- complex lifetime graphs (multi-lifetime outputs, struct/slice element edges);
- callbacks, traits, async shapes, free functions, custom bindings, and
  cross-crate composition.

`BackendAttrSupport` is conservative. Fine-grained shapes that cannot be
expressed by its flags are checked during Rust generation. Any error makes
the backend return an empty `FileMap`; the top-level driver also refuses all
writes when diagnostics exist.

The declared flags are not the whole story — what backs each one matters. All eleven are
gated in the fixture with `#[diplomat::cfg(supports = ...)]`, so dropping a flag removes
the API and the consumer's test targets stop compiling:

| flag | gated fixture API |
|---|---|
| `constructors` | `Counter::from_value` (a plain `attr(auto, constructor)`) |
| `memory_sharing` | `Numbers::from_slice`, `Float64Vec::new` |
| `mutable_slices` | `Numbers::values_mut`, `Numbers::fill`, `Points::scale` |
| `named_constructors` | `Counter::with_value` (from `new_named`) |
| `option` | `Counter::add`, `Counter::maybe_snapshot` |
| `owned_byte_slice_returns` | `Bytes::make`, `Bytes::join` |
| `custom_errors` | `Counter::try_from_value`, `Counter::take` |
| `static_slices` | `Numbers::from_static` |
| `utf8_strings` | `Message::utf8_len` |
| `utf16_strings` | `WideMessage::new`, `WideMessage::units` |
| `abi_compatibles` | `Points::total`, `Points::new`, `Points::as_slice`, `Points::scale` |

The gating was verified by turning each flag off in turn, regenerating, and requiring
three things: generation still succeeds (the guard must *disable* an API, not raise a
diagnostic), the gated symbol disappears from the generated source, and
`cargo check --all-targets` on the consumer fails. `--all-targets` is load-bearing here:
the consumer's lib target references no gated API — every reference lives in
`tests/runtime.rs` — so a plain `cargo check` passes with all eleven flags off.

Two of the guards are not the obvious ones, and both were found by that experiment
rather than by reading the fixture:

- **`option` does not cover nullable owned opaques.** Lowering reads the flag only for
  `Option<struct/enum/primitive>` (`Type::DiplomatOption`); `Option<Box<Opaque>>` is a
  different arm that never consults it. Gating `Counter::maybe_new` on `option` would
  therefore have been a false claim, so the gated shapes are the value-type ones.
- **`named_constructors` cannot gate its own attribute.** The `auto` support check calls
  `BackendAttrSupport::check_string` with the *attribute path*, but that match's keys are
  the *flag* names — `named_constructors`, plural — while the path is
  `named_constructor`, singular. The lookup returns `None`, the check is skipped, and the
  attribute is applied even with the flag off: with `support.named_constructors = false`
  the generated `Counter` still exposed `with_value`. Nothing else in `core` reads that
  flag either, so an explicit `cfg` guard is the only thing that can gate it. The same
  spelling mismatch applies to every `attr(auto, ...)` whose path is not spelled exactly
  like a flag name (`constructor`, `comparison`, `stringifier`, `getter`, `setter`,
  `iterator`, `namespace`, `indexer`); `constructors` *is* read in `core`, but only to
  decide whether to diagnose a fallible constructor, not to gate `attr(auto, constructor)`.

`memory_sharing` is claimed because generated code borrows directly out of
provider-owned memory. Note the interaction with the rejected primitive set: the
shared `feature_tests/src` corpus gates `&[f64]` constructors on that same flag, so the
fixture's `Float64Vec` holds that exact shape — it failed outright until floats were
added to the primitive subset.

## Safety audit

Ownership: only an owned wrapper implements `Drop`. Its private constructor is
reachable only from an owned HIR return, and it invokes only the provider's
destructor. Leaking with `mem::forget` remains possible in safe Rust, as for
other owning Rust values, but double destruction is not exposed.

Borrowing and mutability: borrowed wrappers never gain destruction capability.
Shared wrappers have no mutable methods or mutable argument capability.
Exclusive wrappers are non-copyable and originate from an HIR mutable borrow.
Compile-fail fixtures prove a child cannot escape its parent, a shared view
cannot mutate or satisfy a mutable parameter, and two exclusive views cannot
coexist.

Nullability: raw pointer declarations remain nullable pointers. Every
non-null HIR return is checked before constructing `NonNull`; optional opaque
returns map through `NonNull::new` to `Option`.

Panic/unwind: current provider shims are ordinary non-unwinding `extern "C"`
functions and do not add `catch_unwind`. A provider panic therefore cannot
unwind into the consumer, but may abort the process. The consumer backend adds
no redundant catch layer and does not claim panic recovery.

Allocation: for provider-owned handles the consumer never names
`Box::from_raw`, `Vec::from_raw_parts`, or a local allocator — allocation and
deallocation stay in the provider cdylib, and borrowed views carry no `Drop`.
The single exception is the owned-slice return contract
(`owned_byte_slice_returns`): ownership of a provider-allocated `Box<[u8]>`
transfers to the consumer, so `Box::from(result)` converts the returned
`diplomat_runtime::DiplomatOwnedSlice<u8>` back into a `Box`, under Diplomat's
shared-allocator contract. Null with zero length normalizes to an empty `Box`.

## Fixture and binary evidence

`feature_tests/rust/` is a separate workspace:

- `provider`: real `#[diplomat::bridge]` crate configured as `cdylib` only;
- `generated`: the complete output of `diplomat-tool rust`, depending on
  `diplomat-runtime` by version (the fixture workspace patches that version to
  this checkout so the fixture builds against the local runtime);
- `consumer`: `#![forbid(unsafe_code)]`, depending only on `generated`, whose
  `tests/runtime.rs` and `tests/compile_fail.rs` carry the coverage as ordinary
  `#[test]` functions run by `cargo test`;
- `compile_fail`: standalone negative sources, excluded from normal workspace
  builds and driven case-by-case from `tests/compile_fail.rs`;
- `scripts/check.sh`: reproducible generation, `cargo test`, lint/format gates,
  and graph/binary evidence, including checks that the generated crate declares
  `diplomat-runtime` and defines no local ABI types.

The safe consumer creates one `Counter`, increments the same native object
three times, checks its value and identity through `CounterRef`, uses a
borrowed `ChildRef`, exercises shared/mutable opaque parameters, enums,
structs, options, and observes the provider-side atomic destructor count. It
also exercises the newer surface: `Numbers` (borrowed `&[u32]` read/write and a
`&mut [u32]` out-parameter), `Message` (`&[u8]` and `&str` borrows), `SliceView`
(an owned opaque carrying a type-level `'a` tied to the caller's buffer),
`FieldView<'a>` (a lifetime-bearing value struct returned and consumed by
value), `Bytes` (owned `Box<[u8]>` returns, including an empty buffer and a
return built from two borrowed slice parameters), and `Points` (a slice of
plain `repr(C)` `Point` values: one call to sum, a consumer-side loop over
`as_slice()`, and an in-place `&mut [Point]` scale). Opaque wrappers print as
`TypeName(<addr>)`, which is what lets `.expect` / `.unwrap_err` compile on a
`Result` that names an opaque.

Observed on macOS after a clean nested target:

```text
$ cargo tree --manifest-path feature_tests/rust/consumer/Cargo.toml
diplomat-rust-backend-consumer
└── diplomat-rust-backend-generated
    └── diplomat-runtime

$ cargo tree --manifest-path feature_tests/rust/generated/Cargo.toml
diplomat-rust-backend-generated
└── diplomat-runtime

$ nm feature_tests/rust/target/debug/libdiplomat_rust_backend_provider.dylib
... T _Counter_destroy
... T _Counter_new

$ nm -u feature_tests/rust/target/debug/diplomat-rust-backend-consumer
_Counter_destroy
_Counter_new

$ otool -L feature_tests/rust/target/debug/diplomat-rust-backend-consumer
.../libdiplomat_rust_backend_provider.dylib
```

The clean top-level artifact directory contains the consumer binary and the
provider dylib, but no provider rlib. The script also rejects any provider
constructor/destructor definition in the consumer. These checks establish
that the generated crate calls Diplomat native symbols and that the consumer
does not contain a statically linked provider implementation.

Verification completed for this implementation:

- `cargo fmt --all --check`;
- `cargo clippy --workspace --all-targets -- -D warnings` for the main workspace and
  for the generated/provider/consumer fixture workspace;
- all 134 `diplomat-tool` unit tests, including 23 Rust-backend generator tests;
- the complete `cargo test --workspace --no-fail-fast` unit and doc-test suite;
- `feature_tests/rust/scripts/check.sh`, including the safe runtime and
  compile-fail test suites (11 + 12 cases);
- every capability flag turned off in turn, with generation, generated-symbol and
  consumer `--all-targets` compilation checked each time (see the flag table above).

Cargo resolves `rustdoc` through PATH, and on the development host that served Homebrew
rustdoc 1.97.1 while `rustc` was rustup 1.96.0 (there is no rustdoc shim beside the rustc
shim), so every doc-test failed with `E0514`. Because `check.sh` stops at the first
error, that also silently skipped the format, lint, symbol and dynamic-dependency proofs
that follow it — the script reported a test failure while most of its evidence never ran.
`check.sh` now pins `RUSTDOC` to the toolchain cargo is using
(`$(rustc --print sysroot)/bin/rustdoc`) unless the caller set it, and says so when PATH
would have picked a different one.

## Assessment

Architecturally, this is natural as a “Safe Rust native-ABI backend,” not as a
Rust-to-Rust ABI backend. Primitive values, opaque ownership, receivers,
direct opaque parameters, nullability, and simple borrowed returns map cleanly
back into Rust. Rust's borrow checker and `Drop` eliminate the .NET runtime
borrow leases, source versions, refcounts, and `IDisposable` machinery for
this subset.

The unexpectedly difficult parts are not basic opaque calls but full HIR
generality. Type-level opaque lifetimes, borrowed slices/strings, and
struct-contained lifetime edges are now implemented: the hard part was building
a lifetime-graph emitter that maps HIR's `LifetimeEnv`/edge data onto Rust
generic parameters and bounds (and a `repr(C)`-to-reference bridge for struct
fields). Still remaining are owned slice returns of non-byte
elements, slices of structs/strings/opaques, multiple/transitive output lifetime
graphs, callbacks, writers, and arbitrary tagged-union payload ownership (`Result`
payloads are no longer on this list). HIR has enough
information for the implemented lifetime subset. The material missing datum is
a per-opaque `Send`/`Sync` contract; therefore conservative negative auto traits
are required.

Production quality would require shared ABI-lowering utilities rather than
parallel formatting, broader conformance/layout tests on all supported
platforms, richer name/module handling, semver and packaging policy, better
formatted generated source without an external rustfmt step, documentation
tests, and deliberate designs for the rejected HIR shapes. The ABI containers
are no longer duplicated — they are the runtime's — so what remains parallel is
the lowering logic around them, not the layout. The implementation
uses the real backend dispatch, config, HIR, diagnostics, and generated-package
patterns, so it is plausibly structured as the start of an upstream
contribution. That is not evidence that maintainers would accept its API or
scope unchanged.

Experiment 3 should keep this ABI boundary and add two independently built
Diplomat provider libraries: library A owns an opaque `Endpoint`; library B's
Diplomat API accepts that same native handle type without depending on or
statically linking A. It should determine how type identity/import metadata,
symbol ownership, destructor routing, and generated Rust crate composition
work across separately authored HIR contexts, while retaining the same graph,
symbol, lifetime, and exactly-once destruction proofs.
