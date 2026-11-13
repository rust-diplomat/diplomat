use core::fmt;
use core::mem::ManuallyDrop;

#[repr(C)]
union DiplomatResultValue<T, E> {
    ok: ManuallyDrop<T>,
    err: ManuallyDrop<E>,
}

/// A [`Result`]-like type that can be passed across the FFI boundary
/// as a value. Used internally to return [`Result`]s and [`Option`]s
/// from functions.
///
/// In `#[diplomat::bridge]` function signatures, prefer returning standard [`Result<T, E>`]
/// instead; Diplomat will automatically convert it to `DiplomatResult<T, E>`. This type
/// cannot be used in Diplomat structs.
///
/// # Examples
///
/// ```rust
/// #[diplomat::bridge]
/// mod ffi {
///     #[diplomat::opaque]
///     pub struct Foo;
///
///     impl Foo {
///         // Preferred: use `Result<T, E>` in function signatures
///         pub fn bar() -> Result<i32, ()> {
///             Ok(42)
///         }
///
///         // Equivalent without sugar:
///         pub fn bar_explicit() -> DiplomatResult<i32, ()> {
///             Ok(42).into()
///         }
///     }
/// }
/// ```
#[repr(C)]
pub struct DiplomatResult<T, E> {
    value: DiplomatResultValue<T, E>,
    /// Whether the result is `Ok`.
    ///
    /// Note: Mutating this field directly in Rust is unsound, and this field will become
    /// private in `diplomat-runtime` 1.0 ([#1239](https://github.com/rust-diplomat/diplomat/issues/1239)).
    /// Prefer calling [`DiplomatResult::is_ok()`] or [`DiplomatResult::is_err()`].
    pub is_ok: bool,
}

/// A type to represent `Option<T>` over FFI.
///
/// Used internally to handle `Option<T>` arguments and return types, and needs to be
/// used explicitly for optional struct fields of non-reference types (primitives, structs,
/// enums, and slices).
///
/// In `#[diplomat::bridge]` function signatures (and for optional references to opaque types
/// like `Option<&T>`), prefer using standard [`Option<T>`] instead.
///
/// # Examples
///
/// ```rust
/// #[diplomat::bridge]
/// mod ffi {
///     pub struct MyStruct {
///         pub field: DiplomatOption<i32>,
///     }
///
///     impl MyStruct {
///         pub fn print(self) {
///             if let Some(val) = self.field.into_option() {
///                 println!("{val}");
///             }
///         }
///     }
///
///     #[diplomat::opaque]
///     pub struct Foo;
///
///     impl Foo {
///         // In function signatures, prefer `Option<T>`:
///         pub fn print(val: Option<i32>) {
///             if let Some(val) = val {
///                 println!("{val}");
///             }
///         }
///     }
/// }
/// ```
pub type DiplomatOption<T> = DiplomatResult<T, ()>;

impl<T, E> DiplomatResult<T, E> {
    /// Returns `true` if the result is `Ok`.
    #[inline]
    pub const fn is_ok(&self) -> bool {
        self.is_ok
    }

    /// Returns `true` if the result is `Err`.
    #[inline]
    pub const fn is_err(&self) -> bool {
        !self.is_ok
    }

    pub fn as_ref(&self) -> Result<&T, &E> {
        // Safety: we're only accessing the union variants when the flag is correct
        unsafe {
            if self.is_ok {
                Ok(&self.value.ok)
            } else {
                Err(&self.value.err)
            }
        }
    }
}

impl<T> DiplomatOption<T> {
    /// Helper for converting into an Option to avoid trait ambiguity errors with Into
    #[inline]
    pub fn into_option(self) -> Option<T> {
        self.into()
    }

    /// Helper for converting into an Option with the inner type converted
    #[inline]
    pub fn into_converted_option<U: From<T>>(self) -> Option<U> {
        self.into_option().map(Into::into)
    }

    pub fn map<U, F>(mut self, f: F) -> DiplomatOption<U>
    where
        F: FnOnce(T) -> U,
    {
        if self.is_ok {
            unsafe {
                let res = f(ManuallyDrop::take(&mut self.value.ok));
                DiplomatResult {
                    value: DiplomatResultValue {
                        ok: ManuallyDrop::new(res),
                    },
                    is_ok: true,
                }
            }
        } else {
            unsafe {
                DiplomatResult {
                    value: DiplomatResultValue {
                        err: self.value.err,
                    },
                    is_ok: false,
                }
            }
        }
    }
}

impl<T: Clone, E: Clone> Clone for DiplomatResult<T, E> {
    fn clone(&self) -> Self {
        unsafe {
            if self.is_ok {
                Ok((*self.value.ok).clone()).into()
            } else {
                Err((*self.value.err).clone()).into()
            }
        }
    }
}

impl<T, E> Drop for DiplomatResult<T, E> {
    fn drop(&mut self) {
        unsafe {
            if self.is_ok {
                let _ = ManuallyDrop::take(&mut self.value.ok);
            } else {
                let _ = ManuallyDrop::take(&mut self.value.err);
            }
        }
    }
}

impl<T, E> From<Result<T, E>> for DiplomatResult<T, E> {
    fn from(result: Result<T, E>) -> Self {
        match result {
            Result::Ok(ok) => DiplomatResult {
                value: DiplomatResultValue {
                    ok: ManuallyDrop::new(ok),
                },
                is_ok: true,
            },

            Result::Err(err) => DiplomatResult {
                value: DiplomatResultValue {
                    err: ManuallyDrop::new(err),
                },
                is_ok: false,
            },
        }
    }
}

impl<T> From<Option<T>> for DiplomatOption<T> {
    fn from(option: Option<T>) -> Self {
        option.ok_or(()).into()
    }
}

impl<T> From<DiplomatOption<T>> for Option<T> {
    fn from(result: DiplomatOption<T>) -> Self {
        Result::<T, ()>::from(result).ok()
    }
}

impl<T, E> From<DiplomatResult<T, E>> for Result<T, E> {
    fn from(mut result: DiplomatResult<T, E>) -> Result<T, E> {
        unsafe {
            if result.is_ok {
                Ok(ManuallyDrop::take(&mut result.value.ok))
            } else {
                Err(ManuallyDrop::take(&mut result.value.err))
            }
        }
    }
}

impl<T: fmt::Debug, E: fmt::Debug> fmt::Debug for DiplomatResult<T, E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}
