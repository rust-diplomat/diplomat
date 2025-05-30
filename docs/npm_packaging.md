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
> Instructions for how to write bindings are in [The Book](https://rust-diplomat.github.io/diplomat/types.html).
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

Lastly, create a `diplomat.config.js` file. There are currently two settings:
1. `wasm_path`: URL path to the compiled `.wasm` binary. The reason a URL is required is so that if consumers choose to use Webpack, it can detect that the `wasm` file needs to be cached. It's recommended to put the binary in `my-bindings/lib/api/` for releases.
2. `init` (optional): A function that takes a `wasm` object and gets run during initialization. This is particularly useful when initializing a global, such as a logger. When omitted, no additional initialization is run.

An example config file for `my-bindings` could look like this:
```js
// my-bindings/lib/diplomat.config.js
export default {
    wasm_path: new URL('./api/my_bindings.wasm', import.meta.url),
};
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
cargo build --target wasm32-unknown-unknown
```

Make sure that the generated `wasm` binary is in the location and has the name that `diplomat.config.js` expects it to be.

For example, the above configuration expects it to be in `my-bindings/lib/api`, so we have to copy it over:
```sh
# my-bindings/
cp target/wasm32-unknown-unknown/debug/my_bindings.wasm lib/api
```

## Step 4. Publish to NPM

Follow NPM's instructions [here](https://docs.npmjs.com/packages-and-modules/contributing-packages-to-the-registry).