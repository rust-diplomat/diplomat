/// The type behind `RenamedDeprecatedOpaque` pointers at the ABI level.
#[repr(C)]
pub(crate) struct RenamedDeprecatedOpaqueRaw {
    _private: [u8; 0],
}

pub struct RenamedDeprecatedOpaque(pub(crate) core::ptr::NonNull<RenamedDeprecatedOpaqueRaw>);

impl Drop for RenamedDeprecatedOpaque {
    fn drop(&mut self) {
        unsafe { namespace_DeprecatedOpaque_destroy(self.0.as_ptr()) }
    }
}

extern "C" {
    fn namespace_DeprecatedOpaque_destroy(this: *mut RenamedDeprecatedOpaqueRaw);
}
