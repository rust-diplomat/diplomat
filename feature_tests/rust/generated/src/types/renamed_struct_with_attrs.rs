use crate::ffi;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RenamedStructWithAttrs {
    pub a: bool,
    pub b: u32,
}
impl RenamedStructWithAttrs {
    pub fn new_fallible(a: bool, b: u32) -> Result<RenamedStructWithAttrs, ()> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::namespace_StructWithAttrs_new_fallible(a, b) };
        result.into()
    }
    pub fn c(self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::namespace_StructWithAttrs_c(self) }
    }
    pub fn deprecated(self) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::namespace_StructWithAttrs_deprecated(self) };
    }
}
