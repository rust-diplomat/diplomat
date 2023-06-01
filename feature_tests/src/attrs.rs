#[diplomat::bridge]
pub mod ffi {
    #[diplomat::opaque]
    pub struct AttrOpaque1;

    impl AttrOpaque1 {
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
