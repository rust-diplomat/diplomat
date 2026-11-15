/// The type behind `RenamedAttrOpaque2` pointers at the ABI level.
#[repr(C)]
pub(crate) struct RenamedAttrOpaque2Raw {
    _private: [u8; 0],
}

pub struct RenamedAttrOpaque2(pub(crate) core::ptr::NonNull<RenamedAttrOpaque2Raw>);

impl Drop for RenamedAttrOpaque2 {
    fn drop(&mut self) {
        unsafe { namespace_AttrOpaque2_destroy(self.0.as_ptr()) }
    }
}

extern "C" {
    fn namespace_AttrOpaque2_destroy(this: *mut RenamedAttrOpaque2Raw);
}
