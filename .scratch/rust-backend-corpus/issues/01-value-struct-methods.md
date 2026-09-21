# 01 — Inherent methods on value structs and enums

**Status:** DONE — landed in `e3a0304d`.

**What it built:** A provider that declares an inherent method on a value struct or a
value enum gets a working `impl` in the generated package. Associated functions,
`self`-by-value methods and (`&self`/`&mut self`) borrows are all handled by the
codegen; the borrow shapes are not reachable yet because the corpus gates them on
`struct_refs`/`mut_struct_refs` — see ticket 11.

**The finding that made it small:** a value type has **no ABI mirror**.
`ffi_value_name` names the same type the safe API uses, so a receiver is the value
itself or a plain pointer to it — no field-by-field conversion anywhere.
`ScalarPairWithPadding::assert_value` generates
`fn ScalarPairWithPadding_assert_value(this: super::ScalarPairWithPadding);`, which is
exactly what the C backend emits for the same method.

Four changes: the blanket `methods on value structs/enums` guards in `validate.rs`
became real method validation; `ffi_self_type` gained `SelfType::Struct`/`Enum` arms;
`emit_method` branches on the receiver owner; `gen/types.rs` and `gen/mod.rs` emit the
impl block and its extern declarations alongside the ones opaques already had.

**Delivered:** `ErrorStruct`, `StructWithAttrs`, `ScalarPairWithPadding`, `MyEnum`,
`DefaultEnum`, `CyclicStructB`, `BorrowedFields`, `BorrowedFieldsWithBounds` are
ungated. `CyclicStructB::get_a`/`get_a_option` stay gated individually — they return
`CyclicStructA`, which is gated for its nested field (ticket 03).

**Not delivered:** `MyStruct` and `PrimitiveStruct` are ungated by this ticket but
re-gated by ticket 02's finding (a `char` field needs an ABI mirror). `BorrowedFields`
and `BorrowedFieldsWithBounds` are ungated but still generate nothing: every one of
their methods is gated on `struct_refs`.
