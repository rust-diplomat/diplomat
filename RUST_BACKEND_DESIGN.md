# Experimental Safe Rust native-ABI backend

Status: implemented and exercised against Diplomat `main` at `84f57d78`.

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
src/lib.rs   # safe public API; `mod ffi` is private
src/ffi.rs   # raw repr(C) mirrors and extern declarations
```

`[rust] crate-name` selects the generated package name. `[rust] dylib-name`
selects the `#[link]` name. `build.rs` has no dependencies and only turns
`DIPLOMAT_RUST_NATIVE_LIB_DIR` into a native link-search path.

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
| `bool`, `DiplomatByte`, 8–64-bit integers, `isize`, `usize` | Corresponding C-compatible scalar | Native scalar conversions | Same Rust scalar (`DiplomatByte` becomes `u8`) | `Type::Primitive`, `PrimitiveType`; primitive formatters informed the mapping | Unsafe extern call only. Both sides use the exact scalar ABI type. Unsupported primitive kinds are rejected. |
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
| Value `Option<T>` | `DiplomatResult<T, ()>`: `repr(C)` union plus bool tag | Target option/nullable abstraction over raw tagged union | Public `Option<T>`; private generated `DiplomatOption<T: Copy>` mirror | `Type::DiplomatOption` and `ReturnType::Nullable`; reuse layout from `runtime/src/result.rs` | Unsafe active-union read only after checking the tag. Supported payloads are `Copy` ABI values, avoiding cross-library drop or double-free. |
| Result/write | Tagged result or writer callback | Backend-specific result/exception/writer | Rejected | `ReturnType::Fallible`, `SuccessType::Write` identify it | No unsafe code is emitted. A diagnostic prevents all output until payload ownership and callback semantics are designed. |
| Borrowed slices/strings (`&[T]`, `&mut [T]`, `&str`, `&DiplomatStr`, `&DiplomatStr16`) | `repr(C)` `{ptr, len}` pair (`DiplomatSlice`/`DiplomatSliceMut`) | Pins/leases in .NET; spans/views in C++ | `&'a [T]` / `&'a mut [T]` / `&'a str` / `&'a [u8]` / `&'a [u16]` | `Type::Slice(Slice::Primitive|Str)`, `Slice::lifetime`, `LifetimeEdgeKind::SliceParam`; C layout informed mapping | `from_raw_parts` after null/empty normalization (`nullptr` with `len == 0` maps to `&[]`). The return lifetime is taken from the HIR edge (receiver or slice parameter), never `'static`. |
| Lifetime-bearing value struct (`BorrowedFields<'a>`, `BorrowedFieldsWithBounds<'a,'b:'a,'c:'b>`) | `repr(C)` struct of `{ptr,len}` fields | Pinned managed fields; C++ reference members | `pub struct Name<'a>` with `pub` reference fields (plus a private invariant `PhantomData`) | `StructDef::lifetimes`, per-field `Lifetime`, `LifetimeEdgeKind::StructLifetime`, `MaybeStatic`/`LifetimeEnv` bound graph | Converted field-by-field: `from_raw_parts` on output, `as_ptr`/`len` on input. Each field's HIR lifetime index is mapped to the declaring struct's lifetime name; bounds are reproduced. |
| Owned slice return (`Box<[u8]>`) | `DiplomatOwnedSlice<u8>` (ownership transferred) | Zero-copy `RustVec` in .NET | `Box<[u8]>` | `Slice::Primitive(MaybeOwn::Own, _)`, `owned_byte_slice_returns` capability gate | `Box::from_raw(slice_from_raw_parts_mut(ptr, len))`, matching Diplomat's owned-slice contract that the provider and consumer share an allocator. Null+zero maps to an empty `Box`. |
| Slices of structs/strings/opaques (`&[Struct]`, `&[DiplomatStrSlice]`, `&[&Opaque]`) | Nested pointer/length shapes | Backend-specific | Rejected | `Slice::Struct`, `Slice::Strs`, `Slice::Opaque` identify them | Generated wrappers are not layout-compatible with raw pointers, so an intermediate buffer or a different API shape is required. |
| Callback/trait/free function/async | Function pointer, vtable, or standalone symbols | Backend-specific runtime/trampolines | Rejected | HIR variants and `TypeContext` iterators identify them | No guessed trampoline or lifetime behavior is generated. |

## Generated opaque capabilities

For each opaque `T`, the safe file generates:

- `T`: owned, private `NonNull`, non-cloneable, provider destructor in `Drop`;
- `TRef<'a>`: shared, non-owning, method set limited to shared methods;
- `TRefMut<'a>`: exclusive, non-owning, shared and mutable methods;
- public, methodless `TSharedArg` and `TMutArg` marker traits whose pointer
  operations live in a private sealing module.

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

