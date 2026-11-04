/// The type behind `FixedDecimalFormatter` pointers at the ABI level.
#[repr(C)]
pub(crate) struct FixedDecimalFormatterRaw {
    _private: [u8; 0],
}

/// An  Fixed Decimal Format object, capable of formatting a [FixedDecimal] as a string.
///
/// See the [Rust documentation for `FixedDecimalFormatter`](https://docs.rs/icu/latest/icu/decimal/struct.FixedDecimalFormatter.html) for more information.
pub struct FixedDecimalFormatter(pub(crate) core::ptr::NonNull<FixedDecimalFormatterRaw>);

impl FixedDecimalFormatter {
    /// Creates a new [FixedDecimalFormatter] from locale data.
    ///
    /// See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/decimal/struct.FixedDecimalFormatter.html#method.try_new) for more information.
    pub fn try_new(locale: &crate::Locale, provider: &crate::DataProvider, options: crate::FixedDecimalFormatterOptions) -> Result<crate::FixedDecimalFormatter, ()> {
        // Safety: `Box` returns are never null, and the wrapper takes ownership.
        unsafe { icu4x_FixedDecimalFormatter_try_new_mv1(locale.0.as_ptr(), provider.0.as_ptr(), options) }
            .into_result()
            .map(|ok| unsafe { crate::FixedDecimalFormatter(core::ptr::NonNull::new_unchecked(ok)) })
    }
}

extern "C" {
    fn icu4x_FixedDecimalFormatter_try_new_mv1(locale: *const crate::locale::LocaleRaw, provider: *const crate::data_provider::DataProviderRaw, options: crate::FixedDecimalFormatterOptions) -> crate::DiplomatResult<*mut crate::fixed_decimal_formatter::FixedDecimalFormatterRaw, ()>;
}

impl Drop for FixedDecimalFormatter {
    fn drop(&mut self) {
        unsafe { icu4x_FixedDecimalFormatter_destroy_mv1(self.0.as_ptr()) }
    }
}

extern "C" {
    fn icu4x_FixedDecimalFormatter_destroy_mv1(this: *mut FixedDecimalFormatterRaw);
}
