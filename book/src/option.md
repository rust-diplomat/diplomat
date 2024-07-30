# Option types

Option types in Diplomat are relatively straightforward, you simply use `Option<T>` and it turns into the idiomatic equivalent over FFI.

`Option<T>` currently only works when wrapping reference types (`Box<OpaqueType>` and `&OpaqueType`), or in return type position:

```rust
#[diplomat::bridge]
mod ffi {
    // just exists so we can get methods
    #[diplomat::opaque]
    pub struct Thingy;

    impl Thingy {
        pub fn maybe_create() -> Option<Box<Thingy>> {
            Some(Box::new(Thingy))
        }

        // works in return position, but not elsewhere
        pub fn make_option() -> Option<u8> {
            Some(1)
        }
    }
}
```

In C++ `maybe_create` will return a `std::optional<std::unique_ptr<Thingy>>`, and in JS it will return a potentially-null object.

`make_option` will have similar behavior, returning `std::optional<uint8_t>` and an integer-or-null in JS.


In the future, `Option<T>` will be supported for most types `T` ([#246](https://github.com/rust-diplomat/diplomat/issues/246)).