//! Compile-fail coverage for the generated safe API.
//!
//! Every case is a standalone file under `compile_fail/src/bin/` that must be
//! rejected by rustc for a specific reason. Driving `cargo check` from a test
//! keeps `cargo test` the single entry point for the fixture's coverage.
//!
//! These cases are the part of the coverage that has no analogue in another
//! backend's suite. The C# consumer gets borrow safety from `BorrowLease` and
//! `IDisposable` at run time; here the same defects have to be rejected by the
//! type system, and that is what is asserted. The provider in every case is a
//! type from the shared `feature_tests/src` corpus.
#![forbid(unsafe_code)]

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;

/// Nested `cargo check` runs share one target directory, so serialize them
/// instead of letting them contend for cargo's build lock.
static BUILD_LOCK: Mutex<()> = Mutex::new(());

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the consumer manifest always has a parent directory")
        .to_path_buf()
}

/// A dedicated target directory so the nested `cargo check` never contends with
/// the `cargo test` invocation that is running this test.
fn nested_target_dir() -> PathBuf {
    match std::env::var_os("CARGO_TARGET_DIR") {
        Some(dir) => PathBuf::from(dir).join("compile-fail-ui"),
        None => fixture_dir().join("target/compile-fail-ui"),
    }
}

/// Runs `cargo check` for one case, asserts that it fails, and returns the
/// combined diagnostics.
fn check(case: &str) -> String {
    let _serial = BUILD_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let manifest = fixture_dir().join("compile_fail/Cargo.toml");

    let output = Command::new(cargo)
        .arg("check")
        .arg("--manifest-path")
        .arg(&manifest)
        .arg("--bin")
        .arg(case)
        .env("CARGO_TARGET_DIR", nested_target_dir())
        .output()
        .expect("failed to spawn `cargo check` for a compile-fail case");

    assert!(
        !output.status.success(),
        "compile-fail case `{case}` compiled but should not have"
    );

    let mut diagnostics = String::from_utf8_lossy(&output.stderr).into_owned();
    diagnostics.push_str(&String::from_utf8_lossy(&output.stdout));
    diagnostics
}

/// Asserts that a compile-fail case was rejected with one of the expected rustc
/// **error codes**.
///
/// Codes are the stable part of a rejection; message wording is not, and has changed
/// under this suite before (a case written for E0597 has actually emitted E0515 since
/// the day it landed, and only passed because the older shell-based check accepted
/// either string). Where rustc legitimately varies between codes for the same defect,
/// list every accepted code. The rendered diagnostics are printed on failure, so the
/// wording is still available as context — it is just not what is asserted.
macro_rules! compile_fail_case {
    ($name:ident => [$($code:literal),+ $(,)?]) => {
        #[test]
        fn $name() {
            let diagnostics = check(stringify!($name));
            let accepted = [$($code),+];
            assert!(
                accepted
                    .iter()
                    .any(|code| diagnostics.contains(&format!("error[{code}]"))),
                "compile-fail case `{}` was rejected without any of the expected error codes {:?}:\n{diagnostics}",
                stringify!($name),
                accepted
            );
        }
    };
    ($name:ident => $code:literal) => {
        compile_fail_case!($name => [$code]);
    };
}

// Returning a view borrowed from a local owner: it cannot outlive the owner.
compile_fail_case!(borrowed_view_outlives_owner => ["E0515", "E0597"]);
// Borrowed views of an opaque are deliberately neither `Send` nor `Sync`.
compile_fail_case!(borrowed_not_send => "E0277");
compile_fail_case!(borrowed_not_sync => "E0277");
// Owned opaques are deliberately neither `Send` nor `Sync`.
compile_fail_case!(not_send => "E0277");
compile_fail_case!(not_sync => "E0277");
// A live shared borrow of provider memory blocks the exclusive borrow.
compile_fail_case!(shared_borrow_blocks_mutation => "E0502");
// The shared-argument capability is sealed against downstream implementations.
compile_fail_case!(capability_trait_is_sealed => ["E0277", "E0603", "E0624"]);
// A borrowed slice cannot outlive the opaque owning the buffer.
compile_fail_case!(slice_view_outlives_receiver => ["E0597", "E0515"]);
compile_fail_case!(slice_outlives_owner => ["E0597", "E0515"]);
// Two exclusive views of one opaque.
compile_fail_case!(two_exclusive_views => "E0499");
// An exclusive view blocks any further borrow of its source.
compile_fail_case!(mutable_view_blocks_source => ["E0502", "E0499"]);
// A shared borrow of provider-owned memory blocks an exclusive borrow.
compile_fail_case!(borrowed_view_blocks_source => "E0502");
// An opaque input must preserve its type-level lifetime; a shorter Foo cannot
// be passed to a method whose provider signature requires the owner's Foo<'a>.
compile_fail_case!(opaque_type_lifetime_mismatch => ["E0277", "E0597"]);
// Storing a short-lived opaque into one tied to a longer buffer, then handing
// that borrow back to the caller, is rejected.
compile_fail_case!(lifetime_store_short_as_long => "E0515");
