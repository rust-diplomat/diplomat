# Option types

Option types in Diplomat are relatively straightforward, you simply use `Option<T>` and it turns into the idiomatic equivalent over FFI.

`Option<T>` currently only works when wrapping reference types (`Box<OpaqueType>` and `&OpaqueType`).

```rust
#[diplomat::bridge]
mod ffi {
    // just exists so we can get methods
    #[diplomat::opaque]
    pub struct Thingy;

    impl Thingy {
        fn maybe_create() -> Option<Box<Thingy>> {
            Some(Box::new(Thingy))
        }
    }
}
```

In C++ this will return a `std::option<Thingy>`, and in JS it will return a potentially-null object.