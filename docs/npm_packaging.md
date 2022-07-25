# Publishing Diplomat bindings for your Rust library on NPM

This guide is heavily inspired by https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/getting-started/manual-setup.html.

## Step 1. Create Diplomat bindings
It's recommended to decouple your Diplomat bindings from your Rust library.

To do this, begin by creating a new crate where your bindings will live:
```sh
cargo new --lib my-bindings
```

Before writing bindings, go to your `Cargo.toml` file and add the following configuration settings:

```toml
# my-bindings/Cargo.toml

[package]
name = "my-bindings"
version = "0.1.0"
edition = "2021"

[lib]
# Enable compilation to WebAssembly
crate-type = ["cdylib", "rlib"]

[dependencies]
diplomat = "*"
diplomat-runtime = "*"
# your crate here

[profile.release]
# Optimize for small code size
opt-level = "s"
```

Next, create the bindings to your crate.
> Instructions for how to write bindings are in [The Book](https://rust-diplomat.github.io/book/types.html).
```rust
// my-bindings/src/lib.rs
#[diplomat::bridge]
mod ffi {
    // your bindings here
}
```

## Step 2. Setup file tree for NPM package

Once you've written the bridge
Next, we'll configure the NPM packaging. Start by setting up the file tree:

```sh
# my-bindings/
mkdir lib lib/api lib/docs lib/tests
```

Navigate to `my-bindings/lib` and initialize your `package.json`. The easiest method for this is with `npm init`, which gives a series of prompts to generate a `package.json` for you.
```sh
# my-bindings/lib/
npm init
```

Edit your `package.json` file to contain the following configuration:
```js
// my-bindings/lib/package.json
{
    // other config options...
    "main": "./api/index.js", // diplomat-tool is going to only write in `api/` later
    "type": "module",
    "directories": {
        // other directories...
        "doc": "docs"
    }
}
```

Then, add your `tsconfig.json` with the following configuration to transpile your TypeScript tests:
```js
// my-bindings/lib/tsconfig.json
{
    "compilerOptions": {
        "module": "es2020",
        "target": "es2020",
        "moduleResolution": "node"
    }
}
```

## Step 3. Generate bindings and compile WebAssembly

Since Diplomat provides JavaScript bindings to make the developer-facing API idiomatic in JS/TS, it requires a binding generation step in addition to the WASM compilation step.

Generating Diplomat bindings requires `diplomat-tool`. If you don't already have it installed, run:
```sh
cargo install diplomat-tool
```

Next, generate the JavaScript bindings and TypeScript headers using `diplomat-tool`:
```sh
# my-bindings/
diplomat-tool js lib/api --docs lib/docs
```

To compile your bindings into a `.wasm` file, run:
```sh
# my-bindings/
cargo build --target wasm32-unknown-unknown --target-dir target
```

Finally, copy the generated `wasm` binary into `my-bindings/lib/api/`:
> ⚠️ Note that it must be renamed to `diplomat-lib.wasm` ⚠️
```sh
# my-bindings/
cp target/wasm32-unknown-unknown/debug/my_lib.wasm lib/api/diplomat-lib.wasm
```

## Step 4. Publish to NPM

Follow NPM's instructions [here](https://docs.npmjs.com/packages-and-modules/contributing-packages-to-the-registry).