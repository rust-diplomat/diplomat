# 09 — Slices of structs declared in another module

**What to build:** A slice whose element is a value struct declared in a different
bridge module generates — `MyString::take_slice_from_other_namespace(&[StructWithAttrs])`,
where the element type lives in the `attrs` module rather than the caller's.

**Blocked by:** 01 — the element type currently has methods and is gated for that
reason, so this cannot be verified until a slice-able element type exists ungated.

**Status:** ready-for-agent

- [ ] `take_slice_from_other_namespace` is generated and its `rust` gate removed
- [ ] The generated signature names the element type by a path that resolves from the
      consuming module
- [ ] `check.sh` passes
