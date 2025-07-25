# Slices

## Static Slices

(Supported by `c`, `cpp`, `kotlin`, `nanobind`, queried with `supports=static_slices`)

Support for `&'static` slices.

## Primitive Structs

(Supported by `c`, `cpp`, `nanobind`, queried with `supports=abi_compatibles`)

Some Diplomat backends support providing slices of structs as function parameters:

```rs
#[diplomat::bridge]
mod ffi {
    #[diplomat::attr(auto, abi_compatible)]
    struct Struct {
        a : bool,
        b : i32
    }

    impl Struct {
        pub fn bar(slice : &mut [Struct]) {}
        pub fn baz(other_slice : &[Struct]) {}
    }
}
```

Mapping Diplomat types from the bound language to the C API can vary based on what types are in the struct. To reduce this complexity, slices of structs come with some restrictions:

- The struct type must have `#[diplomat::attr(auto, abi_compatible)]` attribute.
- The struct can only have fields that are either:
    - Primitive
    - Nested primitive structs
