#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CyclicStructA {
    pub a: crate::CyclicStructB,
}

impl CyclicStructA {
    pub fn get_b() -> crate::CyclicStructB {
        unsafe { CyclicStructA_get_b() }
    }
}

extern "C" {
    fn CyclicStructA_get_b() -> crate::CyclicStructB;
}
