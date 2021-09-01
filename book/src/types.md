# Types

Diplomat only supports a small set of types that can be passed over FFI. These are:

 - All integers, as well as `bool` and `char`
 - String slices
 - References (mutable and immutable)
 - `Box<T>`
 - [Opaque types](./opaque.md)
 - [Structs and C-like enums](./structs.md)
 - [`Option<T>`](./option.md)
 - [`DiplomatResult<T, E>`](./result.md)
 - [`DiplomatWriteable`](./writeable.md) for returning strings
 - `()`

If these types contain other types (e.g. the fields of a struct, or the `T/E` values of `DiplomatResult`), they must also be "allowed" types.

More types can be supported in the future. Many of these types have restrictions, for example opaque types must be behind some form of pointer, and `()` only works in return positions.
