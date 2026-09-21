# 02 — `char` across the ABI

**Status:** DONE for by-value — scalar `char` in `28a15f53`, struct fields in the
ABI-mirror work. Slices of structs that contain `char` stay rejected.

**What landed:** `char` crosses as a `DiplomatChar` (`u32`), so the two sides need
different spellings — `u32` on the wire, `char` in the public API. `primitive_name`
split into an ABI name and a `safe_primitive_name`, converting both ways:
`ch as u32` out, `char::from_u32` back, as a hard error rather than a lossy fallback.

- Scalar: `OptionOpaqueChar::assert_char(&self, ch: char)` round-trips.
- Struct field: `MyStruct.f` is `char` to the consumer. The `extern` struct in
  `ffi` has `f: u32`. `MyStruct::new`, `into_a`, and `Opaque::assert_struct` convert
  at the boundary. Consumer test: `char_struct_field_is_a_char`.

**Decision:** public `char`, wire `u32`. Same rule as the rest of the safe layer;
do not leak `DiplomatChar`.

**Still gated:** `PrimitiveStruct` and `PrimitiveStructVec` slice methods
(`as_slice`, `as_slice_mut`, `get`, `push`, `take_in_slice`). A borrowed
`&[PrimitiveStruct]` cannot be zero-copy once the public element type has a `char`
validity invariant the provider's `DiplomatChar` does not. `char` slices stay
rejected for the same reason.

**Acceptance criteria:**

- [x] Scalar `char` round-trips
- [x] `MyStruct.f` is `char`; `Opaque::assert_struct` generates
- [x] `check.sh` passes
- [ ] Slices of char-containing structs (left for a later ticket)
