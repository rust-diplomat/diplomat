#[diplomat::bridge]
mod ffi {

    pub struct Wrapper {
        cant_be_empty: bool,
    }

    pub struct TestingStruct {
        x: i32,
        y: i32,
    }

    impl Wrapper {
        #[cfg(supports = "callbacks")]
        pub fn test_multi_arg_callback(f: impl Fn(i32) -> i32, x: i32) -> i32 {
            f(10 + x)
        }
        // #[cfg(supports = "callbacks")]
        // pub fn test_multiarg_void_callback(f: impl Fn(i32, &str)) {
        //     // note: make sure the string passed to f is a C-string (ending with \0)
        //     // if you're passing to C
        //     f(-10, "hello it's a string\0");
        // }
        // #[cfg(supports = "callbacks")]
        // pub fn test_mod_array(g: impl Fn(&[u8])) {
        //     let bytes: Vec<u8> = vec![0x11, 0x22];
        //     println!("Pre callback: {:?}", bytes);
        //     g(bytes.as_slice().into());
        //     println!("Back in Rust post callback: {:?}", bytes);
        // }
        #[cfg(supports = "callbacks")]
        pub fn test_no_args(h: impl Fn()) -> i32 {
            h();
            -5
        }
        #[cfg(supports = "callbacks")]
        pub fn test_cb_with_struct(f: impl Fn(TestingStruct) -> i32) -> i32 {
            let arg = TestingStruct {
                x: 1,
                y: 5,
            };
            f(arg)
        }
        #[cfg(supports = "callbacks")]
        pub fn test_multiple_cb_args(f: impl Fn() -> i32, g: impl Fn(i32) -> i32) -> i32 {
            f() + g(5)
        }
    }

}