/// An  Fixed Decimal Format object, capable of formatting a [FixedDecimal] as a string.
///
/// See the [Rust documentation for `FixedDecimalFormatter`](https://docs.rs/icu/latest/icu/decimal/struct.FixedDecimalFormatter.html) for more information.
#[repr(C)]
pub struct FixedDecimalFormatter {
    // Lives in the library's memory, so it is only ever reached through `&`, `&mut`
    // or `DiplomatBox`. Not `Send`, `Sync` or `Unpin`: the library does not say
    // whether that would be safe.
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl FixedDecimalFormatter {
    /// Creates a new [FixedDecimalFormatter] from locale data.
    ///
    /// See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/decimal/struct.FixedDecimalFormatter.html#method.try_new) for more information.
    pub fn try_new(locale: &crate::Locale, provider: &crate::DataProvider, options: crate::FixedDecimalFormatterOptions) -> Result<crate::DiplomatBox<crate::FixedDecimalFormatter>, ()> {
        // Safety: `Box` returns are never null, and the caller owns them.
        unsafe { icu4x_FixedDecimalFormatter_try_new_mv1(locale, provider, options) }
            .into_result()
            .map(|ok| unsafe { crate::DiplomatBox::from_raw(ok) })
    }
}

extern "C" {
    fn icu4x_FixedDecimalFormatter_try_new_mv1(locale: *const crate::Locale, provider: *const crate::DataProvider, options: crate::FixedDecimalFormatterOptions) -> crate::DiplomatResult<*mut crate::FixedDecimalFormatter, ()>;
}

// Safety: `icu4x_FixedDecimalFormatter_destroy_mv1` frees a `Box<FixedDecimalFormatter>` returned by the library.
unsafe impl crate::DiplomatOpaque for FixedDecimalFormatter {
    unsafe fn destroy(ptr: *mut Self) {
        icu4x_FixedDecimalFormatter_destroy_mv1(ptr)
    }
}

extern "C" {
    fn icu4x_FixedDecimalFormatter_destroy_mv1(this: *mut FixedDecimalFormatter);
}