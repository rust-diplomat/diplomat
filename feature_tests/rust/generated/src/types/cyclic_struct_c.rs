#[allow(unused_imports)]
use super::CyclicStructA;
use crate::ffi;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CyclicStructC {
    pub a: CyclicStructA,
}
impl CyclicStructC {
    pub fn takes_nested_parameters(c: CyclicStructC) -> CyclicStructC {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::CyclicStructC_takes_nested_parameters(c) }
    }
    pub fn cyclic_out(self) -> String {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        crate::private::with_write(|write| {
            unsafe { ffi::CyclicStructC_cyclic_out(self, write) };
        })
        .1
    }
}
