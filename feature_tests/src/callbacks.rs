#[diplomat::bridge]
mod ffi {

    pub struct CallbackWrapper {
        cant_be_empty: bool,
    }

    pub struct CallbackTestingStruct {
        x: i32,
        y: i32,
    }

    impl CallbackWrapper {
        #[cfg(supports = "callbacks")]
        pub fn test_multi_arg_callback(f: impl Fn(i32) -> i32, x: i32) -> i32 {
            f(10 + x)
        }
        #[cfg(supports = "callbacks")]
        pub fn test_no_args(h: impl Fn()) -> i32 {
            h();
            -5
        }
        #[cfg(supports = "callbacks")]
        pub fn test_cb_with_struct(f: impl Fn(CallbackTestingStruct) -> i32) -> i32 {
            let arg = CallbackTestingStruct { x: 1, y: 5 };
            f(arg)
        }
        #[cfg(supports = "callbacks")]
        pub fn test_multiple_cb_args(f: impl Fn() -> i32, g: impl Fn(i32) -> i32) -> i32 {
            f() + g(5)
        }
    }
}
