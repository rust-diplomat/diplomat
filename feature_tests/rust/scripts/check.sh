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

echo "== prove every claimed capability flag is load-bearing =="
# `attr_support()` is a promise. Four flags were once claimed with no effect at all:
# `constructors` and `named_constructors` assert a "special constructor method" Rust
# does not have, and `utf8_strings`/`utf16_strings` were read by nothing. All four were
# nominally "covered" by a prose table whose rows cited a provider that no longer
# exists, so nothing caught them.
#
# The test of a claim is a toggle: turn the flag off and see whether the set of corpus
# items that survive changes. If it does not, the flag buys nothing and is a lie about
# the backend. Evaluate the corpus conditions directly rather than pattern-matching —
# a regex cannot tell `cfg(supports = X)` (include this) from `attr(..., disable)`
# (exclude this), nor notice an item that a second condition already excludes. An
# earlier version of this check made exactly those two mistakes and reported three
# live items for a flag that carries one.
#
# Known blind spot: conditions on an enclosing type or impl block are not inherited by
# the items inside it, so an item excluded only via its container still counts as
# carried. That over-counts, so this check can miss a dead claim, but it will not fail a
# live one.
python3 - "$repo_dir" "$corpus_dir" <<'AUDIT' || fail "capability claims are not load-bearing"
import re, sys, pathlib

repo, corpus = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
body = pathlib.Path(repo, "tool/src/rust/mod.rs").read_text() \
    .split("pub(crate) fn attr_support")[1].split("\n}\n")[0]
CLAIMED = sorted(set(re.findall(r"support\.(\w+)\s*=\s*true", body)))

def split_args(text):
    out, depth, cur = [], 0, ""
    for ch in text:
        if ch == "(":
            depth += 1
        if ch == ")":
            depth -= 1
        if ch == "," and depth == 0:
            out.append(cur)
            cur = ""
            continue
        cur += ch
    if cur.strip():
        out.append(cur)
    return out

def evaluate(cond, flags):
    cond = cond.strip()
    for fn in ("any", "all", "not"):
        if cond.startswith(fn + "(") and cond.endswith(")"):
            args = [evaluate(a, flags) for a in split_args(cond[len(fn) + 1:-1])]
            return any(args) if fn == "any" else (all(args) if fn == "all" else not args[0])
    if cond == "rust":
        return True
    if cond.startswith("supports"):
        return cond.split("=", 1)[1].strip() in flags
    return False  # a `feature = ...` or another backend's name

CFG = re.compile(r"#\[diplomat::cfg\((?P<c>.*?)\)\]", re.S)
DIS = re.compile(r"#\[diplomat::attr\((?P<c>.*?),\s*disable\)\]", re.S)
BOUNDARY = re.compile(r"^\s*$|^\s*[})]|^\s*(pub\b|fn\b|struct\b|enum\b|impl\b|mod\b|use\b|#!|macro_rules)|^\s*#\[diplomat::bridge\]")

def included(block, flags):
    for m in DIS.finditer(block):
        if evaluate(m.group("c"), flags):
            return False
    for m in CFG.finditer(block):
        if not evaluate(m.group("c"), flags):
            return False
    return True

items = []
for path in sorted(corpus.rglob("*.rs")):
    lines = path.read_text().split("\n")
    for i, line in enumerate(lines):
        m = re.match(r"^\s*(?:pub(?:\([^)]*\))?\s+)?(fn|struct|enum)\s+(\w+)", line)
        if not m:
            continue
        top = i
        while top > 0 and not BOUNDARY.match(lines[top - 1]):
            top -= 1
        block = "\n".join(lines[top:i])
        # Key by source location, not by bare name: `new` occurs on most types, so a
        # name-keyed set difference reported "changes nothing" for a flag that carries
        # one of them, because another type's `new` survived without it.
        items.append((f"{path.relative_to(corpus)}:{i + 1} {m.group(1)} {m.group(2)}", block))

full = {k for k, b in items if included(b, set(CLAIMED))}
worse = 0
for flag in CLAIMED:
    without = {k for k, b in items if included(b, set(CLAIMED) - {flag})}
    carried = sorted(full - without)
    if not carried:
        print(f"  {flag}: claimed, but turning it off changes nothing", file=sys.stderr)
        worse += 1
    else:
        print(f"  {flag}: carries {len(carried)} corpus item(s)")
        for c in carried[:4]:
            print(f"      {c}")
sys.exit(1 if worse else 0)
AUDIT

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
