/// The type behind `FixedDecimal` pointers at the ABI level.
#[repr(C)]
pub(crate) struct FixedDecimalRaw {
    _private: [u8; 0],
}

/// See the [Rust documentation for `FixedDecimal`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html) for more information.
pub struct FixedDecimal(pub(crate) core::ptr::NonNull<FixedDecimalRaw>);

impl FixedDecimal {
    /// Construct an [FixedDecimal] from an integer.
    pub fn new(v: i32) -> crate::FixedDecimal {
        // Safety: `Box` returns are never null, and the wrapper takes ownership.
        unsafe {
            crate::FixedDecimal(core::ptr::NonNull::new_unchecked(
                icu4x_FixedDecimal_new_mv1(v),
            ))
        }
    }

    /// Multiply the [FixedDecimal] by a given power of ten.
    ///
    /// See the [Rust documentation for `multiply_pow10`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.multiply_pow10) for more information.
    pub fn multiply_pow10(&mut self, power: i16) {
        unsafe { icu4x_FixedDecimal_multiply_pow10_mv1(self.0.as_ptr(), power) }
    }
}

impl Drop for FixedDecimal {
    fn drop(&mut self) {
        unsafe { icu4x_FixedDecimal_destroy_mv1(self.0.as_ptr()) }
    }
}

extern "C" {
    fn icu4x_FixedDecimal_new_mv1(v: i32) -> *mut crate::fixed_decimal::FixedDecimalRaw;
    fn icu4x_FixedDecimal_multiply_pow10_mv1(this: *mut FixedDecimalRaw, power: i16);
    fn icu4x_FixedDecimal_destroy_mv1(this: *mut FixedDecimalRaw);
}
