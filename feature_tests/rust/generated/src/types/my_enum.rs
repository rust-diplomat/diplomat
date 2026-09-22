use crate::ffi;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MyEnum {
    A = -2,
    B = -1,
    C = 0,
    D = 1,
    /// EEEEEEE
    E = 2,
    F = 3,
}
impl MyEnum {
    pub fn into_value(self) -> i8 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::MyEnum_into_value(self) }
    }
    pub fn get_a() -> MyEnum {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::MyEnum_get_a() }
    }
}
