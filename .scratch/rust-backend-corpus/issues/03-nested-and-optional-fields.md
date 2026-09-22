# 03 — Nested and optional value-struct fields

**What to build:** A value struct whose field is another value struct, or a
`DiplomatOption` of a primitive/enum/struct, round-trips by value.

**Status:** DONE for by-value nested structs and `DiplomatOption` of values.
`NestedBorrowedFields` stays gated: it nests lifetime structs (`BorrowedFields`)
that are themselves still rust-disabled.

**What landed:**

- Nested layout-identical structs (`CyclicStructA`/`CyclicStructB`/`CyclicStructC`,
  `BigStructWithStuff` / `ScalarPairWithPadding`) are public fields with no extra
  wrapper; they share layout with the provider so `&[CyclicStructA]` stays zero-copy.
- `DiplomatOption<T>` fields become `Option<SafeT>` on the public type. The ABI
  field stays `DiplomatOption`. `char` inside an option still converts (`u32` on
  the wire).
- `MyStructContainingAnOption` converts the inner `MyStruct` (its `char` field)
  through the same mirror.
- `BorrowingOptionStruct` (`DiplomatOption<&'a DiplomatStr>` → `Option<&'a [u8]>`)
  generates.

Consumer test: `nested_and_optional_struct_fields_round_trip`.

**Acceptance criteria:**

- [x] `CyclicStructA`, `CyclicStructB`, `CyclicStructC`, `BigStructWithStuff`,
      `MyStructContainingAnOption`, `OptionInputStruct`, `BorrowingOptionStruct`
      generate; their rust gates removed
- [x] A consumer test constructs a nested struct, reads a field back, and
      round-trips each `DiplomatOption` arm
- [x] `check.sh` passes
- [ ] `NestedBorrowedFields` (blocked on ungating `BorrowedFields`)

## Comments

Follow-up correctness work is tracked separately:

- Ticket 12: recursive ABI mirrors for nested converting structs
- Ticket 13: safe conversion of `Result` error payloads
