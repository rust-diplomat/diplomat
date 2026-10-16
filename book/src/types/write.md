# Returning strings: Write

Most languages have their own type to handle strings. To avoid unnecessary allocations, Diplomat supports [`DiplomatWrite`](https://docs.rs/diplomat-runtime/latest/diplomat_runtime/struct.DiplomatWrite.html), a type with a `Write` implementation which can be used to write to appropriate string types on the other side.

For example, if we want to have methods that philosophically return a `String` or a `Result<String>`, we can do the following:

```rust
#[diplomat::bridge]
mod ffi {
    use diplomat_runtime::DiplomatWrite;
    use std::fmt::Write;

    #[diplomat::opaque]
    #[derive(Debug)]
    pub struct Thingy(u8);

    impl Thingy {
        pub fn debug_output(&self, write: &mut DiplomatWrite) {
            write!(write, "{:?}", self);
        }

        pub fn maybe_get_string(&self, write: &mut DiplomatWrite) -> Result<(), ()> {
            write!(write, "integer is {}", self.0).map_err(|_| ())
        }
    }
}
```

On the JS side these will get converted to APIs that return strings (`maybe_get_string` will potentially throw in the case of an error, as is usual with `DiplomatResult`)

In C++ these become APIs that return `std::string` and `diplomat::result<std::string, std::monostate>` respectively.


Essentially, versions of the API returning `std::string` are generated, where the `write!()` operation will end up writing _directly to the `std::string`_ with no additional intermediate Rust `String` allocations.

## `DiplomatWrite16` (UTF-16 Strings)

({{get_supports("utf16_writeables")}})

By default, `DiplomatWrite` buffers UTF-8 bytes (`u8`). For backends whose native string representation is UTF-16 (such as JavaScript, Dart, and Kotlin), returning a `DiplomatWrite` requires decoding UTF-8 into a UTF-16 string on the target language side.

When your Rust code can produce UTF-16 code units directly (or when you want to avoid UTF-8 decoding in UTF-16 runtimes or return `std::u16string` in C++), you can use `DiplomatWrite16` (which is a type alias for `DiplomatWriteGeneric<u16>`):

```rust
#[diplomat::bridge]
mod ffi {
    use diplomat_runtime::DiplomatWrite16;

    #[diplomat::opaque]
    pub struct Formatter;

    impl Formatter {
        #[diplomat::cfg(supports = utf16_writeables)]
        pub fn format_utf16(&self, write: &mut DiplomatWrite16) {
            // `DiplomatWrite16` provides `write_slice(&[u16])`, `push(u16)`,
            // `extend(...)`, and `write_char_as_utf16(char)` for writing UTF-16
            // code units directly.
            write.write_slice(&[0x0048, 0x0069]); // "Hi"
            write.write_char_as_utf16('🦀');
        }
    }
}
```

- In **JS, Dart, and Kotlin**, a method taking `&mut DiplomatWrite16` returns a native `String`, reading the UTF-16 buffer directly without a UTF-8 decode step.
- In **C++**, a method taking `&mut DiplomatWrite16` returns `std::u16string` (and generates a `_write` template accepting any `WriteTrait` implementation, specialized by default for `std::u16string`).
- In **C**, the parameter is exposed as `DiplomatWrite*` (since `DiplomatWrite16` is `#[repr(transparent)]` over `DiplomatWrite` with `len` and `cap` measured in bytes).

If you want to expose a UTF-8 `DiplomatWrite` method for UTF-8 backends and C++, and a `DiplomatWrite16` method as the primary method on UTF-16 backends (plus `foo16` on C/C++), you can combine `supports = utf16_writeables` with [`supports = utf8_strings`](./slices.md#strings-utf8_strings-and-utf16_strings):

```rust
#[diplomat::attr(all(supports = utf16_writeables, not(supports = utf8_strings)), disable)]
#[diplomat::attr(*, rename = "format")]
pub fn format_utf8(&self, write: &mut DiplomatWrite) {
    // ...
}

#[diplomat::cfg(supports = utf16_writeables)]
#[diplomat::attr(not(supports = utf8_strings), rename = "format")]
#[diplomat::attr(supports = utf8_strings, rename = "format16")]
pub fn format_utf16(&self, write: &mut DiplomatWrite16) {
    // ...
}
```

## `DiplomatWriteGeneric<T>` (Primitive Buffers)

({{get_supports("generic_writeables")}})

To return a dynamically-sized buffer of scalar primitives (`i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64`, `u64`, `isize`, `usize`, `f32`, `f64`) without intermediate Rust allocations, Diplomat provides `DiplomatWriteGeneric<T>`:

```rust
#[diplomat::bridge]
mod ffi {
    use diplomat_runtime::DiplomatWriteGeneric;

    #[diplomat::opaque]
    pub struct Float64Vec(Vec<f64>);

    impl Float64Vec {
        #[diplomat::cfg(supports = generic_writeables)]
        pub fn write_to(&self, write: &mut DiplomatWriteGeneric<f64>) {
            write.write_slice(&self.0);
        }

        #[diplomat::cfg(supports = generic_writeables)]
        pub fn write_doubled(&self, write: &mut DiplomatWriteGeneric<f64>) {
            write.extend(self.0.iter().map(|x| x * 2.0));
        }
    }
}
```

`DiplomatWriteGeneric<T>` supports:
- `write.push(val: T)`: Appends a single element.
- `write.write_slice(slice: &[T])`: Appends a contiguous slice of elements via `memcpy`.
- `write.extend(iter)`: Reserves capacity from the iterator's `size_hint` and appends each element.
- `write.reserve(additional: usize) -> bool`: Pre-allocates space for `additional` elements of type `T`.
- `write.len()`, `write.is_empty()`, `write.capacity()`: Queries the length and capacity in elements of type `T`.

On the target language side:
- In **C++**, a method taking `&mut DiplomatWriteGeneric<T>` returns `std::vector<T>` (or `diplomat::result<std::vector<T>, E>` / `std::optional<std::vector<T>>`), writing directly into the `std::vector<T>` allocation via `diplomat::WriteFromVector`. It also generates a `_write` method accepting any type implementing `WriteTrait`.
- In **C**, `DiplomatWriteGeneric<T>` shares the same `DiplomatWrite` C ABI struct (`len` and `cap` are measured in bytes, always a multiple of `sizeof(T)`).



