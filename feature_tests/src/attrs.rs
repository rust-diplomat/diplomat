#[diplomat::bridge]
pub mod ffi {
    #[diplomat::opaque]
    #[diplomat::attr(cpp2, rename = "AttrOpaque1Renamed")]
    pub struct AttrOpaque1;

    impl AttrOpaque1 {
        #[diplomat::attr(cpp2, rename = "method_renamed")]
        pub fn method(&self) {
            println!("method");
        }

        #[diplomat::attr(cpp2, disable)]
        pub fn method_disabledcpp(&self) {
            println!("disabled in cpp");
        }
    }

    #[diplomat::opaque]
    #[diplomat::attr(cpp2, disable)]
    pub struct AttrOpaque2;
}
