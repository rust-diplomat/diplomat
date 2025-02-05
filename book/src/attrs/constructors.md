# Constructors

(Supported by `dart`, queried with `supports = constructors`. Intended to be supported in `kotlin`, `js`, and `cpp`)

Methods that return `Self` can be marked as a constructor:

```rust
#[diplomat::bridge]
mod ffi {
    #[diplomat::opaque]
    struct Foo;

    impl Foo {
        #[diplomat::attr(auto, constructor)]
        pub fn create() -> Box<Self> {
            Box::new(Foo)
        }
    }
}
```


and then instead of there being a regular `Foo::create()` method, `Foo` will now have a direct constructor `Foo()`.

Constructors can additionally be given names using `#[diplomat::attr(auto, named_constructor = "make")]`, for languages that support named constructors (`supports = named_constructors`).

Not all languages support fallible constructors, this can be queried with `supports = fallible_constructors`.