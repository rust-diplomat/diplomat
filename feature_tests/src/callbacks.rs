#[diplomat::bridge]
pub mod ffi {
    #[derive(Debug)]
    pub enum FFIError {
        #[diplomat::attr(auto, ffi_error)]
        FFI,
        User,
    }

    use crate::{slices::ffi::MyString, structs::ffi::MyStruct};

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
        #[diplomat::cfg(not(supports = "callback_returns_must_be_fallible"))]
        pub fn test_multi_arg_callback(f: impl Fn(i32) -> i32, x: i32) -> i32 {
            f(10 + x)
        }
        #[diplomat::cfg(supports = "callback_returns_must_be_fallible")]
        pub fn test_multi_arg_callback_fallible(
            f: impl Fn(i32) -> Result<i32, FFIError>,
            x: i32,
        ) -> i32 {
            f(10 + x).unwrap()
        }

        #[diplomat::cfg(not(supports = "callback_returns_must_be_fallible"))]
        pub fn test_no_args(h: impl Fn()) -> i32 {
            h();
            -5
        }
        #[diplomat::cfg(supports = "callback_returns_must_be_fallible")]
        pub fn test_no_args_fallible(h: impl Fn() -> Result<(), FFIError>) -> i32 {
            h().unwrap();
            -5
        }

        #[diplomat::cfg(not(supports = "callback_returns_must_be_fallible"))]
        pub fn test_cb_with_struct(f: impl Fn(CallbackTestingStruct) -> i32) -> i32 {
            let arg = CallbackTestingStruct { x: 1, y: 5 };
            f(arg)
        }
        #[diplomat::cfg(supports = "callback_returns_must_be_fallible")]
        pub fn test_cb_with_struct_fallible(
            f: impl Fn(CallbackTestingStruct) -> Result<i32, FFIError>,
        ) -> i32 {
            let arg = CallbackTestingStruct { x: 1, y: 5 };
            f(arg).unwrap()
        }

        #[diplomat::cfg(not(supports = "callback_returns_must_be_fallible"))]
        pub fn test_multiple_cb_args(f: impl Fn() -> i32, g: impl Fn(i32) -> i32) -> i32 {
            f() + g(5)
        }
        #[diplomat::cfg(supports = "callback_returns_must_be_fallible")]
        pub fn test_multiple_cb_args_fallible(
            f: impl Fn() -> Result<i32, FFIError>,
            g: impl Fn(i32) -> Result<i32, FFIError>,
        ) -> i32 {
            f().unwrap() + g(5).unwrap()
        }

        #[diplomat::cfg(not(supports = "callback_returns_must_be_fallible"))]
        #[diplomat::attr(kotlin, disable)]
        pub fn test_str_cb_arg(f: impl Fn(&str) -> i32) -> i32 {
            f("bananna")
        }
        #[diplomat::cfg(supports = "callback_returns_must_be_fallible")]
        pub fn test_str_cb_arg_fallible(f: impl Fn(&str) -> Result<i32, FFIError>) -> i32 {
            f("bananna").unwrap()
        }

