#!/bin/sh
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
fixture_dir=$(CDPATH= cd -- "$script_dir/.." && pwd)
corpus_dir=$(CDPATH= cd -- "$fixture_dir/../" && pwd)
repo_dir=$(CDPATH= cd -- "$fixture_dir/../.." && pwd)
target_dir=${CARGO_TARGET_DIR:-"$fixture_dir/target"}
generated_dir="$fixture_dir/generated"
# The provider is the shared feature test corpus crate, exactly as it is for
# every other backend. There is no Rust-specific provider.
provider_pkg=diplomat-feature-tests
provider_name=diplomat_feature_tests
# A constructor/destructor pair the provider really exports.
probe_ctor=OptionOpaque_new
probe_dtor=OptionOpaque_destroy

fail() {
    echo "safe-rust fixture: $*" >&2
    exit 1
}

# Cargo resolves `rustdoc` through PATH, so a machine whose PATH serves a different
# toolchain's rustdoc than the `rustc` cargo compiles with (a Homebrew rustdoc ahead
# of the rustup one is the common shape) fails every doc-test with E0514. Because this
# script stops at the first error, that would silently skip the format, lint and symbol
# proofs below and read as an ordinary test failure. Pin RUSTDOC to the toolchain cargo
# is actually using; an explicit RUSTDOC from the caller still wins.
if [ -z "${RUSTDOC:-}" ]; then
    toolchain_rustdoc="$(rustc --print sysroot)/bin/rustdoc"
    if [ -x "$toolchain_rustdoc" ]; then
        RUSTDOC="$toolchain_rustdoc"
        export RUSTDOC
        if command -v rustdoc >/dev/null 2>&1 && [ "$(command -v rustdoc)" != "$RUSTDOC" ]; then
            echo "note: PATH rustdoc is not rustc's ($(rustdoc --version)); pinning RUSTDOC=$RUSTDOC"
        fi
    fi
fi

echo "== generate Safe Rust package from the shared feature test corpus =="
cargo run --quiet --manifest-path "$repo_dir/Cargo.toml" -p diplomat-tool -- \
    rust "$generated_dir" \
    --entry "$corpus_dir/src/lib.rs" \
    --config-file "$corpus_dir/config.toml" \
    --silent
cargo fmt --manifest-path "$generated_dir/Cargo.toml"

export CARGO_TARGET_DIR="$target_dir"

echo "== build the provider: the shared corpus crate =="
cargo build --manifest-path "$repo_dir/Cargo.toml" -p "$provider_pkg"

case "$(uname -s)" in
    Darwin) provider_lib="$target_dir/debug/lib${provider_name}.dylib" ;;
    Linux) provider_lib="$target_dir/debug/lib${provider_name}.so" ;;
    MINGW*|MSYS*|CYGWIN*) provider_lib="$target_dir/debug/${provider_name}.dll" ;;
    *) fail "unsupported host $(uname -s)" ;;
esac
[ -f "$provider_lib" ] || fail "provider cdylib not found: $provider_lib"

echo "== prove dependency graphs exclude the provider implementation and codegen =="
consumer_tree=$(cargo tree --manifest-path "$fixture_dir/consumer/Cargo.toml")
generated_tree=$(cargo tree --manifest-path "$generated_dir/Cargo.toml")
for tree in "consumer:$consumer_tree" "generated:$generated_tree"; do
    case "$tree" in
        *diplomat-feature-tests*|*"diplomat v"*|*diplomat_core*)
            fail "${tree%%:*} dependency tree contains the provider implementation or codegen machinery"
            ;;
    esac
done
printf '%s\n' "$consumer_tree" >"$target_dir/consumer-cargo-tree.txt"
printf '%s\n' "$generated_tree" >"$target_dir/generated-cargo-tree.txt"

echo "== prove the generated crate owns no ABI types and does depend on diplomat-runtime =="
grep -F 'diplomat-runtime' "$generated_dir/Cargo.toml" >/dev/null \
    || fail "generated Cargo.toml does not declare a diplomat-runtime dependency"
if grep -rF -e 'struct DiplomatSlice' -e 'struct DiplomatSliceMut' \
    -e 'struct DiplomatOwnedSlice' -e 'struct DiplomatOption' \
    -e 'struct DiplomatResult' -e 'union DiplomatOptionValue' \
    -e 'union DiplomatResultValue' "$generated_dir/src" >/dev/null; then
    fail "generated crate defines a local ABI type instead of using diplomat-runtime"
fi

native_dir="$target_dir/debug"
export DIPLOMAT_RUST_NATIVE_LIB_DIR="$native_dir"
export DYLD_LIBRARY_PATH="$native_dir${DYLD_LIBRARY_PATH:+:$DYLD_LIBRARY_PATH}"
export LD_LIBRARY_PATH="$native_dir${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"

