/// The type behind `OptionOpaqueChar` pointers at the ABI level.
#[repr(C)]
pub(crate) struct OptionOpaqueCharRaw {
    _private: [u8; 0],
}

pub struct OptionOpaqueChar(pub(crate) core::ptr::NonNull<OptionOpaqueCharRaw>);

impl Drop for OptionOpaqueChar {
    fn drop(&mut self) {
        unsafe { OptionOpaqueChar_destroy(self.0.as_ptr()) }
    }
}

extern "C" {
    fn OptionOpaqueChar_destroy(this: *mut OptionOpaqueCharRaw);
}
