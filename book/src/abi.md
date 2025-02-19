# ABI naming/renaming

As previously mentioned, the `#[diplomat::bridge]` macro from the `diplomat` crate will generate `extern "C"` functions for all methods expected to be exposed over FFI.

_By default_, the naming scheme for these types is `TypeName_method()`. For example, `Foo::bar()` becomes `Foo_bar()`.

However, these names can be changed, both at an individual and bulk level, using the `#[diplomat::abi_rename = "..."]` attribute.

One use case for this is maintaining ABI stability. Let's say you wished to change the signature or even behavior of `Foo::bar()` to go from returning a `u8` to returning a `u16`, and were okay with requiring new bindings for the new behavior, but wanted to ensure that old headers/bindings would still work against new Rust code without any change. You could do that by 

```rust
#[diplomat::bridge]
mod ffi {

    #[diplomat::opaque]
    struct Foo;

    impl Foo {
        #[diplomat::abi_rename = "Foo_bar"]
        pub fn old_bar(&self) -> u8 {}

        #[diplomat::abi_rename = "Foo_bar2"]
        pub fn bar(&self) -> u16 {}
    }
}
```

Here, `old_bar()` is still exposed over FFI as `Foo_bar()`, whereas the new `bar()` method is exposed as `Foo_bar2()`.


However, from the point of view of higher-level bindings generated over this (e.g. in C++ or JS), `Foo` now has a `bar()` method with a new signature, and an `old_bar()` method with the signature `bar()` used to have.


The attribute can be applied directly on methods, but it can also be applied on impl blocks, types, and even entire bridge modules. It supports using replacement patterns, so it can be used for namespacing the generated FFI functions:


```rust
#[diplomat::bridge]
#[diplomat::abi_rename = "mylibrary_{}"]
mod ffi {

    #[diplomat::opaque]
    struct Foo;

    impl Foo {
        pub fn bar() -> u8 {}
    }
}
```

Here, instead of the function being generated as `Foo_bar()`, it will be generated as `mylibrary_Foo_bar()` which is a more unique symbol, less likely to cause trouble for the linker.