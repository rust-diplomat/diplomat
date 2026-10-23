/// The type behind `FixedDecimalFormatter` pointers at the ABI level.
#[repr(C)]
pub(crate) struct FixedDecimalFormatterRaw {
    _private: [u8; 0],
}

/// An  Fixed Decimal Format object, capable of formatting a [FixedDecimal] as a string.
///
/// See the [Rust documentation for `FixedDecimalFormatter`](https://docs.rs/icu/latest/icu/decimal/struct.FixedDecimalFormatter.html) for more information.
pub struct FixedDecimalFormatter(pub(crate) core::ptr::NonNull<FixedDecimalFormatterRaw>);

impl Drop for FixedDecimalFormatter {
    fn drop(&mut self) {
        unsafe { icu4x_FixedDecimalFormatter_destroy_mv1(self.0.as_ptr()) }
    }
}

extern "C" {
    fn icu4x_FixedDecimalFormatter_destroy_mv1(this: *mut FixedDecimalFormatterRaw);
}
