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
        pub fn test_multi_arg_callback(f: impl Fn(i32) -> i32, x: i32) -> i32 {
            f(10 + x)
        }

        pub fn test_multiarg_void_callback(f: impl Fn(i32, &str)) {
            // note: make sure the string passed to f is a C-string (ending with \0)
            // if you're passing to C
            f(-10, "hello it's a string\0");
        }

        pub fn test_mod_array(g: impl Fn(&[u8])) {
            let bytes: Vec<u8> = vec![0x11, 0x22];
            println!("Pre callback: {:?}", bytes);
            g(bytes.as_slice());
            println!("Back in Rust post callback: {:?}", bytes);
        }

        pub fn test_no_args(h: impl Fn()) -> i32 {
            h();
            -5
        }

        pub fn test_cb_with_struct(f: impl Fn(TestingStruct) -> i32) -> i32 {
            let arg = TestingStruct {
                x: 1,
                y: 5,
            };
            f(arg)
        }

        pub fn test_multiple_cb_args(f: impl Fn() -> i32, g: impl Fn(i32) -> i32) -> i32 {
            f() + g(5)
        }
    }

}

// // ----------------------------------------------------------------------------------------------------

// // // ------------------------------------------------- this next section is specific to the callback (for JVM)

// #[no_mangle]
// pub unsafe extern "system" fn diplomat_callback_create_for_jvm__callback(
//     callback: DiplomatCallbackI32ToI32,
// ) -> *mut DiplomatCallback<i32> {
//     // define the callback runner
//     unsafe extern "C" fn run_callback(data: *const c_void, arg0: i32) -> i32 {
//         unsafe {
//             let cb = data.cast::<DiplomatCallbackI32ToI32>();
//             // unwrap and call the "invoke" function on the JNA Callback
//             (*cb).unwrap()(arg0)
//         }
//     }
//     unsafe extern "C" fn destructor(_this: *const c_void) {
//         // no-op for JNA because the JVM manages the memory for the JNA Callbacks
//     }
//     let ret = DiplomatCallback::<i32> {
//         data: Box::into_raw(Box::new(callback)) as _,
//         run_callback: std::mem::transmute::<unsafe extern "C" fn (*const c_void, i32) -> i32, unsafe extern "C" fn(*const c_void, ...) -> i32>(run_callback),
//         destructor,
//     };

//     Box::into_raw(Box::new(ret))
// }

// #[no_mangle]
// pub extern "C" fn DiplomatCallbackI32ToI32_test_rust_fn_test_call(cb_wrap: &DiplomatCallback<i32>) -> i32 {
//     ffi::Wrapper::test_rust_fn(move |arg0| unsafe {
//         (cb_wrap.run_callback)(cb_wrap.data, arg0)
//     })
// }

// pub type DiplomatCallbackI32ToI32 = Option<unsafe extern "C" fn(i32) -> i32>;