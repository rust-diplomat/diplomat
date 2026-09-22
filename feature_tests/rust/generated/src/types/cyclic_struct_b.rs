#[allow(unused_imports)]
use super::CyclicStructA;
use crate::ffi;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CyclicStructB {
    pub field: u8,
}
impl CyclicStructB {
    pub fn get_a() -> CyclicStructA {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::CyclicStructB_get_a() }
    }
    pub fn get_a_option() -> Option<CyclicStructA> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::CyclicStructB_get_a_option() };
        result.into()
    }
}
