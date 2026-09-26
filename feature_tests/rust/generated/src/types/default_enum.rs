use crate::ffi;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DefaultEnum {
    A = 0,
    B = 1,
}
impl DefaultEnum {
    pub fn new() -> DefaultEnum {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::DefaultEnum_new() }
    }
}
