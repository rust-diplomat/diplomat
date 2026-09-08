# Fallible Callback Returns
{{get_supports("callback_returns_must_be_fallible")}}

For interpreted languages that with weak typing for variables, callbacks and traits pose a problem:

```rs
pub trait MyTrait {
  fn some_fn(a : i32) -> i32;
}
```

In Python, if we implement the trait:

```py
class ImplementsTrait(MyTrait):
  def some_fn(a):
    return "ABC"
```

Python does not impose type restrictions upon return. The method is returned directly into Rust[^cpp], which expects a strict `i32` value. Rust will throw an exception and halt the program. For languages that do not enforce strict typing on method returns, Diplomat requires all callbacks and traits to return `Result<T, E>`. `E` must be marked specially with the `#[diplomat::attr(*, ffi_error)]` attribute.

[^cpp]: In the Nanobind backend, the type must be converted through C++. C++ still has to throw an exception if the type fails to convert, but the principle roughly remains the same.

## FFI Error
Diplomat needs a way to specially denote to you, the bindings writer, that a conversion from a provided type to a given Rust type has failed.

### Currently Supported

#### Enums
```rs
pub enum ErrorEnum {
    A,
    #[diplomat::attr(*, ffi_error)]
    FFIError
}

pub trait SomeTrait {
    fn result_enum_okay() -> Result<T, ErrorEnum>;
}
```

Diplomat will set `ErrorEnum` to `FFIError` if a cast to the Rust type cannot be made when `result_enum_okay` is called. Note that since you define your own return types, it is acceptable to re-use existing enum variants and mark them with `ffi_error`; however, it is recommended that you pick an enum variant which best communicates the `ffi_error` failure case.

### Plans to support
Struct fields.
