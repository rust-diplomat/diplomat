# 10 — Audit the gates inherited from another backend

**What to build:** A triage pass over the gates that are *not* Rust-only decisions.

Of the 55 `rust` gates now in the corpus, 30 are Rust-only. The other 25 were already
disabling an item for another backend before this work, and the Rust gate was merged
into the existing condition rather than added fresh — 23 shared with dotnet, one with
kotlin, one with dart. Those are not necessarily Rust gaps at all: some were disabled
for a reason that is specific to that other backend's runtime (a GC, a `Span<T>`
length limit, a managed wrapper shape) and does not apply here.

Decide, per item, whether the Rust backend should support it, and either schedule the
work or record why the gate should stay.

**Blocked by:** None — can start immediately.

**Status:** ready-for-agent

- [ ] Every gate of the form `attr(any(<other backend>, rust), disable)` is reviewed
      and classified as "Rust should support this" or "correctly disabled"
- [ ] Each "should support" finding is either linked to an existing ticket or filed as
      one
- [ ] The corpus carries a comment wherever a shared gate is deliberately kept, naming
      the reason, so the next reader does not have to re-derive it
