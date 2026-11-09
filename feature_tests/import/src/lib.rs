#[diplomat::bridge]
pub mod ffi {
    pub struct TestStruct {
        pub x : i32,
        pub y: i32,
    }
}