# Basics

When using Diplomat, you'll need to define Rust modules that contain the Rust APIs you want to expose. You can do this by using the `diplomat::bridge` macro:

```rust
#[diplomat::bridge]
mod ffi {
    pub struct MyFFIStruct {
        pub a: i32,
        pub b: bool,
    }
    
    impl MyFFIStruct {
        pub fn create() -> MyFFIStruct {
            MyFFIStruct {
                a: 42,
                b: true
            }
        }

        pub fn do_a_thing(self) {
            println!("doing thing {:?}", self.b);
        }
    }
}
```

This is a simple struct with public fields; which is easier to reason about in an introductory example. _Most_ APIs exposed via Diplomat will be via "opaque types", to be covered in the [chapter on opaque types](./opaque.md).

Every type declared within a `diplomat::bridge` module along with all methods in its associated impl will be exposed over FFI. For example, the above code will generate the following extern API:

```rust
#[no_mangle]
extern "C" fn MyFFIStruct_create() -> MyFFIStruct {
    MyFFIStruct::create()
}

#[no_mangle]
extern "C" fn MyFFIStruct_do_a_thing(this: &MyFFIStruct) {
    this.do_a_thing()
}
```



We can then generate the bindings for this API using the `diplomat-tool` CLI.


## C++

For example, if we want to generate C++ bindings, we can create a folder `cpp/`` and generate bindings in it by running:

```shell
$ diplomat-tool cpp cpp/
```

This will generate the following struct in `MyFFIStruct.hpp`, along with some boilerplate:

```cpp
struct MyFFIStruct {
 public:
  int32_t a;
  bool b;
  static MyFFIStruct create();
  void do_a_thing();
};
```

If we want to generate Sphinx documentation to cpp-docs, we can run with that as an additional parameter:

```shell
$ diplomat-tool cpp cpp/ --docs cpp-docs/
```

## WASM

For WASM JS/TypeScript bindings, you can use the following options, with similarly named directories:

```shell
$ diplomat-tool js js/ --docs js/docs/
```

This will generate JS that has a `MyFFIStruct` class, with a static `create()` method, a `do_a_thing()` method, and getters for the fields. This JS will require there to be a `wasm.mjs` file that loads in the built wasm file (See [issue #80](https://github.com/rust-diplomat/diplomat/issues/80) for improving this), which you can base off of [this file](https://github.com/rust-diplomat/diplomat/blob/38cffa9bc2ef21d0aba89ed7d76236de4153248a/example/js/wasm.mjs).


## C

While low-level C headers are generated in the process of running `diplomat-tool cpp`, you can also generate just the C headers with

```shell
$ diplomat-tool c c/
```

Note that Diplomat's C mode generates direct bindings to the lower level `extern "C"` API, and is not idiomatic C code. It is recommended that one build a higher level API around the C API (perhaps by writing a plugin) if C bindings are desired.