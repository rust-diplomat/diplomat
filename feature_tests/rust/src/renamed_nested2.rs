/// The type behind `RenamedNested2` pointers at the ABI level.
#[repr(C)]
pub(crate) struct RenamedNested2Raw {
    _private: [u8; 0],
}

pub struct RenamedNested2(pub(crate) core::ptr::NonNull<RenamedNested2Raw>);

impl Drop for RenamedNested2 {
    fn drop(&mut self) {
        unsafe { namespace_Nested2_destroy(self.0.as_ptr()) }
    }
}

extern "C" {
    fn namespace_Nested2_destroy(this: *mut RenamedNested2Raw);
}
