# Result types

Result types are returned by using [`Result<T, E>`](https://docs.rs/diplomat-runtime/0.2.0/diplomat_runtime/struct.DiplomatResult.html) (or `DiplomatResult<T, E>`).

For example, let's say we wish to define a fallible constructor:

```rust
#[diplomat::bridge]
mod ffi {
    #[diplomat::opaque]
    struct Thingy(u8);

    impl Thingy {
        pub fn try_create(string: &str) -> Result<Box<Thingy>, ()> {
            let parsed: Result<u8, ()> = string.parse().map_err(|_| ());
            parsed.map(Thingy).map(Box::new)
        }
    }
}
```

On the C++ side, this will generate a method on `Thingy` with the signature

```cpp
  static diplomat::result<std::unique_ptr<Thingy>, std::monostate> try_create(const std::string_view string);
```

`diplomat::result` is a type that can be found in the generated [`diplomat_runtime.hpp`](https://github.com/rust-diplomat/diplomat/blob/main/tool/src/cpp/runtime.hpp) file. The most basic APIs are `.is_ok()` and `.is_err()`, returning `bool`s, and `.ok()` and `.err()` returning `std::option`s. There are further APIs for constructing and manipulating these that can be found in the header file.

On the JS side it will continue to return the `Thingy` class but it will `throw` the error (as an empty object in this case) in case of an error.
