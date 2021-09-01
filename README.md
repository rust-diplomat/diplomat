# Diplomat
Diplomat is an experimental Rust tool for generating FFI definitions allowing many other languages to call Rust code. With Diplomat, you can simply define Rust APIs to be exposed over FFI and get high-level C, C++, and JavaScript bindings automatically!

Diplomat supports languages through a plugin interface that makes it easy to add support for your favourite language. See `tool/src/{c, cpp, js}` for examples of existing language plugins.

## Installation
First, install the CLI tool for generating bindings:
```bash
$ cargo install diplomat-tool
```

Then, add the Diplomat macro and runtime as dependencies to your project:
```toml
diplomat = "0.2.0"
diplomat-runtime = "0.2.0"
```

## Getting Started

Documentation on how to use Diplomat can be found [in the book](https://rust-diplomat.github.io/book/).

### Architecture
See the [design doc](docs/design_doc.md) for more details.

### Building and Testing
Simply run `cargo build` to build all the libraries and compile an example. To run unit tests, run `cargo test`.

Diplomat makes use of snapshot tests to check macro and code generation logic. When code generation logic changes and the snapshots need to be updated, run `cargo insta review` (run `cargo install cargo-insta` to get the tool) to view the changes and update the snapshots.
