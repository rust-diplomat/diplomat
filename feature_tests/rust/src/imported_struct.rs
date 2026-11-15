#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImportedStruct {
    pub foo: crate::UnimportedEnum,
    pub count: u8,
}
