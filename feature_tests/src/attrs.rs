#[diplomat::bridge]
#[diplomat::c_rename = "namespace_{0}"]
pub mod ffi {
    #[diplomat::opaque]
    #[diplomat::attr(cpp2, rename = "AttrOpaque1Renamed")]
    pub struct AttrOpaque1;

    impl AttrOpaque1 {
        #[diplomat::attr(cpp2, rename = "totally_not_{0}")]
        pub fn new() -> Box<AttrOpaque1> {
            Box::new(AttrOpaque1)
        }

        #[diplomat::attr(cpp2, rename = "method_renamed")]
        pub fn method(&self) -> u8 {
            77
        }

        #[diplomat::c_rename("renamed_in_c_only")]
        pub fn crenamed(&self) -> u8 {
            123
        }

        #[diplomat::attr(cpp2, disable)]
        pub fn method_disabledcpp(&self) {
            println!("disabled in cpp");
        }
    }

    #[diplomat::opaque]
    #[diplomat::attr(cpp2, disable)]
    pub struct AttrOpaque2;

    pub enum AttrEnum {
        A,
        B,
        #[diplomat::attr(cpp2, rename = "CRenamed")]
        C,
    }
}
