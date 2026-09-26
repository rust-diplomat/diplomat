#[allow(unused_imports)]
use super::CyclicStructB;
use crate::ffi;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CyclicStructA {
    pub a: CyclicStructB,
}
impl CyclicStructA {
    pub fn get_b() -> CyclicStructB {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::CyclicStructA_get_b() }
    }
    pub fn cyclic_out(self) -> String {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        crate::private::with_write(|write| {
            unsafe { ffi::CyclicStructA_cyclic_out(self, write) };
        })
        .1
    }
    pub fn nested_slice(sl: &[CyclicStructA]) -> u8 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::CyclicStructA_nested_slice(ffi::DiplomatSlice::from(sl)) }
    }
    pub fn double_cyclic_out(self, cyclic_struct_a: CyclicStructA) -> String {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        crate::private::with_write(|write| {
            unsafe { ffi::CyclicStructA_double_cyclic_out(self, cyclic_struct_a, write) };
        })
        .1
    }
    pub fn getter_out(self) -> String {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        crate::private::with_write(|write| {
            unsafe { ffi::CyclicStructA_getter_out(self, write) };
        })
        .1
    }
}
