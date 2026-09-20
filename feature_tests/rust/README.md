# Safe Rust backend fixture

The provider is the **shared feature test corpus** at `../src`, the same one
every other backend generates from. There is no Rust-specific provider crate:
when a shape in the corpus cannot be generated, that is a bug in
`tool/src/rust`, not a reason to write a second corpus with duplicate types.

```text
feature_tests/src (shared corpus) -> diplomat-tool rust -> generated package
                                     native Diplomat ABI -> safe consumer
```

- `generated` — the complete output of `diplomat-tool rust`, depending on
  `diplomat-runtime` by version. `cargo make gen-rust-feature` regenerates it.
- `consumer` — `#![forbid(unsafe_code)]`, depending only on `generated`.
- `compile_fail` — standalone negative sources, excluded from normal workspace
  builds and driven case-by-case from `tests/compile_fail.rs`.

Shapes the backend does not implement yet are disabled **in the shared corpus**,
the same way the corpus already disables shapes for dotnet, kotlin and dart —
e.g. `#[diplomat::attr(rust, disable)]` on a writer method, or
`#[diplomat::attr(any(dotnet, rust), disable)]` when a shape is unavailable to
more than one backend. `rg 'rust, disable' ../src` is therefore the exact list
of what this backend still owes.

## Tests

Coverage is ordinary `#[test]` code run by `cargo test` — there is no bespoke
`main` harness:

- `consumer/tests/runtime.rs` — runtime behaviour of the safe API, compiled with
  `#![forbid(unsafe_code)]`, over the corpus's own types: `Opaque`, `MyString`
  (including the keyword-escaped `MyString::r#unsafe`), `Float64Vec`,
  `Utf16Wrap`, `OpaqueThinVec`, `OpaqueMutexedString`, `OptionOpaque`,
  `ResultOpaque` (every error arm the backend claims, including an owned opaque
  error), `OwnedSliceReturn`, and the corpus's enums.
- `consumer/tests/compile_fail.rs` — one `#[test]` per case under
  `compile_fail/src/bin/`, each driving `cargo check` and asserting that the case
  is rejected for a specific reason (borrow escapes, mutability violations,
  missing `Send`/`Sync`, a forged capability handle). This is the part with no
  analogue in another backend's suite: a C# consumer gets borrow safety from
  `BorrowLease` and `IDisposable` at run time, so the same defects have to be
  rejected by the Rust type system.

Run `scripts/check.sh`. It regenerates the bindings, builds the shared corpus
crate as the provider, runs both test suites, enforces `cargo fmt --check` and
`cargo clippy -- -D warnings`, and records `cargo tree`, `nm`, and
`otool -L`/`readelf` evidence under `target/`.
