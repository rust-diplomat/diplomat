/// The type behind `One` pointers at the ABI level.
#[repr(C)]
pub(crate) struct OneRaw {
    _private: [u8; 0],
}

pub struct One(pub(crate) core::ptr::NonNull<OneRaw>);

impl Drop for One {
    fn drop(&mut self) {
        unsafe { One_destroy(self.0.as_ptr()) }
    }
}

extern "C" {
    fn One_destroy(this: *mut OneRaw);
}
