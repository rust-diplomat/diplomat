#[allow(unused_imports)]
use super::UnimportedEnum;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImportedStruct {
    pub foo: UnimportedEnum,
    pub count: u8,
}
