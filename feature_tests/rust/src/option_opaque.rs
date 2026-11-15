/// The type behind `OptionOpaque` pointers at the ABI level.
#[repr(C)]
pub(crate) struct OptionOpaqueRaw {
    _private: [u8; 0],
}

pub struct OptionOpaque(pub(crate) core::ptr::NonNull<OptionOpaqueRaw>);

impl OptionOpaque {
    pub fn assert_integer(&self, i: i32) {
        unsafe { OptionOpaque_assert_integer(self.0.as_ptr(), i) }
    }
}

extern "C" {
    fn OptionOpaque_assert_integer(this: *const OptionOpaqueRaw, i: i32);
}

impl Drop for OptionOpaque {
    fn drop(&mut self) {
        unsafe { OptionOpaque_destroy(self.0.as_ptr()) }
    }
}

extern "C" {
    fn OptionOpaque_destroy(this: *mut OptionOpaqueRaw);
}
