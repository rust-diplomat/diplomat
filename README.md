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
When using Diplomat, you'll need to define Rust modules that contain the Rust APIs you want to expose. You can do this by using the `diplomat::bridge` macro:
```rust
#[diplomat::bridge]
mod ffi {
    pub struct MyFFIType {
        pub a: i32,
        pub b: bool,
    }
    
    impl MyFFIType {
        pub fn new() -> MyFFIType {
            MyFFIType {
                a: 42,
                b: true
            }
        }
    }
}
```

Every type declared within a `diplomat::bridge` module along with all methods in its associated `impl` will be exposed over the FFI. For example, the above code will generate the following extern API:
```rust
#[no_mangle]
extern "C" fn MyFFIType_new() -> MyFFIType {
    MyFFIType::new()
}
```

We can then generate the bindings for this API using the `diplomat-tool` CLI. For example, if we want to generate C++ bindings, we can create a folder `cpp` and generate bindings in it by running:
```bash
$ diplomat-tool cpp cpp/
```

If we want to generate Sphinx documentation to `cpp-docs`, we can run with that as an additional parameter:
```bash
$ diplomat-tool cpp cpp/ --docs cpp-docs/
```

## Core Concepts
### Opaque Structs
By default, any struct exposed in a bridge module can be returned by value, which means that its size must be known so that the caller can allocate the space to receive it. However, Diplomat only analyzes the code declared within bridge modules, so regular structs cannot contain external types as fields.

To work around this, you can mark a struct as opaque by using the `diplomat::opaque` attribute, which makes it possible to store unknown types in fields by enforcing that the struct is always behind a pointer:
```rust
#[diplomat::bridge]
mod ffi {
    #[diplomat::opaque]
    pub struct MyOpaqueStruct(MyExternalType)

    impl MyOpaqueStruct {
        pub fn new() -> Box<MyOpaqueStruct> {
            ...
        }

        pub fn do_something(&self) {
            ...
        }
    }
}
```

### Special Types
#### `Box`
Especially when dealing with opaque structs, constructors will often return boxes. Diplomat can pass boxes as-is over the FFI boundary, with the corresponding logic in the target language to destroy the box when it is no longer needed. For example in C++, boxes are mapped to `std::unique_ptr` with custom destructors that release the box on the Rust side.

#### `Option`
Currently, Diplomat supports passing `Option<T>` values over the FFI boundary when `T` is a pointer-like type. In C++, this is mapped to `std::optional<T>`, and in JS, you receive a nullable value.

#### `DiplomatResult`
The standard `Result` type in Rust does not have a standard C representation, so it cannot be passed over the FFI boundary. `DiplomatResult` is a `Result`-like type that can be used to pass result values over the FFI boundary. Simply return a `DiplomatResult<T, E>` from your API and use the provided `Into<DiplomatResult<T, E>>` to convert a regular Rust result to a Diplomat result.

#### Strings and `DiplomatWriteable`
When taking a string as a parameter, you can simply take an `&str`. Diplomat handles the logic of taking in a byte buffer and parsing it as a UTF-8 string.

When returning strings, things are a bit more complicated. Since clients of the bindings need to be able to directly interact with the string, we want Rust to write into a buffer owned by the target language. To do this, your method can take in an `&mut DiplomatWriteable` parameter, which represents a (optionally expandable) buffer.

For targets like C, this can be backed by a fixed size character array. For targets like C++, the bindings will automatically collect the written data into a `std::string` and return that to the user. For JavaScript, the bindings will allocate memory in WebAssembly memory and parse the written data into a native string.

## Development
### Architecture
See the [design doc](docs/design_doc.md) for more details.

### Building and Testing
Simply run `cargo build` to build all the libraries and compile an example. To run unit tests, run `cargo test`.

Diplomat makes use of snapshot tests to check macro and code generation logic. When code generation logic changes and the snapshots need to be updated, run `cargo insta review` (run `cargo install cargo-insta` to get the tool) to view the changes and update the snapshots.
