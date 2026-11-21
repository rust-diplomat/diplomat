/// An  Locale, capable of representing strings like `"en-US"`.
///
/// See the [Rust documentation for `Locale`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html) for more information.
#[repr(C)]
pub struct Locale {
    // Lives in the library's memory, so it is only ever reached through `&`, `&mut`
    // or `DiplomatBox`. Not `Send`, `Sync` or `Unpin`: the library does not say
    // whether that would be safe.
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl Locale {
    /// Construct an [Locale] from a locale identifier represented as a string.
    pub fn new(name: &[u8]) -> crate::DiplomatBox<crate::Locale> {
        // Safety: `Box` returns are never null, and the caller owns them.
        unsafe { crate::DiplomatBox::from_raw(icu4x_Locale_new_mv1(crate::DiplomatSlice::new(name))) }
    }
}

extern "C" {
    fn icu4x_Locale_new_mv1(name: crate::DiplomatSlice<u8>) -> *mut crate::Locale;
}

// Safety: `icu4x_Locale_destroy_mv1` frees a `Box<Locale>` returned by the library.
unsafe impl crate::DiplomatOpaque for Locale {
    unsafe fn destroy(ptr: *mut Self) {
        icu4x_Locale_destroy_mv1(ptr)
    }
}

extern "C" {
    fn icu4x_Locale_destroy_mv1(this: *mut Locale);
}