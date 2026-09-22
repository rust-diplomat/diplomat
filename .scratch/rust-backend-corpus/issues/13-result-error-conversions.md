# 13 — Convert `Result` error payloads into safe Rust types

**Status:** DONE

**What to build:** Convert every supported non-opaque `Result<T, E>` error payload
from its private ABI spelling into the public safe Rust spelling, just as success
payloads are converted.

**Why this is a correctness ticket:** `tool/src/rust/gen/method.rs::error_expr`
currently returns `result` unchanged for every non-opaque error. That is only valid
for layout-identical primitives/enums/plain structs. `check_fallible_error` accepts
`char` as a supported primitive and accepts plain value error structs that contain
`char` or `DiplomatOption`, so the generated public signature can disagree with the
ABI expression.

Minimal reproductions:

```rust
pub fn char_error() -> Result<(), DiplomatChar> { ... }

#[diplomat::attr(auto, error)]
pub struct ErrorWithChar { pub c: DiplomatChar }
pub fn struct_error() -> Result<(), ErrorWithChar> { ... }
```

The ABI returns `DiplomatResult<(), u32>` /
`DiplomatResult<(), ffi::ErrorWithChar>`, while the public API promises
`Result<(), char>` / `Result<(), ErrorWithChar>`. The generated crate currently
fails to compile (`E0277`) because `Err(result)` does not convert the payload.
The same path is used for writer methods returning `Result<String, E>`.

**Scope:**

- Route supported non-opaque errors through the shared recursive
  `convert_from_ffi` logic (or an equivalent dedicated error converter).
- Preserve the existing owned-opaque error path: construct the owning wrapper and
  ensure its provider destructor runs exactly once.
- Cover `char`, mirrored structs, nested option fields, enums and ordinary
  primitives without changing the public `Result<T, E>` shape.
- Keep unsupported borrowed/nullable opaque errors and lifetime-bearing error
  structs rejected with their existing contextual diagnostics.
- Ensure the conversion happens inside the `Result::from(result)` match so the ABI
  container's payload is moved exactly once.

**Tests / acceptance criteria:**

- [x] A generator test asserts `char` and mirrored-struct error arms call the safe
      conversion code instead of returning the raw ABI payload.
- [x] A generated-crate compile test covers `Result<(), char>` and
      `Result<(), ErrorWithChar>`; both safe signatures compile.
- [x] A runtime consumer test exercises both `Ok` and `Err` arms and checks the
      converted `char`/struct fields.
- [x] A writer method returning `Result<String, E>` with a converting error payload
      compiles and converts its error arm.
- [x] Existing owned-opaque error tests still prove exactly-once destruction.
- [x] `feature_tests/rust/scripts/check.sh` passes.

**Blocked by:** None. This is a follow-up correctness fix for the Result subset
claimed by the backend and ticket 03's mirror work.

**Implementation:** non-opaque error arms now use `convert_from_ffi`, while owned
opaque errors retain their dedicated wrapper construction and Drop path. The
Rust-only fixture covers a `DiplomatChar` error and the generated unit test covers
mirrored structs, nested options, and writer Results.
