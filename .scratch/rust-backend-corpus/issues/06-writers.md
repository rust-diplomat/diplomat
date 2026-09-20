# 06 — Writer methods (`DiplomatWrite`)

**What to build:** A provider method that writes into a `&mut DiplomatWrite` out
parameter is callable from the Rust consumer and yields the written text.

This is the single largest gate group — ten items, more than any other workstream:
`Float64Vec::to_string`, `MyOpaqueEnum::to_string`, `MyString::get_str`,
`MyString::string_transform`, `Opaque::get_debug_str`,
`OpaqueMutexedString::dummy_str`'s sibling `Utf16Wrap::get_debug_str`,
`ResultOpaque::stringify_error`, `OptionString::write`, `OpaqueThin::c`,
`MixinTest::hello`.

**Blocked by:** None technically, but **do not start before the decision below.**

**Status:** needs-decision

**Decision needed before starting:** what is the Rust-side shape of a writer method?

The corpus declares these as `fn get_str(&self, write: &mut DiplomatWrite)`. Every
other backend constructs a callback-backed writer and reads the buffer out. Rust has
no reason to: the natural generated signature is `fn get_str(&self) -> String`, with
the wrapper building the writer internally. That is a *reshaping* of the declared API,
not a translation of it — the same call the naming rules already make. The alternative
is to expose the writer object itself and make every caller drive it, which is strictly
worse ergonomics for no ABI reason.

Confirm the `-> String` shape (and `-> Vec<u16>` for the UTF-16 writer) before
implementing. This also decides whether the generated method can be fallible: a writer
call can fail mid-write, and `stringify_error` returns `Result`.

**Acceptance criteria** (once the shape is settled):

- [ ] All ten writer methods are generated and their `rust` gates removed
- [ ] A consumer test reads the written text back and compares it to the provider's
      expected output, for both a UTF-8 writer and a UTF-16 one
- [ ] `check.sh` passes