- `bool`, `DiplomatByte`, `i8`–`i64`, `u8`–`u64`, `isize`, and `usize`;
- opaque definitions with or without type-level lifetime parameters;
- static methods/constructors, `&self`, and `&mut self`;
- shared and mutable opaque parameters through sealed capabilities;
- owned and nullable-owned opaque returns;
- shared/mutable borrowed opaque returns, including nullable returns;
- borrowed slices and strings (`&[T]`, `&mut [T]`, `&str`, `&DiplomatStr`,
  `&DiplomatStr16`) as parameters and as returns, with the HIR lifetime edge
  re-materialized as a Rust lifetime;
- owned `Box<[u8]>` returns (`owned_byte_slice_returns`), taking ownership of
  provider-allocated memory;
- simple enums;
- `repr(C)` value structs whose fields are supported primitives/enums;
- lifetime-bearing value structs whose fields are supported primitives/enums or
  borrowed slices (e.g. `BorrowedFields<'a>`), with per-field lifetime mapping;
- value `Option<T>` for supported `Copy` ABI values;
- docs and applicable HIR renames;
- invalid-name and generated-name collision diagnostics.

Explicitly rejected with contextual backend errors:

- unsupported primitives (`char`, ordering, 128-bit integers, floats);
- nested/owning struct fields and output-only structs;
- owned slice *inputs* and owned slices of non-byte elements;
- slices of structs/strings/opaques and `Option`/`Result` composition of owned
  slices;
- fallible results and writers;
- complex lifetime graphs (multi-lifetime outputs, struct/slice element edges);
- callbacks, traits, async shapes, free functions, custom bindings, and
  cross-crate composition.

`BackendAttrSupport` is conservative. Fine-grained shapes that cannot be
expressed by its flags are checked during Rust generation. Any error makes
the backend return an empty `FileMap`; the top-level driver also refuses all
writes when diagnostics exist.

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

Allocation: the consumer never calls `Box::from_raw`, `Vec::from_raw_parts`,
or a local allocator for provider-owned data. Allocation and deallocation
stay in the provider cdylib.

## Fixture and binary evidence

`feature_tests/rust/` is a separate workspace:

- `provider`: real `#[diplomat::bridge]` crate configured as `cdylib` only;
- `generated`: the complete output of `diplomat-tool rust`;
- `consumer`: `#![forbid(unsafe_code)]`, depending only on `generated`, whose
  `tests/runtime.rs` and `tests/compile_fail.rs` carry the coverage as ordinary
  `#[test]` functions run by `cargo test`;
- `compile_fail`: standalone negative sources, excluded from normal workspace
  builds and driven case-by-case from `tests/compile_fail.rs`;
- `scripts/check.sh`: reproducible generation, `cargo test`, lint/format gates,
  and graph/binary evidence.

The safe consumer creates one `Counter`, increments the same native object
three times, checks its value and identity through `CounterRef`, uses a
borrowed `ChildRef`, exercises shared/mutable opaque parameters, enums,
structs, options, and observes the provider-side atomic destructor count. It
also exercises the newer surface: `Numbers` (borrowed `&[u32]` read/write and a
`&mut [u32]` out-parameter), `Message` (`&[u8]` and `&str` borrows), `SliceView`
(an owned opaque carrying a type-level `'a` tied to the caller's buffer), and
`FieldView<'a>` (a lifetime-bearing value struct returned and consumed by
value), and `Bytes` (owned `Box<[u8]>` returns, including an empty buffer and a
return built from two borrowed slice parameters).

Observed on macOS after a clean nested target:

```text
$ cargo tree --manifest-path feature_tests/rust/consumer/Cargo.toml
diplomat-rust-backend-consumer
└── diplomat-rust-backend-generated

$ cargo tree --manifest-path feature_tests/rust/generated/Cargo.toml
diplomat-rust-backend-generated

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
- all 115 `diplomat-tool` tests, including fourteen Rust-backend generator tests;
- the complete `cargo test --workspace --no-fail-fast` unit and doc-test suite;
- `feature_tests/rust/scripts/check.sh`, including the safe runtime and
  compile-fail test suites (8 + 12 cases);

The host PATH initially selected rustc 1.96 with Homebrew rustdoc 1.97, so the
workspace doc-test pass used the matching rustup rustdoc 1.96 explicitly. The
first run's failures were compiler metadata-version errors; all test binaries
had already passed, and the matched-toolchain rerun passed in full.

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
fields). Still remaining are owned slice returns, slices of
structs/strings/opaques, multiple/transitive output lifetime graphs, callbacks,
results, writers, and arbitrary tagged-union payload ownership. HIR has enough
information for the implemented lifetime subset. The material missing datum is
a per-opaque `Send`/`Sync` contract; therefore conservative negative auto traits
are required.

Production quality would require shared ABI-lowering utilities rather than
parallel formatting, broader conformance/layout tests on all supported
platforms, richer name/module handling, semver and packaging policy, better
formatted generated source without an external rustfmt step, documentation
tests, and deliberate designs for the rejected HIR shapes. The implementation
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
