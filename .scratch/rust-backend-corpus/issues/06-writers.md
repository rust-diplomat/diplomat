# 06 — Writer methods (`DiplomatWrite`)

**What to build:** A provider method that writes into a `&mut DiplomatWrite` out
parameter is callable from the Rust consumer and yields the written text.

This is the single largest gate group — ten items, more than any other workstream:
`Float64Vec::to_string`, `MyOpaqueEnum::to_string`, `MyString::get_str`,
`MyString::string_transform`, `Opaque::get_debug_str`,
`Utf16Wrap::get_debug_str`, `ResultOpaque::stringify_error`,
`OptionString::write`, `OpaqueThin::c`, `MixinTest::hello`.

**Blocked by:** None technically.

**Status:** DONE

**Decision:** hide `DiplomatWrite`. The public signature is `fn get_str(&self) -> String`
(and `Result<String, E>` / `Option<String>` when the provider method is fallible or
nullable). The writer is constructed in the generated crate, passed as the trailing
ABI argument, and never named on the public API. `DiplomatWrite` is UTF-8
(`fmt::Write`), so there is no `Vec<u16>` shape — `Utf16Wrap::get_debug_str` still
returns `String` (it writes the `Debug` form of the `Vec<u16>`).

`ResultOpaque::stringify_error` stays gated: it is a writer *and* a borrowed opaque
error (`Result<(), &'a Self>`), which is ticket 07.

**Acceptance criteria:**

- [x] Decision: `-> String` (not an exposed writer)
- [x] Writer methods above (except `stringify_error`) are generated and their `rust` gates removed
- [x] A consumer test reads the written text back and compares it to the provider's
      expected output (`writer_methods_return_the_text_the_provider_wrote`)
- [x] `check.sh` passes
