/// The type behind `RenamedNested` pointers at the ABI level.
#[repr(C)]
pub(crate) struct RenamedNestedRaw {
    _private: [u8; 0],
}

pub struct RenamedNested(pub(crate) core::ptr::NonNull<RenamedNestedRaw>);

impl Drop for RenamedNested {
    fn drop(&mut self) {
        unsafe { namespace_Nested_destroy(self.0.as_ptr()) }
    }
}

extern "C" {
    fn namespace_Nested_destroy(this: *mut RenamedNestedRaw);
}
