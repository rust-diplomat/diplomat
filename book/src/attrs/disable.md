# Disabling APIs

(Supported by all backends)

Any type or method can be "disabled" by applying the `disable` attribute:


```rust
#[diplomat::bridge]
mod ffi {
    #[diplomat::opaque]
    #[diplomat::attr(cpp, disable)]
    struct Foo;


    impl Foo {
        #[diplomat::attr(js, disable)]
        pub fn bar() {}
        pub fn baz() {}
    }
}
```


Here, the class `Foo` will not show up in the C++ backend. It will in the JS backend, however it will not have the function `bar()`.


Currently enum variants cannot be disabled, however this is technically feasible for input-only enums and could be added if people request. It is also not possible to disable struct fields.