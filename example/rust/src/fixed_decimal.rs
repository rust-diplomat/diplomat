/// See the [Rust documentation for `FixedDecimal`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html) for more information.
#[repr(C)]
pub struct FixedDecimal {
    // Lives in the library's memory, so it is only ever reached through `&`, `&mut`
    // or `DiplomatBox`. Not `Send`, `Sync` or `Unpin`: the library does not say
    // whether that would be safe.
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl FixedDecimal {
    /// Construct an [FixedDecimal] from an integer.
    pub fn new(v: i32) -> crate::DiplomatBox<crate::FixedDecimal> {
        // Safety: `Box` returns are never null, and the caller owns them.
        unsafe { crate::DiplomatBox::from_raw(icu4x_FixedDecimal_new_mv1(v)) }
    }

    /// Multiply the [FixedDecimal] by a given power of ten.
    ///
    /// See the [Rust documentation for `multiply_pow10`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.multiply_pow10) for more information.
    pub fn multiply_pow10(&mut self, power: i16) {
        unsafe { icu4x_FixedDecimal_multiply_pow10_mv1(self, power) }
    }
}

extern "C" {
    fn icu4x_FixedDecimal_new_mv1(v: i32) -> *mut crate::FixedDecimal;
    fn icu4x_FixedDecimal_multiply_pow10_mv1(this: *mut FixedDecimal, power: i16);
}

// Safety: `icu4x_FixedDecimal_destroy_mv1` frees a `Box<FixedDecimal>` returned by the library.
unsafe impl crate::DiplomatOpaque for FixedDecimal {
    unsafe fn destroy(ptr: *mut Self) {
        icu4x_FixedDecimal_destroy_mv1(ptr)
    }
}

extern "C" {
    fn icu4x_FixedDecimal_destroy_mv1(this: *mut FixedDecimal);
}