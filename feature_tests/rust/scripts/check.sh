#!/bin/sh
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
fixture_dir=$(CDPATH= cd -- "$script_dir/.." && pwd)
repo_dir=$(CDPATH= cd -- "$fixture_dir/../.." && pwd)
target_dir=${CARGO_TARGET_DIR:-"$fixture_dir/target"}
generated_dir="$fixture_dir/generated"
provider_name=diplomat_rust_backend_provider

fail() {
    echo "safe-rust fixture: $*" >&2
    exit 1
}

echo "== generate Safe Rust package from provider HIR =="
cargo run --quiet --manifest-path "$repo_dir/Cargo.toml" -p diplomat-tool -- \
    rust "$generated_dir" \
    --entry "$fixture_dir/provider/src/lib.rs" \
    --config-file "$fixture_dir/provider/config.toml" \
    --silent
cargo fmt --manifest-path "$generated_dir/Cargo.toml"

export CARGO_TARGET_DIR="$target_dir"

echo "== build provider as its only configured artifact: cdylib =="
cargo build --manifest-path "$fixture_dir/Cargo.toml" -p diplomat-rust-backend-provider

case "$(uname -s)" in
    Darwin) provider_lib="$target_dir/debug/lib${provider_name}.dylib" ;;
    Linux) provider_lib="$target_dir/debug/lib${provider_name}.so" ;;
    MINGW*|MSYS*|CYGWIN*) provider_lib="$target_dir/debug/${provider_name}.dll" ;;
    *) fail "unsupported host $(uname -s)" ;;
esac
[ -f "$provider_lib" ] || fail "provider cdylib not found: $provider_lib"

echo "== prove dependency graphs exclude provider implementation and codegen =="
consumer_tree=$(cargo tree --manifest-path "$fixture_dir/consumer/Cargo.toml")
generated_tree=$(cargo tree --manifest-path "$fixture_dir/generated/Cargo.toml")
case "$consumer_tree" in
    *diplomat-rust-backend-provider*|*"diplomat v"*|*diplomat_core*)
        fail "consumer dependency tree contains provider implementation or codegen machinery"
        ;;
esac
case "$generated_tree" in
    *diplomat-rust-backend-provider*|*"diplomat v"*|*diplomat_core*)
        fail "generated dependency tree contains provider implementation or codegen machinery"
        ;;
esac
printf '%s\n' "$consumer_tree" >"$target_dir/consumer-cargo-tree.txt"
printf '%s\n' "$generated_tree" >"$target_dir/generated-cargo-tree.txt"

echo "== prove generated crate owns no ABI types and does depend on diplomat-runtime =="
grep -F 'diplomat-runtime' "$generated_dir/Cargo.toml" >/dev/null \
    || fail "generated Cargo.toml does not declare a diplomat-runtime dependency"
if grep -rF -e 'struct DiplomatSlice' -e 'struct DiplomatSliceMut' \
    -e 'struct DiplomatOwnedSlice' -e 'struct DiplomatOption' \
    -e 'union DiplomatOptionValue' "$generated_dir/src" >/dev/null; then
    fail "generated crate defines a local ABI type instead of using diplomat-runtime"
fi

native_dir="$target_dir/debug"
export DIPLOMAT_RUST_NATIVE_LIB_DIR="$native_dir"
export DYLD_LIBRARY_PATH="$native_dir${DYLD_LIBRARY_PATH:+:$DYLD_LIBRARY_PATH}"
export LD_LIBRARY_PATH="$native_dir${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"

echo "== run the safe consumer and compile-fail test suites =="
cargo test --manifest-path "$fixture_dir/Cargo.toml" --workspace

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
grep 'Counter_new' "$target_dir/provider-symbols.txt" >/dev/null || fail "provider does not define Counter_new"
grep 'Counter_destroy' "$target_dir/provider-symbols.txt" >/dev/null || fail "provider does not define Counter_destroy"
grep 'Counter_new' "$target_dir/consumer-undefined-symbols.txt" >/dev/null || fail "consumer does not import Counter_new"
grep 'Counter_destroy' "$target_dir/consumer-undefined-symbols.txt" >/dev/null || fail "consumer does not import Counter_destroy"
if nm "$consumer_bin" | grep -E '[[:space:]][Tt][[:space:]].*Counter_(new|destroy)' >/dev/null; then
    fail "consumer defines provider constructor/destructor symbols"
fi

echo "== prove provider is a dynamic dependency =="
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
