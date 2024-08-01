use core::ffi::c_void;

// struct representing a callback from Rust into a foreign language
// TODO restrict the return type?
#[repr(C)]
pub struct DiplomatCallback<ReturnType> {
    // any data required to run the callback; e.g. a pointer to the
    // callback wrapper object in the foreign runtime + the runtime itself
    pub data: *const c_void,
    // function to actually run the callback
    pub run_callback: unsafe extern "C" fn(*const c_void, ...) -> ReturnType,
    // function to destroy this callback struct
    pub destructor: Option<unsafe extern "C" fn(*const c_void)>,
}

impl<ReturnType> Drop for DiplomatCallback<ReturnType> {
    fn drop(&mut self) {
        if let Some(destructor) = self.destructor {
            unsafe {
                (destructor)(self.data);
            }
        }
    }
}
