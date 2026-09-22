# 12 — Recursive ABI mirrors for nested value structs

**Status:** DONE

**What to build:** Make nested value structs safe when an inner struct needs a
public/ABI split. A plain nested struct such as `CyclicStructA { a: CyclicStructB }`
can remain layout-identical, but an outer struct containing a `char`/`DiplomatOption`
struct must receive its own ABI mirror and recursive field conversions.

**Why this is a correctness ticket:** `tool/src/rust/type_map.rs::struct_needs_abi_mirror`
currently checks only the outer struct's direct fields (`char`, slices and
`DiplomatOption`). It does not inspect `Type::Struct` fields. The validator admits
nested structs recursively, so the generator can emit a public `#[repr(C)]` outer
struct containing safe `char`/`Option<T>` fields while declaring that same public
struct in the `extern` signature.

A minimal provider:

```rust
pub struct InnerChar { pub f: DiplomatChar }
pub struct InnerOption { pub x: DiplomatOption<u8> }
pub struct Outer { pub inner: InnerChar }
pub struct OuterOption { pub inner: InnerOption }
```

Currently `OuterOption` is rendered with `Option<u8>` in the public type but is
passed as `super::OuterOption` over the ABI instead of as an `ffi::OuterOption`
mirror containing `DiplomatOption<u8>`. This is not a stable ABI layout and skips
the `char`/option validity conversions. The same predicate also controls whether a
struct slice is treated as zero-copy, so a nested converting struct must not be
admitted as `&[Outer]`/`&mut [Outer]`.

**Scope:**

- Make mirror detection recursive through nested value-struct fields.
- Thread the recursive predicate through `ffi_value_name`, `ffi_value_type`,
  `is_supported_slice`, struct generation, method receivers/parameters/returns,
  and `convert_to_ffi` / `convert_from_ffi`.
- Keep layout-identical nested plain structs on the zero-copy path.
- For this ticket, reject nested lifetime-bearing structs with a contextual
  diagnostic unless the implementation also correctly maps their lifetime
  arguments through the enclosing struct. Do not silently emit `Inner` without
  its required generic arguments.
- Do not make `char`-containing or option-containing struct slices zero-copy;
  retain the existing explicit rejection until a converting-buffer API is designed.

**Tests / acceptance criteria:**

- [x] A generator test covers an outer struct containing a `char` struct and an
      outer struct containing an option struct. The ABI file contains mirrors for
      both outer and inner structs; extern signatures use the ABI mirror; generated
      call/return expressions recursively convert fields.
- [x] A generator test proves a plain nested `repr(C)` struct remains layout-identical
      and a slice of it remains accepted as zero-copy.
- [x] A nested lifetime-bearing struct is rejected with no partial `FileMap` output
      and a contextual diagnostic until lifetime remapping is implemented.
- [x] A consumer round-trip exercises the nested converting structs, including
      `Some`/`None` and a `char` field.
- [x] `feature_tests/rust/scripts/check.sh` passes.

**Blocked by:** None. This is a follow-up correctness fix for ticket 03.

**Implementation:** `struct_needs_abi_mirror` now recursively follows nested
value structs. Nested lifetime-bearing structs are rejected rather than emitted
with missing generic arguments. The generated fixture covers both nested
conversion and the layout-identical zero-copy path.
