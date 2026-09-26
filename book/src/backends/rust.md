# Rust Backend

The Rust backend emits a Cargo package whose public API is safe Rust. It calls the provider cdylib through Diplomat's C ABI. The generated crate depends on `diplomat-runtime` and does not depend on the provider crate.

```sh
diplomat-tool -e {PATH_TO_LIB.RS} -c {CONFIG_FILE} rust {OUTPUT_PATH}
```

Configuration:

* `lib-name` — the default native library name, and the stem of the package name when `rust.crate-name` is unset (`{lib-name}-bindings`).
* `rust.crate-name` — Cargo package name of the generated crate.
* `rust.dylib-name` — name passed to `#[link]`. Defaults to `lib-name`. The consumer looks for `lib{dylib-name}.so`, `lib{dylib-name}.dylib`, or `{dylib-name}.dll`.
* `rust.publish` — optional bool. Set it to `false` to emit `publish = false`. When omitted, the generated package can be published.

`build.rs` adds a native link-search path only when `DIPLOMAT_RUST_NATIVE_LIB_DIR` is set. Point it at the directory that contains the provider cdylib:

```sh
DIPLOMAT_RUST_NATIVE_LIB_DIR=target/debug cargo build
```

An owned `Box<[u8]>` from the provider is returned as `DiplomatBoxU8`. `Deref` and `AsRef<[u8]>` borrow the bytes. `clone_to_box` copies them into a `Box<[u8]>` this crate allocates. `into_box` reuses the allocation and is `unsafe`: this crate and the provider cdylib must use the same global allocator. `Drop` calls `diplomat_owned_slice_u8_destroy` in the provider cdylib.

Method names are rendered in snake_case, including a `named_constructor` name. `f64BeBytes` becomes `f64_be_bytes`.

The backend is experimental. Callbacks, traits, and free functions are rejected. A shape it cannot encode produces a diagnostic and no files.

The shared corpus and the safe consumer live under `feature_tests/rust`. `example/config.toml` sets `rust.dylib-name` to the example cdylib.

{{supports("rust")}}
