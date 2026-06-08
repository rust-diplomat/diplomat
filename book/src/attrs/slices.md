# Slices

## Static Slices

({{get_supports("static_slices")}})

Support for `&'static` slices.

## Primitive Structs

({{get_supports("abi_compatibles")}})

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

## Opaques
({{get_supports("opaque_slices")}})

Currently, opaque slices are restricted to input only, with no borrows allowed in the output (See [the relevant issue on GitHub](https://github.com/rust-diplomat/diplomat/issues/1168)):

```rs
// Allowed:
pub fn takes_opaque_slice<'a>(&'a self, sl: &'a [MyOpaque]);

// Will throw an error on lowering:
pub fn borrows_slice_opaque<'a>(sl : &'a [MyOpaque]) -> &'a SomeType;
```