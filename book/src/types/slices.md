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

## Strings (`utf8_strings` and `utf16_strings`)

(UTF-8 Strings {{get_supports("utf8_strings")}})

(UTF-16 Strings {{get_supports("utf16_strings")}})

Diplomat supports three string slice types:
- `&str` (`DiplomatUtf8StrSlice`): Validated UTF-8 string slice.
- `&DiplomatStr` (`DiplomatStrSlice`): Unvalidated (potentially ill-formed) UTF-8 string slice.
- `&DiplomatStr16` (`DiplomatStr16Slice`): Unvalidated (potentially ill-formed) UTF-16 string slice.

Every backend can marshal `&str` and `&DiplomatStr`, even if the target language's native string type is UTF-16 (by transcoding across the FFI boundary). Rather than indicating whether UTF-8 or UTF-16 marshalling is implemented at all, the `utf8_strings` and `utf16_strings` support flags indicate **which string encodings a backend wants exposed as idioms** when a Rust library provides both UTF-8 and UTF-16 variants of an API:

- **Backends with a single UTF-16 `String` type** (`js`, `dart`, `kotlin`, `dotnet`) set `utf8_strings = false` and `utf16_strings = true`. Passing `&DiplomatStr16` avoids UTF-8 transcoding (and in `.NET` can be pinned zero-copy), while exposing both `_utf8` and `_utf16` methods would produce duplicate or redundant `String` parameters.
- **Backends with a single UTF-8 `str` type** (`nanobind`) set `utf8_strings = true` and `utf16_strings = false`.
- **Backends with distinct UTF-8 and UTF-16 string types** (`c`, `cpp`) set both `utf8_strings = true` and `utf16_strings = true`, since `std::string_view` and `std::u16string_view` are distinct types and users may want either overload.

When exposing both UTF-8 and UTF-16 versions of a function, you can use `supports = utf8_strings` and `supports = utf16_strings` to give single-string-type backends a single idiomatic method while giving C and C++ both `foo` and `foo16`:

```rust
#[diplomat::bridge]
mod ffi {
    use diplomat_runtime::{DiplomatStr, DiplomatStr16};

    #[diplomat::opaque]
    pub struct Normalizer;

    impl Normalizer {
        #[diplomat::attr(not(supports = utf8_strings), disable)]
        #[diplomat::attr(*, rename = "is_normalized")]
        pub fn is_normalized_utf8(&self, s: &DiplomatStr) -> bool {
            // ...
        }

        #[diplomat::attr(not(supports = utf16_strings), disable)]
        #[diplomat::attr(not(supports = utf8_strings), rename = "is_normalized")]
        #[diplomat::attr(supports = utf8_strings, rename = "is_normalized16")]
        pub fn is_normalized_utf16(&self, s: &DiplomatStr16) -> bool {
            // ...
        }
    }
}
```

With these attributes:
- In **JS, Dart, Kotlin, and .NET** (`utf8_strings = false, utf16_strings = true`), `is_normalized_utf8` is disabled and `is_normalized_utf16` is renamed to `is_normalized` (or `isNormalized` / `IsNormalized`).
- In **Nanobind** (`utf8_strings = true, utf16_strings = false`), `is_normalized_utf16` is disabled and `is_normalized_utf8` is renamed to `is_normalized`.
- In **C and C++** (`utf8_strings = true, utf16_strings = true`), both methods are enabled as `is_normalized` (taking `std::string_view`) and `is_normalized16` (taking `std::u16string_view`).
