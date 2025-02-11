# Option types

Option types in Diplomat are relatively straightforward, you simply use `Option<T>` and it turns into the idiomatic equivalent over FFI.

`Option<T>` currently only works when wrapping reference types (`Box<OpaqueType>` and `&OpaqueType`), structs/enums/primitives, or slices of the above. It may be used as an input argument, or in return type position:

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

        pub fn increment_option(x: Option<u8>) -> Option<u8> {
            x.map(|inner| inner + 1)
        }
    }
}
```

In C++ `maybe_create` will return a `std::optional<std::unique_ptr<Thingy>>`, and in JS it will return a potentially-null object.

`make_option` will have similar behavior, returning `std::optional<uint8_t>` and an integer-or-null in JS. It will accept `std::optional<uint8_t>` in C++ and null-check the parameter in JS.

## DiplomatOption

`Option<T>` is FFI-safe for reference types but not for other arbitrary types. When used in function parameters, Diplomat will automatically use FFI-safe types over the boundary, however with structs layout concerns prevent automatically doing this. Instead, if you wish to use an `Option<T>` in a struct (for struct, enum, or primitive `T`), use `DiplomatOption<T>`

```rust
#[diplomat::bridge]
mod ffi {
    use diplomat_runtime::DiplomatOption;

    #[diplomat::opaque]
    pub struct MyOpaque(u8);

    pub enum MyEnum {
        Foo, Bar
    }

    pub struct MyStruct<'a> {
        a: DiplomatOption<u8>,
        b: DiplomatOption<MyEnum>,
        c: Option<&'a MyOpaque>
    }
}
```