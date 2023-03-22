# Writeables

Most languages have their own type to handle strings. To avoid unnecessary allocations, Diplomat supports [`DiplomatWriteable`](https://docs.rs/diplomat-runtime/0.2.0/diplomat_runtime/struct.DiplomatWriteable.html), a type with a `Write` implementation which can be used to write to appropriate string types on the other side.

For example, if we want to have methods that philosophically return a `String` or a `Result<String>`, we can do the following:

```rust
#[diplomat::bridge]
mod ffi {
    use diplomat_runtime::DiplomatWriteable;
    use std::fmt::Write;

    #[diplomat::opaque]
    #[derive(Debug)]
    pub struct Thingy(u8);

    impl Thingy {
        pub fn debug_output(&self, writeable: &mut DiplomatWriteable) {
            write!(writeable, "{:?}", self);
        }

        pub fn maybe_get_string(&self, writeable: &mut DiplomatWriteable) -> Result<(), ()> {
            write!(writeable, "integer is {}", self.0).map_err(|_| ())
        }
    }
}
```

On the JS side these will get converted to APIs that return strings (`maybe_get_string` will potentially throw in the case of an error, as is usual with `DiplomatResult`)

In C++ multiple APIs are generated: 

```cpp
  std::string debug_output();
  diplomat::result<std::string, std::monostate> maybe_get_string();
// and
  template<typename W> void debug_output_to_writeable(W& writeable);
  template<typename W> diplomat::result<std::monostate, std::monostate> maybe_get_string_to_writeable(W& writeable);
```

Essentially, versions of the API returning `std::string` are generated, where the `write!()` operation will end up writing _directly to the `std::string`_ with no additional intermediate Rust `String` allocations.

## WriteableTrait

The template versions work on any type that is hooked into `WriteableTrait`, allowing . Types can be hooked into `WriteableTrait` as follows:

```cpp
template<> struct WriteableTrait<MyStringType> {
  static inline capi::DiplomatWriteable Construct(MyStringType& t) {
    // ...
  }
}
```

This requires constructing a [`DiplomatWriteable`](https://docs.rs/diplomat-runtime/0.2.0/diplomat_runtime/struct.DiplomatWriteable.html) from the custom string type, which is documented in more detail [in the source](https://github.com/rust-diplomat/diplomat/blob/38cffa9bc2ef21d0aba89ed7d76236de4153248a/runtime/src/writeable.rs#L6-L62). Essentially, it involves constructing an ad-hoc virtual dispatch object for the type.
