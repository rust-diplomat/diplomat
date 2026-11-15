/// The type behind `RenamedTestOpaque` pointers at the ABI level.
#[repr(C)]
pub(crate) struct RenamedTestOpaqueRaw {
    _private: [u8; 0],
}

pub struct RenamedTestOpaque(pub(crate) core::ptr::NonNull<RenamedTestOpaqueRaw>);

impl Drop for RenamedTestOpaque {
    fn drop(&mut self) {
        unsafe { namespace_TestOpaque_destroy(self.0.as_ptr()) }
    }
}

extern "C" {
    fn namespace_TestOpaque_destroy(this: *mut RenamedTestOpaqueRaw);
}
