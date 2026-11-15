#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CyclicStructB {
    pub field: u8,
}

impl CyclicStructB {
    pub fn get_a() -> crate::CyclicStructA {
        unsafe { CyclicStructB_get_a() }
    }
}

extern "C" {
    fn CyclicStructB_get_a() -> crate::CyclicStructA;
}
