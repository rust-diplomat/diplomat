# Returning strings: Writeables

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

In C++ these become APIs that return `std::string` and `diplomat::result<std::string, std::monostate>` respectively.


Essentially, versions of the API returning `std::string` are generated, where the `write!()` operation will end up writing _directly to the `std::string`_ with no additional intermediate Rust `String` allocations.

