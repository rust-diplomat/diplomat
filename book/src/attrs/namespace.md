# Namespacing

(Supported by `cpp`, queried with `supports = namespacing`)

This attribute can be applied on types and on bridge modules (applying to all types in the module). 
It allows code to be organized under one or more namespaces.

```rust
#[diplomat::bridge]
#[diplomat::attr(auto, namespace = "mylib")]
mod ffi {
    #[diplomat::opaque]
    struct Foo;

    #[diplomat::opaque]
    struct Bar;
}
```

Here, in C++ `Foo` and `Bar` will both be available as `mylib::Foo` and `mylib::Bar`.


Nested namespaces are technically supported using things like `mylib::mymodule`, however they're not tested and support is brittle, so use at your own risk ([#591](https://github.com/rust-diplomat/diplomat/issues/591)).

There is some discussion over separating the concept of a namespace and a library in [#589](https://github.com/rust-diplomat/diplomat/issues/589).