use core::ffi::c_void;
use alloc::boxed::Box;

// struct representing a callback from Rust into a foreign language
// TODO restrict the return type?
#[repr(C)]
pub struct DiplomatCallback<ReturnType> {
    // any data required to run the callback; e.g. a pointer to the
    // callback wrapper object in the foreign runtime + the runtime itself
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

// create a DiplomatCallback for C
// this is the same for all callbacks, because `run_callback` will be a noop,
// instead, we just call the wrapper.data directly as it'll be a function pointer
#[no_mangle]
pub unsafe extern "C" fn diplomat_callback_create_for_c(
    callback: *const c_void,
) -> *mut DiplomatCallback<()> {
    // define the callback runner
    unsafe extern "C" fn run_callback(_data: *const c_void) {
        // no-op for C because the function pointer itself is all that's needed
    }
    unsafe extern "C" fn destructor(_this: *const c_void) {
        // no-op for C b/c C manages the memory for its own function
    }
    let ret = DiplomatCallback::<()> {
        data: Box::into_raw(Box::new(callback)) as _,
        run_callback: core::mem::transmute::<unsafe extern "C" fn (*const c_void), unsafe extern "C" fn(*const c_void, ...)>(run_callback),
        destructor,
    };

    Box::into_raw(Box::new(ret))
}