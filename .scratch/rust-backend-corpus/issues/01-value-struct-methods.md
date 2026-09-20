# 01 — Inherent methods on value structs and enums

**What to build:** A provider that declares an inherent method on a value struct or a
value enum gets a working `impl` in the generated Rust package, callable from the
consumer. Covers the three receiver shapes the corpus uses: an associated function
returning `Self`, a `self`-by-value method, and a `&self`/`&mut self` method on the
`repr(C)` value type. Verifiable end to end by calling `MyStruct::new()`,
`MyEnum::into_value()`, `ScalarPairWithPadding::assert_value()` and
`BorrowedFields::extract_from_fields()` from a consumer test.

This is the only workstream with no ABI design question: a Rust consumer calling an
associated function on a value struct is ordinary Rust, and the `repr(C)` mirror
already exists.

**Blocked by:** None — can start immediately.

**Status:** ready-for-agent

- [ ] `ErrorStruct`, `StructWithAttrs`, `ScalarPairWithPadding`, `BorrowedFields`,
      `BorrowedFieldsWithBounds`, `MyEnum`, `DefaultEnum` are generated and their
      `rust` gates removed
- [ ] A consumer test calls an associated function, a `self`-by-value method and a
      `&self` method on a generated value struct, and a method on a generated enum
- [ ] These items drop out of `rg 'rust, disable' feature_tests/src`
- [ ] `feature_tests/rust/scripts/check.sh` passes

**Notes:** Several items are blocked by this *and* another ticket (`MyStruct`,
`PrimitiveStruct`, `BigStructWithStuff`, `CyclicStructA/B/C`,
`MyStructContainingAnOption`, `NestedBorrowedFields`, `StructWithSlices`,
`StructOfOpaque`, `TestMacroStruct`). They unblock as those land; only the list above
is in scope here.
