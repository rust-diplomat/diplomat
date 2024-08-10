#[diplomat::bridge]
pub mod ffi {
    #[diplomat::attr(java, disable)]
    pub enum UnimportedEnum {
        A,
        B,
        C,
    }

    #[diplomat::attr(java, disable)]
    pub struct ImportedStruct {
        foo: UnimportedEnum,
        count: u8,
    }
}
