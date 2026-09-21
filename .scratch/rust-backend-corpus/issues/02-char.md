# 02 — `char` across the ABI

**Status:** PARTLY DONE — scalar `char` landed in `28a15f53`. `char` inside a value
struct is blocked and needs a decision.

**What landed:** `char` crosses as a `DiplomatChar` (`u32`), so the two sides need
different spellings — `u32` on the wire, `char` in the public API. `primitive_name`
split into an ABI name and a `safe_primitive_name`, converting both ways:
`ch as u32` out, `char::from_u32` back, as a hard error rather than a lossy fallback.
`OptionOpaqueChar::assert_char(&self, ch: char)` generates and round-trips.

**What is blocked, and why it is now measured rather than assumed:** `char` in a
**value struct field** does not work. A value type has no ABI mirror — the generated
struct *is* the extern signature's type — so a `char` field makes the ABI type carry a
`char`, and rustc reports `improper_ctypes`:

```
warning: `extern` block uses type `char`, which is not FFI-safe
418 |  pub(super) fn PrimitiveStruct_mutable_slice(a: DiplomatSliceMut<super::PrimitiveStruct>);
    = help: consider using `u32` or `libc::wchar_t` instead
    = note: the `char` type has no C equivalent
```

`MyStruct` (field `f`) and `PrimitiveStruct` (field `b`) are re-gated on exactly that,
with the reason written on the gate.

**Decision needed to finish:** build an ABI mirror for value structs that carry a
`char`, or expose the field as `u32` and skip the mirror.

- A mirror makes `char` fields, `&Struct` and by-value passing all work, but
  introduces "the safe type is not the ABI type" — the machinery this backend has
  deliberately avoided, and the reason ticket 01 was small.
- Exposing `u32` needs no mirror but leaks the wire representation into the public API.

Neither is obviously right, which is why it is a decision and not a task.

**Blocked by this:** 8 corpus items — `MyStruct`, `PrimitiveStruct`,
`Opaque::assert_struct`, `OptionOpaqueChar::assert_char` (done), and the
`PrimitiveStructVec` methods `as_slice`, `as_slice_mut`, `get`, `push`,
`take_in_slice`.

**Note:** `char` slices stay rejected. `slice_element_ty` serves both the ABI and the
safe spelling, the corpus exercises no `char` slice, and splitting it is unproven work.
