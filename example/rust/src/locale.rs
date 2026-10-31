/// The type behind `Locale` pointers at the ABI level.
#[repr(C)]
pub(crate) struct LocaleRaw {
    _private: [u8; 0],
}

/// An  Locale, capable of representing strings like `"en-US"`.
///
/// See the [Rust documentation for `Locale`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html) for more information.
pub struct Locale(pub(crate) core::ptr::NonNull<LocaleRaw>);

impl Locale {
    /// Construct an [Locale] from a locale identifier represented as a string.
    pub fn new(name: &[u8]) -> crate::Locale {
        // Safety: `Box` returns are never null, and the wrapper takes ownership.
        unsafe {
            crate::Locale(core::ptr::NonNull::new_unchecked(
                icu4x_Locale_new_mv1(crate::DiplomatSlice::new(name)),
            ))
        }
    }
}

extern "C" {
    fn icu4x_Locale_new_mv1(name: crate::DiplomatSlice<u8>) -> *mut crate::locale::LocaleRaw;
}

impl Drop for Locale {
    fn drop(&mut self) {
        unsafe { icu4x_Locale_destroy_mv1(self.0.as_ptr()) }
    }
}

extern "C" {
    fn icu4x_Locale_destroy_mv1(this: *mut LocaleRaw);
}
