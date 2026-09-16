//! Compile-fail coverage for the generated safe API.
//!
//! Every case is a standalone file under `compile_fail/src/bin/` that must be
//! rejected by rustc for a specific reason. Driving `cargo check` from a test
//! keeps `cargo test` the single entry point for the fixture's coverage.
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

macro_rules! compile_fail_case {
    ($name:ident => $expected:literal) => {
        #[test]
        fn $name() {
            let diagnostics = check(stringify!($name));
            assert!(
                diagnostics.contains($expected),
                "compile-fail case `{}` failed for an unexpected reason; expected {:?}:\n{diagnostics}",
                stringify!($name),
                $expected
            );
        }
    };
}

compile_fail_case!(borrowed_child_outlives_parent => "returns a value referencing data owned by the current function");
compile_fail_case!(borrowed_not_send => "cannot be sent between threads safely");
compile_fail_case!(borrowed_not_sync => "cannot be shared between threads safely");
compile_fail_case!(not_send => "cannot be sent between threads safely");
compile_fail_case!(not_sync => "cannot be shared between threads safely");
compile_fail_case!(shared_cannot_mutate => "no method named");
compile_fail_case!(shared_cannot_satisfy_mut_param => "is not satisfied");
compile_fail_case!(slice_mut_while_shared => "as mutable because it is also borrowed as immutable");
compile_fail_case!(slice_outlives_receiver => "does not live long enough");
compile_fail_case!(struct_field_outlives_source => "does not live long enough");
compile_fail_case!(two_exclusive_views => "as mutable more than once");
compile_fail_case!(view_outlives_buffer => "does not live long enough");
