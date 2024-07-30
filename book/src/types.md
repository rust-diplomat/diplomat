# Types

Diplomat only supports a small set of types that can be passed over FFI.


 - Builtins:
     - All integers
     - `bool`
     - `char`
     - `DiplomatChar` (`u32`), which is treated as `char`-equivalent on the backend language, but need not be a valid Unicode code point for the Rust code to be sound.
     - Slices, `&[T]` where `T` is one of:
         - An integer type
         - `bool`
         - `char`
         - `DiplomatByte` (`u8`): The same as `u8` except in languages where "byte buffer" and "list of integers" are different types
     - String slices:
         - `&str`: A validated, UTF-8 string. Will be converted/validated by the target language bindings if necessary.
         - `&DiplomatStr`: An unvalidated string expected to be UTF-8.
         - `&DiplomatStr16`: An unvalidated string expected to be UTF-16.
     - [`DiplomatWriteable`](./writeable.md) for returning strings. This needs to be the last parameter of the method.
     - [`Option<&T>` ,`Option<Box<T>>`](./option.md) of opaque types
     - [`Result<T, E>` and `Option<T>`](./result.md) in return values
     - `()` as a `Result` `Ok`/`Error` type, or as a return value
 - Custom types
     - Custom [opaque types](./opaque.md) (passed as references or via `Box<T>`)
     - Custom [structs and C-like enums](./structs.md)

More types can be supported in the future (We have issues for [callbacks](https://github.com/rust-diplomat/diplomat/issues/146) and [full option types](https://github.com/rust-diplomat/diplomat/issues/246))

The _main_ distinction to keep track of is between "opaque types" and "structs": opaque types are for when you want to wrap a Rust object that has its own semantics, whereas "structs" are for when you want to transparently pass around multiple values at once (usually when you want to make an options struct as an argument, or return multiple values at once).
