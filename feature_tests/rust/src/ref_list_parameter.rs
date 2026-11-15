/// The type behind `RefListParameter` pointers at the ABI level.
#[repr(C)]
pub(crate) struct RefListParameterRaw {
    _private: [u8; 0],
}

pub struct RefListParameter(pub(crate) core::ptr::NonNull<RefListParameterRaw>);

impl Drop for RefListParameter {
    fn drop(&mut self) {
        unsafe { RefListParameter_destroy(self.0.as_ptr()) }
    }
}

extern "C" {
    fn RefListParameter_destroy(this: *mut RefListParameterRaw);
}