        #[diplomat::cfg(not(supports = "callback_returns_must_be_fallible"))]
        #[diplomat::attr(kotlin, disable)]
        pub fn test_opaque_cb_arg<'a>(cb: impl Fn(&mut MyString), a: &'a mut MyString) {
            cb(a);
        }
        #[diplomat::cfg(supports = "callback_returns_must_be_fallible")]
        pub fn test_opaque_cb_arg_fallible<'a>(
            cb: impl Fn(&mut MyString) -> Result<(), FFIError>,
            a: &'a mut MyString,
        ) {
            cb(a).unwrap();
        }

        #[diplomat::cfg(not(supports = "callback_returns_must_be_fallible"))]
        pub fn test_slice_cb_arg(arg: &[u8], f: impl Fn(&[u8])) {
            f(arg);
        }

        #[diplomat::cfg(not(supports = "callback_returns_must_be_fallible"))]
        #[diplomat::attr(kotlin, disable)]
        pub fn test_result_output(t: impl Fn() -> Result<(), ()>) {
            assert_eq!(t(), Ok(()));
        }

        #[diplomat::cfg(not(supports = "callback_returns_must_be_fallible"))]
        #[diplomat::attr(kotlin, disable)]
        pub fn test_result_usize_output(t: impl Fn() -> Result<usize, ()>) {
            assert_eq!(t(), Ok(0));
        }

        #[diplomat::cfg(not(supports = "callback_returns_must_be_fallible"))]
        #[diplomat::attr(kotlin, disable)]
        pub fn test_option_output(t: impl Fn() -> Option<()>) {
            assert_eq!(t(), None);
        }

        #[diplomat::cfg(not(supports = "callback_returns_must_be_fallible"))]
        #[diplomat::attr(kotlin, disable)]
        pub fn test_diplomat_option_output(t: impl Fn() -> DiplomatOption<u32>) {
            let out = t();
            assert_eq!(out.into_option(), Some(0));
        }

        #[diplomat::cfg(not(supports = "callback_returns_must_be_fallible"))]
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

        #[diplomat::cfg(not(supports = "callback_returns_must_be_fallible"))]
        #[diplomat::attr(kotlin, disable)]
        pub fn test_owned_opaque(t: impl Fn(Box<crate::structs::ffi::Opaque>)) {
            t(crate::structs::ffi::Opaque::from_str("Some string"))
        }

        #[diplomat::cfg(not(supports = "callback_returns_must_be_fallible"))]
        #[diplomat::attr(kotlin, disable)]
        pub fn test_diplomat_result(t: impl Fn() -> DiplomatResult<usize, usize>) {
            let out = t();
            assert_eq!(out.as_ref().err().cloned(), Some(10));
        }

        #[diplomat::attr(kotlin, disable)]
        pub fn test_result_opaque<'a>(
            t: impl Fn() -> Result<&'a crate::structs::ffi::Opaque, FFIError>,
            w: &mut DiplomatWrite,
        ) {
            let op = t();

            assert!(op.is_ok());
            let a = op.unwrap();
            a.get_debug_str(w);
        }

        #[diplomat::attr(kotlin, disable)]
        #[diplomat::cfg(not(supports = "callback_returns_must_be_fallible"))]
        pub fn test_inner_conversion(
            t: impl Fn() -> Result<crate::structs::ffi::MyStructContainingAnOption, usize>,
        ) {
            let out = t();
            let out = out.expect("Could not get struct out.");
            assert!(out.a.is_ok && out.b.is_ok);
            assert_eq!(out.a.into_option().unwrap().into_a(), 42);
        }

        #[diplomat::attr(kotlin, disable)]
        pub fn test_str_conversion<'a>(t: impl Fn() -> Result<DiplomatStrSlice<'a>, FFIError>) {
            let str = t().expect("Could not get string.");
            let str = String::from_utf8(str.to_vec()).unwrap();
            assert_eq!(str, "Slice conversion test string");
        }

        #[diplomat::attr(kotlin, disable)]
        pub fn test_slice_conversion<'a>(t: impl Fn() -> Result<&'a [f64], FFIError>) {
            let sl = t().expect("Could not get f64 slice.");
            assert_eq!(sl[1], 2.0);
        }

        #[diplomat::attr(kotlin, disable)]
        pub fn test_result_option_struct_conversion(
            t: impl Fn() -> Result<DiplomatOption<MyStruct>, FFIError>,
        ) {
            let st = t();
            assert!(st.is_ok());
            let st_opt = st.unwrap().into_option();
            assert!(st_opt.is_some());
            let st = st_opt.unwrap();
            assert_eq!(st.into_a(), 5);
        }

        #[diplomat::attr(kotlin, disable)]
        pub fn test_struct_slice_conversion<'a>(
            t: impl Fn() -> Result<&'a [crate::structs::ffi::PrimitiveStruct], FFIError>,
        ) {
            let sl = t().expect("Could not get &[PrimitiveStruct].");
            assert_eq!(sl[1].b, 'f' as u32);
        }

        #[diplomat::attr(kotlin, disable)]
        #[diplomat::cfg(not(supports = "callback_returns_must_be_fallible"))]
        pub fn test_opaque_result_error<'a>(
            t: impl Fn() -> Result<(), &'a crate::structs::ffi::Opaque>,
            w: &mut DiplomatWrite,
        ) {
            let op = t();

            assert!(op.is_err());
            let a = op.unwrap_err();
            a.get_debug_str(w);
        }

        #[diplomat::cfg(supports = "callback_returns_must_be_fallible")]
        pub fn test_ffi_error(t: impl Fn() -> Result<(), FFIError>) -> Result<(), FFIError> {
            t()
        }

        #[diplomat::cfg(supports = "callback_returns_must_be_fallible")]
        pub fn test_ffi_error_as_ok(t: impl Fn() -> Result<(), FFIError>) -> FFIError {
            t().unwrap_err()
        }
    }

    #[diplomat::cfg(supports = "callbacks")]
    #[diplomat::opaque]
    pub struct CallbackHolder {
        held: Box<dyn Fn(i32) -> i32>,
    }

    impl CallbackHolder {
        #[diplomat::attr(auto, constructor)]
        #[diplomat::cfg(not(supports = "callback_returns_must_be_fallible"))]
        pub fn new(func: impl Fn(i32) -> i32 + 'static) -> Box<Self> {
            Box::new(Self {
                held: Box::new(func),
            })
        }

        #[diplomat::attr(auto, constructor)]
        #[diplomat::cfg(supports = "callback_returns_must_be_fallible")]
        pub fn new_fallible(func: impl Fn(i32) -> Result<i32, FFIError> + 'static) -> Box<Self> {
            Box::new(Self {
                held: Box::new(move |v| func(v).unwrap()),
            })
        }

        pub fn call(&self, a: i32) -> i32 {
            (self.held)(a)
        }
    }

    #[diplomat::cfg(supports = "callbacks")]
    #[diplomat::opaque_mut]
    pub struct MutableCallbackHolder {
        held: Box<dyn FnMut(i32) -> i32>,
    }

    impl MutableCallbackHolder {
        #[diplomat::attr(auto, constructor)]
        #[diplomat::cfg(not(supports = "callback_returns_must_be_fallible"))]
        pub fn new(func: impl FnMut(i32) -> i32 + 'static) -> Box<Self> {
            Box::new(Self {
                held: Box::new(func),
            })
        }

        #[diplomat::attr(auto, constructor)]
        #[diplomat::cfg(supports = "callback_returns_must_be_fallible")]
        pub fn new_fallible(
            mut func: impl FnMut(i32) -> Result<i32, FFIError> + 'static,
        ) -> Box<Self> {
            Box::new(Self {
                held: Box::new(move |v| func(v).unwrap()),
            })
        }

        pub fn call(&mut self, a: i32) -> i32 {
            (self.held)(a)
        }
    }

    // FIXME: https://github.com/rust-diplomat/diplomat/issues/1204
    #[diplomat::cfg(all(supports = "callbacks", not(kotlin)))]
    #[diplomat::opaque_mut]
    pub struct OpaqueCallbacks;

    impl OpaqueCallbacks {
        #[diplomat::cfg(not(supports = "callback_returns_must_be_fallible"))]
        pub fn ret_op<'a>(f: impl Fn(&MyString) -> &'a MyString, st: &MyString) -> &'a MyString {
            f(st)
        }
        #[diplomat::cfg(supports = "callback_returns_must_be_fallible")]
        pub fn ret_op_fallible<'a>(
            f: impl Fn(&MyString) -> Result<&'a MyString, FFIError>,
            st: &MyString,
        ) -> &'a MyString {
            f(st).unwrap()
        }

        #[diplomat::cfg(not(supports = "callback_returns_must_be_fallible"))]
        #[diplomat::attr(auto, constructor)]
        pub fn ctor<'a>(f: impl Fn(&MyString) -> &'a MyString, st: &MyString) -> Box<Self> {
            let _ = f(st);
            Box::new(Self {})
        }
        #[diplomat::cfg(supports = "callback_returns_must_be_fallible")]
        #[diplomat::attr(auto, constructor)]
        pub fn ctor_fallible<'a>(
            f: impl Fn(&MyString) -> Result<&'a MyString, FFIError>,
            st: &MyString,
        ) -> Box<Self> {
            let _ = f(st).unwrap();
            Box::new(Self {})
        }

        #[diplomat::cfg(not(supports = "callback_returns_must_be_fallible"))]
        pub fn opaque_cb_self<'a>(
            &self,
            cb: impl Fn(&MyString) -> &'a MyString,
            st: &MyString,
        ) -> &'a MyString {
            cb(st)
        }
        #[diplomat::cfg(supports = "callback_returns_must_be_fallible")]
        pub fn opaque_cb_self_fallible<'a>(
            &self,
            cb: impl Fn(&MyString) -> Result<&'a MyString, FFIError>,
            st: &MyString,
        ) -> &'a MyString {
            cb(st).unwrap()
        }

        #[diplomat::cfg(not(supports = "callback_returns_must_be_fallible"))]
        pub fn opaque_cb_mut_self<'a>(
            &mut self,
            cb: impl Fn(&MyString) -> &'a MyString,
            st: &MyString,
        ) -> &'a MyString {
            cb(st)
        }
        #[diplomat::cfg(supports = "callback_returns_must_be_fallible")]
        pub fn opaque_cb_mut_self_fallible<'a>(
            &mut self,
            cb: impl Fn(&MyString) -> Result<&'a MyString, FFIError>,
            st: &MyString,
        ) -> &'a MyString {
            cb(st).unwrap()
        }
    }

    fn hidden_internal() {}

    #[diplomat::attr(
        any(
            not(supports = "callbacks"),
            supports = "callback_returns_must_be_fallible"
        ),
        disable
    )]
    #[diplomat::attr(kotlin, disable)]
    pub fn free_callback_holder(f: impl Fn() -> Result<(), ()>) {
        assert_eq!(f(), Ok(()))
    }
}