echo "== run the safe consumer and compile-fail test suites =="
cargo test --manifest-path "$fixture_dir/Cargo.toml" --workspace

echo "== prove a rejected provider reports its diagnostic instead of aborting on the span =="
# The bridge module below is inline in the *entry* file, which is the shape that makes
# the tool record the entry file's parent directory as the module's source location.
# Reading that directory as a file used to abort the whole report, so a provider in
# this shape got a bare panic instead of a diagnostic.
probe_dir="$target_dir/reject-probe"
rm -rf "$probe_dir"
mkdir -p "$probe_dir/src"
cat >"$probe_dir/src/lib.rs" <<'PROBE'
#[diplomat::bridge]
pub mod ffi {
    #[diplomat::opaque]
    pub struct Probe;

    impl Probe {
        pub fn bad(&self, c: char) {}
    }
}
PROBE
cat >"$probe_dir/config.toml" <<'PROBE'
[rust]
crate-name = "diplomat-rust-backend-reject-probe"
dylib-name = "diplomat_rust_backend_probe"
PROBE
probe_log="$probe_dir/output.txt"
if cargo run --quiet --manifest-path "$repo_dir/Cargo.toml" -p diplomat-tool -- \
    rust "$probe_dir/generated" \
    --entry "$probe_dir/src/lib.rs" \
    --config-file "$probe_dir/config.toml" >"$probe_log" 2>&1
then
    fail "a provider that uses char was accepted instead of rejected"
fi
if grep -q 'Could not read source file' "$probe_log"; then
    fail "diagnostic reporting aborted on an unreadable span instead of reporting"
fi
if ! grep -q 'Could not resolve char' "$probe_log"; then
    cat "$probe_log" >&2
    fail "the diagnostic for a rejected provider was not reported"
fi
echo "   rejected provider reported: $(head -n 2 "$probe_log" | tr '\n' ' ')"
rm -rf "$probe_dir"

echo "== prove fixture sources, generated code, and tests are fmt- and lint-clean =="
cargo fmt --manifest-path "$fixture_dir/Cargo.toml" --all --check
cargo clippy --manifest-path "$fixture_dir/Cargo.toml" --workspace --all-targets -- -D warnings

echo "== prove symbols are provider-defined and consumer-imported =="
command -v nm >/dev/null 2>&1 || fail "nm is required"
command -v python3 >/dev/null 2>&1 || fail "python3 is required to locate the consumer test executable"

consumer_bin=$(
    cargo test --manifest-path "$fixture_dir/consumer/Cargo.toml" \
        --test runtime --no-run --message-format=json 2>/dev/null \
        | python3 -c 'import json, sys
for line in sys.stdin:
    try:
        message = json.loads(line)
    except ValueError:
        continue
    executable = message.get("executable")
    if executable:
        print(executable)' \
        | head -n 1
)
[ -n "$consumer_bin" ] && [ -f "$consumer_bin" ] || fail "consumer test executable not found"

nm "$provider_lib" >"$target_dir/provider-symbols.txt"
nm -u "$consumer_bin" >"$target_dir/consumer-undefined-symbols.txt" 2>/dev/null || true
grep "$probe_ctor" "$target_dir/provider-symbols.txt" >/dev/null || fail "provider does not define $probe_ctor"
grep "$probe_dtor" "$target_dir/provider-symbols.txt" >/dev/null || fail "provider does not define $probe_dtor"
grep "$probe_ctor" "$target_dir/consumer-undefined-symbols.txt" >/dev/null || fail "consumer does not import $probe_ctor"
grep "$probe_dtor" "$target_dir/consumer-undefined-symbols.txt" >/dev/null || fail "consumer does not import $probe_dtor"
if nm "$consumer_bin" | grep -E "[[:space:]][Tt][[:space:]].*$probe_ctor" >/dev/null; then
    fail "consumer defines provider constructor/destructor symbols"
fi

echo "== prove the provider is a dynamic dependency =="
case "$(uname -s)" in
    Darwin)
        command -v otool >/dev/null 2>&1 || fail "otool is required on macOS"
        otool -L "$consumer_bin" >"$target_dir/consumer-dependencies.txt"
        ;;
    Linux)
        command -v readelf >/dev/null 2>&1 || fail "readelf is required on Linux"
        readelf -d "$consumer_bin" >"$target_dir/consumer-dependencies.txt"
        ;;
esac
grep "$provider_name" "$target_dir/consumer-dependencies.txt" >/dev/null || fail "provider is not a recorded dynamic dependency"

echo "Safe Rust backend fixture and architecture checks passed"
