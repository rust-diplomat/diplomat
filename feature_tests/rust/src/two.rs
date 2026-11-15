/// The type behind `Two` pointers at the ABI level.
#[repr(C)]
pub(crate) struct TwoRaw {
    _private: [u8; 0],
}

pub struct Two(pub(crate) core::ptr::NonNull<TwoRaw>);

impl Drop for Two {
    fn drop(&mut self) {
        unsafe { Two_destroy(self.0.as_ptr()) }
    }
}

extern "C" {
    fn Two_destroy(this: *mut TwoRaw);
}
