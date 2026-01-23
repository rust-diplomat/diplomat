#[diplomat::bridge]
mod ffi {
    use crate::slices::ffi::MyString;

    #[diplomat::cfg(supports = "callbacks")]
    pub struct CallbackWrapper {
        cant_be_empty: bool,
    }
    #[diplomat::cfg(supports = "callbacks")]
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

        #[diplomat::attr(kotlin, disable)]
        pub fn test_opaque_cb_arg<'a>(cb: impl Fn(&mut MyString), a: &'a mut MyString) {
            cb(a);
        }

        pub fn test_slice_cb_arg(arg: &[u8], f: impl Fn(&[u8])) {
            f(arg);
        }

        #[diplomat::attr(kotlin, disable)]
        pub fn test_result_output(t: impl Fn() -> Result<(), ()>) {
            assert_eq!(t(), Ok(()));
        }

        #[diplomat::attr(kotlin, disable)]
        pub fn test_result_usize_output(t: impl Fn() -> Result<usize, ()>) {
            assert_eq!(t(), Ok(0));
        }

        #[diplomat::attr(kotlin, disable)]
        pub fn test_option_output(t: impl Fn() -> Option<()>) {
            assert_eq!(t(), None);
        }

        #[diplomat::attr(kotlin, disable)]
        pub fn test_diplomat_option_output(t: impl Fn() -> DiplomatOption<u32>) {
            let out = t();
            assert_eq!(out.into_option(), Some(0));
        }

        #[diplomat::attr(kotlin, disable)]
        pub fn test_option_opaque<'a>(
            t: impl Fn() -> Option<&'a crate::structs::ffi::Opaque>,
            w: &mut DiplomatWrite,
        ) {
            let op = t();

            assert!(op.is_some());
            let a = op.unwrap();
            a.get_debug_str(w);
        }

        #[diplomat::attr(kotlin, disable)]
        pub fn test_diplomat_result(t: impl Fn() -> DiplomatResult<usize, usize>) {
            let out = t();
            assert_eq!(out.as_ref().err().cloned(), Some(10));
        }

        #[diplomat::attr(kotlin, disable)]
        pub fn test_result_opaque<'a>(
            t: impl Fn() -> Result<&'a crate::structs::ffi::Opaque, ()>,
            w: &mut DiplomatWrite,
        ) {
            let op = t();

            assert!(op.is_ok());
            let a = op.unwrap();
            a.get_debug_str(w);
        }

        #[diplomat::attr(kotlin, disable)]
        pub fn test_inner_conversion(
            t: impl Fn() -> Result<crate::structs::ffi::MyStructContainingAnOption, usize>,
        ) {
            let out = t();
            let out = out.expect("Could not get struct out.");
            assert!(out.a.is_ok && out.b.is_ok);
            assert_eq!(out.a.into_option().unwrap().into_a(), 42);
        }

        #[diplomat::attr(kotlin, disable)]
        pub fn test_str_conversion<'a>(t: impl Fn() -> Result<DiplomatStrSlice<'a>, ()>) {
            let str = t().expect("Could not get string.");
            let str = String::from_utf8(str.to_vec()).unwrap();
            assert_eq!(str, "Slice conversion test string");
        }

        #[diplomat::attr(kotlin, disable)]
        pub fn test_slice_conversion<'a>(t: impl Fn() -> Result<&'a [f64], ()>) {
            let sl = t().expect("Could not get f64 slice.");
            assert_eq!(sl[1], 2.0);
        }

        #[diplomat::attr(kotlin, disable)]
        pub fn test_struct_slice_conversion<'a>(
            t: impl Fn() -> Result<&'a [crate::structs::ffi::PrimitiveStruct], ()>,
        ) {
            let sl = t().expect("Could not get &[PrimitiveStruct].");
            assert_eq!(sl[1].b, 'f' as u32);
        }

        #[diplomat::attr(kotlin, disable)]
        pub fn test_opaque_result_error<'a>(
            t: impl Fn() -> Result<(), &'a crate::structs::ffi::Opaque>,
            w: &mut DiplomatWrite,
        ) {
            let op = t();

            assert!(op.is_err());
            let a = op.unwrap_err();
            a.get_debug_str(w);
        }
    }

    #[diplomat::cfg(supports = "callbacks")]
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

    #[diplomat::cfg(supports = "callbacks")]
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

    fn hidden_internal() {}

    #[diplomat::attr(not(supports = "callbacks"), disable)]
    #[diplomat::attr(kotlin, disable)]
    pub fn free_callback_holder(f: impl Fn() -> Result<(), ()>) {
        assert_eq!(f(), Ok(()))
    }
}
