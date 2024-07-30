# Renaming APIs

(Supported by all backends)

Any type, method, field, or enum variant can be renamed with the `rename` attribute.


```rust
#[diplomat::bridge]
mod ffi {
    #[diplomat::opaque]
    #[diplomat::attr(cpp, rename = "Foo2")]
    struct Foo;

    impl Foo {
        #[diplomat::attr(js, rename = "barbar")]
        pub fn bar() {}
        pub fn baz() {}
    }
}
```

Here, C++ will see a class `Foo2` instead of `Foo`, and in JS the method `bar()` will be renamed to `barbar()`.

Note that some backends apply some of their own renames as well, which will be applied on top of the attribute rename. For example, the JS and Dart backends both turn `snake_case` into `camelCase` for better idiomaticity. Renaming to `bar_bar` would then produce `barBar()` in these backends.
