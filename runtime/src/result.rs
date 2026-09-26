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
#[repr(C)]
pub struct DiplomatResult<T, E> {
    value: DiplomatResultValue<T, E>,
    pub is_ok: bool,
}

/// A type to represent `Option<T>` over FFI.
///
/// Used internally to handle `Option<T>` arguments and return types, and needs to be
/// used explicitly for optional struct fields.
pub type DiplomatOption<T> = DiplomatResult<T, ()>;

impl<T, E> DiplomatResult<T, E> {
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
    fn from(result: DiplomatResult<T, E>) -> Result<T, E> {
        // The payload is moved out below with `ManuallyDrop::take`, which is a bitwise
        // copy and leaves the union field looking untouched. If `DiplomatResult`'s own
        // `Drop` were still allowed to run it would take — and drop — the same bytes a
        // second time: double counting for a plain value, a double free for a
        // pointer-carrying one. Suppress the container's destructor for the whole
        // conversion, the way `From<DiplomatOwnedSlice<T>> for Box<[T]>` does in
        // `slices.rs`. Only one union field is ever live and it is being taken out here,
        // so nothing leaks.
        let mut result = ManuallyDrop::new(result);
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

#[cfg(test)]
mod tests {
    use core::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    /// A payload with drop glue.
    ///
    /// The conversions below read a payload out of the union with `ManuallyDrop::take`,
    /// which is a bitwise copy that leaves the source bytes intact. Nothing marks the
    /// union field as taken, so if the container's own `Drop` is still allowed to run it
    /// takes the payload a second time and drops it again. For a plain counter that is
    /// double counting; for a pointer-carrying payload it is a double free. Each test
    /// owns its own counter so the tests cannot race on a shared static.
    struct Counted(&'static AtomicUsize);

    impl Drop for Counted {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    static OK_DROPS: AtomicUsize = AtomicUsize::new(0);
    static ERR_DROPS: AtomicUsize = AtomicUsize::new(0);
    static OPTION_DROPS: AtomicUsize = AtomicUsize::new(0);

    #[test]
    fn converting_a_successful_result_takes_the_payload_exactly_once() {
        OK_DROPS.store(0, Ordering::SeqCst);

        let raw: DiplomatResult<Counted, ()> = DiplomatResult::from(Ok(Counted(&OK_DROPS)));
        let converted: Result<Counted, ()> = raw.into();

        assert_eq!(
            OK_DROPS.load(Ordering::SeqCst),
            0,
            "the conversion must move the payload, not drop it"
        );
        drop(converted);
        assert_eq!(
            OK_DROPS.load(Ordering::SeqCst),
            1,
            "the payload must be dropped exactly once"
        );
    }

    #[test]
    fn converting_an_unsuccessful_result_takes_the_error_exactly_once() {
        ERR_DROPS.store(0, Ordering::SeqCst);

        let raw: DiplomatResult<(), Counted> = DiplomatResult::from(Err(Counted(&ERR_DROPS)));
        let converted: Result<(), Counted> = raw.into();

        assert_eq!(
            ERR_DROPS.load(Ordering::SeqCst),
            0,
            "the conversion must move the error, not drop it"
        );
        drop(converted);
        assert_eq!(
            ERR_DROPS.load(Ordering::SeqCst),
            1,
            "the error must be dropped exactly once"
        );
    }

    #[test]
    fn converting_a_fulfilled_option_takes_the_payload_exactly_once() {
        OPTION_DROPS.store(0, Ordering::SeqCst);

        let raw: DiplomatOption<Counted> = DiplomatOption::from(Some(Counted(&OPTION_DROPS)));
        let converted: Option<Counted> = raw.into();

        assert_eq!(
            OPTION_DROPS.load(Ordering::SeqCst),
            0,
            "the conversion must move the payload, not drop it"
        );
        drop(converted);
        assert_eq!(
            OPTION_DROPS.load(Ordering::SeqCst),
            1,
            "the payload must be dropped exactly once"
        );
    }
}
