# Safe Rust backend fixture

This nested workspace proves the generated package boundary:

```text
provider (cdylib only) <- native Diplomat ABI <- generated package <- safe consumer
```

`consumer/Cargo.toml` depends only on `generated`. `generated/Cargo.toml` is
fully emitted by `diplomat-tool rust` and has no dependency on the provider,
Diplomat macros, or `diplomat-runtime`. The provider owns every opaque and its
destructor increments an owner-side atomic probe.

## Tests

Coverage is ordinary `#[test]` code run by `cargo test` — there is no bespoke
`main` harness:

- `consumer/tests/runtime.rs` — runtime behaviour of the safe API, compiled with
  `#![forbid(unsafe_code)]`: owner-side destruction exactly once, shared and
  exclusive views reaching the same native object, optional owned opaques,
  borrowed slices/strings, an opaque carrying a type-level lifetime, a
  lifetime-bearing value struct round-tripped by value, owned slice returns,
  slices of plain `repr(C)` value structs (`&[Point]`, `&mut [Point]`), and
  opaque `Debug` (`TypeName(<addr>)`, which is what lets `.expect` / `.unwrap_err`
  compile).
- `consumer/tests/compile_fail.rs` — one `#[test]` per case under
  `compile_fail/src/bin/`, each driving `cargo check` and asserting that the case
  is rejected for a specific reason (borrow escapes, mutability violations,
  missing `Send`/`Sync`).

Run `scripts/check.sh`. It regenerates the bindings, builds the provider, runs
both test suites (`cargo test --workspace`), enforces `cargo fmt --check` and
`cargo clippy -- -D warnings`, and records `cargo tree`, `nm`, and
`otool -L`/`readelf` evidence under `target/`.
