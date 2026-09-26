use crate::ffi;

/// Testing JS-specific layout/padding behavior
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScalarPairWithPadding {
    pub first: u8,
    pub second: u32,
}
impl ScalarPairWithPadding {
    pub fn assert_value(self) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::ScalarPairWithPadding_assert_value(self) };
    }
}
