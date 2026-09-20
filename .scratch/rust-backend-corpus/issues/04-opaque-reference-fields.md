# 04 — Opaque references as value-struct fields

**What to build:** A value struct carrying borrowed opaque handles —
`StructOfOpaque<'a> { i: &'a Opaque, j: &'a mut OpaqueMut }` — generates and round-trips,
including the exclusive borrow of the `&'a mut` field.

**Blocked by:** 01 (the type also has a `&'a mut self` method).

**Status:** ready-for-agent

- [ ] `StructOfOpaque` is generated and its `rust` gate removed
- [ ] A consumer test builds one from a shared and an exclusive handle and calls its
      `take_in` method
- [ ] `check.sh` passes
