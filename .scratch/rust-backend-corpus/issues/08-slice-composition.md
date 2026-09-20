# 08 — Owned-slice and slice-of-slice composition

**What to build:** Slices composed with the containers the backend already handles:

- `Option<Box<[u8]>>` — `OwnedSliceReturn::maybe_make_bytes`
- `&[DiplomatStrSlice]` and `&[DiplomatStr16Slice]` — `MyString::new_from_first`,
  `MyString::new_from_utf16`

The backend already generates a bare owned `Box<[u8]>` return and a bare borrowed
string slice; what it refuses is composing either with `Option` or with a slice element.

**Blocked by:** None — can start immediately.

**Status:** ready-for-agent

- [ ] The three methods above are generated and their `rust` gates removed
- [ ] A consumer test covers `Some(Box<[u8]>)`, `None`, and a slice of two borrowed
      string slices in both encodings
- [ ] `check.sh` passes
