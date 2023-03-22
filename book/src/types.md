# Types

Diplomat only supports a small set of types that can be passed over FFI. These are:

 - Builtins:
     - All integers, as well as `bool` and `char`
     - `&[T]` where `T` is an integer, `bool`, or `char`
     - `&str` (string slices)
     - [`DiplomatWriteable`](./writeable.md) for returning strings
     - [`Result<T, E>`](./result.md) in return values
     - [`Option<T>`](./option.md) of opaque types
     - `()` as a `Result` `Ok`/`Error` type, or as a return value
 - Custom types
     - Custom [opaque types](./opaque.md) (passed as references or via `Box<T>`)
     - Custom [structs and C-like enums](./structs.md)

More types can be supported in the future (We have issues for [iterators](https://github.com/rust-diplomat/diplomat/issues/251) and [callbacks](https://github.com/rust-diplomat/diplomat/issues/146))

The _main_ distinction to keep track of is between "opaque types" and "structs": opaque types are for when you want to wrap a Rust object that has its own semantics, whereas "structs" are for when you want to transparently pass around multiple values at once (usually when you want to make an options struct as an argument, or return multiple values at once).
