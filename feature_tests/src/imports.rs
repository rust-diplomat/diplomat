#[diplomat::bridge]
pub mod ffi {
    #[diplomat::attr(kotlin, disable)]
    pub enum UnimportedEnum {
        A,
        B,
        C,
    }

    #[diplomat::attr(kotlin, disable)]
    pub struct ImportedStruct {
        foo: UnimportedEnum,
        count: u8,
    }
}
