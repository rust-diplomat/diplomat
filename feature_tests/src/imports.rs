#[diplomat::bridge]
pub mod ffi {
    pub enum UnimportedEnum {
        A,
        B,
        C,
    }

    pub struct ImportedStruct {
        foo: UnimportedEnum,
        count: u8,
    }
}
