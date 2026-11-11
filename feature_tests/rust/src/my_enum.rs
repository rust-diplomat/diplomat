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
        unsafe { MyEnum_into_value(self) }
    }

    pub fn get_a() -> crate::MyEnum {
        unsafe { MyEnum_get_a() }
    }
}

extern "C" {
    fn MyEnum_into_value(this: crate::MyEnum) -> i8;
    fn MyEnum_get_a() -> crate::MyEnum;
}
