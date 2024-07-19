// // #[diplomat::bridge]
// // mod ffi {

fn test_rust_fn(f: impl Fn(i32) -> i32) -> i32 {
    f(10)
}

// // }

// // ----------------------------------------------------------------------------------------------------

use core::ffi::c_void;

// struct representing a callback from Rust into a foreign language
// TODO restrict the return type?
#[repr(C)]
pub struct DiplomatCallback<ReturnType> {
    // any data required to run the callback; e.g. a pointer to the callback wrapper object in the foreign runtime + the runtime itself
    data: *const c_void,
    // function to actually run the callback
    run_callback: unsafe extern "C" fn(*const c_void, ...) -> ReturnType, 
    // function to destroy this callback struct
    destructor: unsafe extern "C" fn(*const c_void),
}

impl<ReturnType> Drop for DiplomatCallback<ReturnType> {
    fn drop(&mut self) {
        unsafe { (self.destructor)(self.data); }
    }
}

// // ------------------------------------------------- this next section is specific to the callback

#[no_mangle]
pub unsafe extern "system" fn diplomat_callback_create_for_jvm__callback(
    callback: DiplomatCallbackI32ToI32,
) -> *mut DiplomatCallback<i32> {
    // define the callback runner
    unsafe extern "C" fn run_callback(data: *const c_void, arg0: i32) -> i32 {
        unsafe {
            let cb = data.cast::<DiplomatCallbackI32ToI32>();
            (*cb).unwrap()(arg0)
        }
    }
    unsafe extern "C" fn destructor(_this: *const c_void) {
        // no-op for JNA because the JVM manages the memory for the JNA Callbacks
    }
    let ret = DiplomatCallback::<i32> {
        data: Box::into_raw(Box::new(callback)) as _,
        run_callback: std::mem::transmute::<unsafe extern "C" fn (*const c_void, i32) -> i32, unsafe extern "C" fn(*const c_void, ...) -> i32>(run_callback),
        destructor,
    };

    Box::into_raw(Box::new(ret))
}

#[no_mangle]
pub extern "C" fn DiplomatCallbackI32ToI32_test_rust_fn_test_call(cb_wrap: &DiplomatCallback<i32>) -> i32 {
    test_rust_fn(move |arg0| unsafe {
        (cb_wrap.run_callback)(cb_wrap.data, arg0)
    })
}

pub type DiplomatCallbackI32ToI32 = Option<unsafe extern "C" fn(i32) -> i32>;

struct OpaqueFunctionPointer(u64);