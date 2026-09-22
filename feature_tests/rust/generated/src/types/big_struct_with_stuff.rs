#[allow(unused_imports)]
use super::ScalarPairWithPadding;
use crate::ffi;

/// Testing JS-specific layout/padding behavior
/// Also being used to test CPP backends taking structs with primitive values.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BigStructWithStuff {
    pub first: u8,
    pub second: u16,
    pub third: u16,
    pub fourth: ScalarPairWithPadding,
    pub fifth: u8,
}
impl BigStructWithStuff {
    pub fn assert_value(self, extra_val: u16) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::BigStructWithStuff_assert_value(self, extra_val) };
    }
    pub fn assert_slice(slice: &[BigStructWithStuff], second_value: u16) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::BigStructWithStuff_assert_slice(ffi::DiplomatSlice::from(slice), second_value)
        };
    }
}
