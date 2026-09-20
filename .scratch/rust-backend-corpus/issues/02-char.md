# 02 — `char` across the ABI

**What to build:** A provider that declares `char` in a field, parameter or return
round-trips as a Rust `char` in the generated API. `char` crosses the ABI as
`DiplomatChar`, a `u32` scalar, so the backend needs a split between the ABI spelling
(`u32`) and the safe spelling (`char`) wherever a primitive is mapped — slices, struct
fields, parameters, returns.

**Blocked by:** None — can start immediately.

**Status:** ready-for-agent

- [ ] `Opaque::assert_struct`, `OptionOpaqueChar::assert_char` and the `char` field in
      `MyStruct`/`PrimitiveStruct`/`OptionInputStruct` round-trip through a consumer test
- [ ] The validity policy for an out-of-range code point arriving from the ABI is
      written down where the conversion lives
- [ ] `check.sh` passes

**Decision needed before starting:** what should the generated conversion do when the
`u32` is not a valid Unicode scalar? The contract is a Rust-to-Rust ABI, so a
provider-authored `char` cannot produce one, but the wrapper must still be total. The
proposal is `char::from_u32(..).expect(..)` with a message naming the invariant —
a panic that is unreachable in contract. Alternatives are a lossy `U+FFFD` mapping
(silently corrupts) or threading `Result` through the field type (reshapes every
signature). Confirm the choice before implementing.

**Notes:** this is a near-refactor of the primitive mapping rather than a new feature,
so it touches every path that maps a primitive. Do it as one ticket — the whole blast
radius lives in the Rust backend's type mapping, not across crates — but verify by
regenerating the full corpus, not just the `char` call sites.
