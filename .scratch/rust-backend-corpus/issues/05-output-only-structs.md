# 05 — Output-only structs carrying owned opaques

**What to build:** A struct that is only ever returned by value — `OptionStruct` with
`Option<Box<OptionOpaque>>`, `Box<OptionOpaqueChar>`, `Box<OptionOpaque>` fields —
generates as a type the consumer can read, and the owned opaques inside it are
destructed exactly once when it drops.

The backend currently rejects `OutStruct` outright. This is the one shape where
ownership of several native allocations crosses at once, so the destructor pairing is
the thing to prove, not the field access.

**Blocked by:** None — can start immediately.

**Status:** ready-for-agent

- [ ] `OptionStruct` is generated and its `rust` gate removed
- [ ] `OptionOpaque::new_struct`, `new_struct_nones`, `returns` and
      `returns_option_input_struct` are generated
- [ ] A consumer test drops a returned struct and proves each owned opaque inside it
      was destroyed exactly once
- [ ] `check.sh` passes
