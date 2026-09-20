# 07 — Nullable and borrowed opaque payloads in parameters and errors

**What to build:** Two related shapes where an opaque appears in a position the backend
currently refuses:

- a **nullable borrowed opaque parameter** — `OptionOpaque::option_opaque_argument(arg: Option<&OptionOpaque>)`
- a **borrowed opaque error payload** — `ResultOpaque::give_self(&'a self) -> Result<(), &'a Self>`

Both are about an opaque crossing as a borrow *inside* an optional or result wrapper,
rather than as a direct parameter or an owned return.

**Blocked by:** None — can start immediately.

**Status:** ready-for-agent

- [ ] `OptionOpaque::option_opaque_argument` and `ResultOpaque::give_self` are generated
      and their `rust` gates removed
- [ ] A consumer test passes `None` and `Some(&handle)`, and matches on both arms of the
      borrowed error
- [ ] `check.sh` passes
