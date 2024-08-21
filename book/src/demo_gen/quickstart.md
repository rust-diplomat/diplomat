# Quickstart

Demo Gen takes a bit of configuration if you don't already have your own Diplomat library set up.

For that reason, we have a [quickstart repository](https://github.com/rust-diplomat/demo-gen-quickstart). You can also follow along in your own library if you'd like.

## Requirements

You'll need to clone [the repository](https://github.com/rust-diplomat/demo-gen-quickstart).

You'll need to have Rust, Cargo, and the `wasm32-unknown-unknown` target installed:

```sh
rustup target add wasm32-unknown-unknown
```

You'll also need `Node` and `npm` [installed](https://nodejs.org/en/download/package-manager), as Diplomat generates JS code in modules that is easier for Node to parse as a package.

## Getting Started

You just need to run (in the repository folder):

```sh
cargo build -p adder_bindings --target wasm32-unknown-unknown
cargo run -p generator
cp target/wasm32-unknown-unknown/debug/adder_bindings.wasm adder_bindings/demo
```

You'll notice the `demo` folder now has a `demo_gen` folder, which is full of JS and rendering files. We can view our results in an HTTP server:

```sh
npm -C adder_bindings/demo install
npm -C adder_bindings/demo run start
```

If you open the server, you should see a webpage listing `AddResult.getAddStr` with a link. If you click the link, you should see something like:

![A search bar for the web server, with the added URL /demo_gen/rendering/template.html?func=AddResult.getAddStr. Displayed on the webpage is "AddResult.getAddStr" in large text. Below are two inputs: one labelled "left" that has a value of 1, and one labelled "right" that has a value of 2. Below is a submit button. There is output below the button, with the label "Output" and a value of 3.](images/demo_output.png)

And that's it! Let's talk about what each step means.

# What We Just Accomplished

When you clone the repository, you'll notice three packages. 

## The Library

`basic_adder` is our Rust library that we want to make examples of. It only has one function:

```rs
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
```

Everything else we build will be based on this.

## The Bindings

`adder_bindings` is our [Diplomat bridge](../basics.md). When we want to make our library accessible in other languages, we need to run `diplomat-tool` on these bindings.

We build these bundings with

```sh
cargo build -p adder_bindings --target wasm32-unknown-unknown
```

`--target wasm32-unknown-unknown` tells `cargo` to build a `.wasm` file.

For more on how you can explicitly configure your bindings to work better in demos, see [the chapter on attributes](attributes.md).

## The Generator

`generator` is our wrapper for calling `diplomat-tool`. demo_gen is still in progress, and so we need a wayt to use the latest version of `diplomat-tool` to use this experimental backend. `generator` may be removed in future versions of this tutorial.

We run the generator with

```sh
cargo run -p generator
```

And it performs the equivalent of:

```sh
diplomat-tool demo_gen adder_bindings/demo/demo_gen --entry adder_bindings/src/lib.rs
```

demo_gen will automatically generate JS bindings by default, along with a bunch of other files to help you get started as easily as possible. If you have JS bindings elsewhere, or a different file structure from the Quickstart repository, you can configure how demo_gen works with a library config file. See [the section on markup generation for more](markup.md).

## The Web Demo

demo_gen is designed to work with most Diplomat libraries with minimal configuration. However, we currently don't provide *everything* that you'll need to instantly see a demo 

The minimum requirement is at least a web server to load JS modules.

This is why we have you run

```sh
npm -C adder_bindings/demo install
npm -C adder_bindings/demo run start
```

See the chapter on [the renderer](./renderer.md) for more.