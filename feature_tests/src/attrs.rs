#[diplomat::bridge]
pub mod ffi {
    #[diplomat::opaque]
    pub struct AttrOpaque1;

    impl AttrOpaque1 {
        pub fn method(&self) {
            println!("method");
        }

        pub fn method_disabledcpp(&self) {
            println!("disabled in cpp");
        }
    }

    #[diplomat::opaque]
    pub struct AttrOpaque2;
}