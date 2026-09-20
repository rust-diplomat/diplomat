# 03 — Nested and optional value-struct fields

**What to build:** A value struct whose field is another value struct, or a
`DiplomatOption` of a primitive/enum/struct, round-trips by value. Today the backend
admits only primitives, enums and borrowed slices as fields.

Verifiable end to end: `CyclicStructA`/`CyclicStructC` (struct-inside-struct),
`BigStructWithStuff` (its `ScalarPairWithPadding` field),
`MyStructContainingAnOption` (`DiplomatOption<MyStruct>`,
`DiplomatOption<DefaultEnum>`) and `OptionInputStruct`
(`DiplomatOption<u8>`, `DiplomatOption<DiplomatChar>`, `DiplomatOption<OptionEnum>`)
all generate and round-trip.

**Blocked by:** 01 (methods on value structs), 02 (`char`, for `OptionInputStruct`'s
`DiplomatOption<DiplomatChar>` field).

**Status:** ready-for-agent

- [ ] `CyclicStructA`, `CyclicStructB`, `CyclicStructC`, `BigStructWithStuff`,
      `NestedBorrowedFields`, `MyStructContainingAnOption`, `OptionInputStruct`,
      `BorrowingOptionStruct` are generated and their `rust` gates removed
- [ ] A consumer test constructs a nested struct, reads a field back through the
      generated accessor, and round-trips each `DiplomatOption` arm
- [ ] `check.sh` passes
