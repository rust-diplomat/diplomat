#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DefaultEnum {
    A = 0,
    B = 1,
}

impl DefaultEnum {
    pub fn new() -> crate::DefaultEnum {
        unsafe { DefaultEnum_new() }
    }
}

extern "C" {
    fn DefaultEnum_new() -> crate::DefaultEnum;
}
