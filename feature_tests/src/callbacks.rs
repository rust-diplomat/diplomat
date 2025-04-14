#[diplomat::bridge]
mod ffi {
    use crate::slices::ffi::MyString;

    #[diplomat::attr(not(supports = "callbacks"), disable)]
    pub struct CallbackWrapper {
        cant_be_empty: bool,
    }
    #[diplomat::attr(not(supports = "callbacks"), disable)]
    pub struct CallbackTestingStruct {
        x: i32,
        y: i32,
    }

    impl CallbackWrapper {
        pub fn test_multi_arg_callback(f: impl Fn(i32) -> i32, x: i32) -> i32 {
            f(10 + x)
        }
        pub fn test_no_args(h: impl Fn()) -> i32 {
            h();
            -5
        }
        pub fn test_cb_with_struct(f: impl Fn(CallbackTestingStruct) -> i32) -> i32 {
            let arg = CallbackTestingStruct { x: 1, y: 5 };
            f(arg)
        }
        pub fn test_multiple_cb_args(f: impl Fn() -> i32, g: impl Fn(i32) -> i32) -> i32 {
            f() + g(5)
        }
        #[diplomat::attr(kotlin, disable)]
        pub fn test_str_cb_arg(f: impl Fn(&str) -> i32) -> i32 {
            f("bananna")
        }

        pub fn test_opaque_cb_arg(cb: impl Fn(&mut MyString), a: &mut MyString) {
            cb(a);
        }
    }

    #[diplomat::attr(not(supports = "callbacks"), disable)]
    #[diplomat::opaque]
    pub struct CallbackHolder {
        held: Box<dyn Fn(i32) -> i32>,
    }

    impl CallbackHolder {
        #[diplomat::attr(auto, constructor)]
        pub fn new(func: impl Fn(i32) -> i32 + 'static) -> Box<Self> {
            Box::new(Self {
                held: Box::new(func),
            })
        }

        pub fn call(&self, a: i32) -> i32 {
            (self.held)(a)
        }
    }

    #[diplomat::attr(not(supports = "callbacks"), disable)]
    #[diplomat::opaque]
    pub struct MutableCallbackHolder {
        held: Box<dyn FnMut(i32) -> i32>,
    }

    impl MutableCallbackHolder {
        #[diplomat::attr(auto, constructor)]
        pub fn new(func: impl FnMut(i32) -> i32 + 'static) -> Box<Self> {
            Box::new(Self {
                held: Box::new(func),
            })
        }

        pub fn call(&mut self, a: i32) -> i32 {
            (self.held)(a)
        }
    }
}
