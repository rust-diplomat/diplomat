#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CyclicStructC {
    pub a: crate::CyclicStructA,
}

impl CyclicStructC {
    pub fn takes_nested_parameters(c: crate::CyclicStructC) -> crate::CyclicStructC {
        unsafe { CyclicStructC_takes_nested_parameters(c) }
    }
}

extern "C" {
    fn CyclicStructC_takes_nested_parameters(c: crate::CyclicStructC) -> crate::CyclicStructC;
}
