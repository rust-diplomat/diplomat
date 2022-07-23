# Publishing Diplomat bindings for your Rust library on NPM

This guide is heavily inspired by https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/getting-started/manual-setup.html.

## Step 1. Create Diplomat bindings
It's recommended to decouple your Diplomat bindings from your Rust library.

To do this, begin by creating a new crate where your bindings will live:
```sh
cargo new --lib my-bindings
```

The first thing to do is edit your `Cargo.toml` file. In order to support compiling
to WebAssembly and unit testing, add `"cdylib"` and `"rlib"` as the crate types.
```toml
[lib]
crate-type = ["cdylib", "rlib"]
```

Next, add `diplomat` and `diplomat-runtime` as dependencies.
```sh
# my-bindings/
cargo add diplomat
cargo add diplomat-runtime
```
Finally, you can tell `rustc` to optimize for space on release mode, in order to
reduce the size of your generated `.wasm` file that has to be transmitted over
the network when you webpage loads.
```toml
[profile.release]
opt-level = "s"
```

Make sure to add your Rust library as a dependency as well.

After these changes, you're `Cargo.toml` file should look something like this:
```toml
# my-bindings/Cargo.toml

[package]
name = "my-bindings"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
diplomat = "*"
diplomat-runtime = "*"
# presumably your Rust library

[profile.release]
# Tell `rustc` to optimize for small code size.
opt-level = "s"
```

Now you can implement your bindings:
```rust
// my-bindings/src/lib.rs
#[diplomat::bridge]
mod ffi {
    // your bindings here
}
```

## Step 2. Create NPM package

Next, we'll configure the NPM packaging. Start by setting up the file tree:

```sh
# my-bindings/
mkdir lib lib/api lib/docs
```

Navigate to `my-bindings/lib` and initialize your `package.json`. The easiest method for this is with `npm init`, which gives a series of prompts to generate a `package.json` for you.
```sh
# my-bindings/lib/
npm init
```

Edit your `package.json` file to contain the following configuration:
```json
// my-bindings/lib/package.json
{
    // ...
    "main": "./api/index.js", // diplomat-tool is going to only write in `api/` later
    "type": "module",
    "directories": {
        // ...
        "doc": "docs"
    },
    // ...
}
```

Since you're writing tests (right?), create a `tests/` folder:
```sh
# my-bindings/lib/
mkdir tests
```

Then, add your `tsconfig.json` with the following configuration to transpile your TypeScript tests:
```json
// my-bindings/lib/tsconfig.json
{
    "compilerOptions": {
        "module": "es2020",
        "target": "es2020",
        "moduleResolution": "node"
    }
}
```

To generate the JavaScript bindings and TypeScript headers, start by installing `diplomat-tool`:
```sh
cargo install diplomat-tool
```

Use `diplomat-tool` to generate bindings.
> Note that additional flags, including the base doc URL.
```sh
# my-bindings/
diplomat-tool js lib/api --docs lib/docs
```

Run `cargo build` to compile your bindings into a `.wasm` file:
```sh
# my-bindings/
cargo build --target wasm32-unknown-unknown --target-dir target
```

Finally, copy the generated `wasm` binary into `my-bindings/lib/api/`:
```sh
# my-bindings/
cp target/wasm32-unknown-unknown/debug/my_lib.wasm lib/api/diplomat-lib.wasm
```
