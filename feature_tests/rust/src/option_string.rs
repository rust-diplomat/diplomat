/// The type behind `OptionString` pointers at the ABI level.
#[repr(C)]
pub(crate) struct OptionStringRaw {
    _private: [u8; 0],
}

pub struct OptionString(pub(crate) core::ptr::NonNull<OptionStringRaw>);

impl Drop for OptionString {
    fn drop(&mut self) {
        unsafe { OptionString_destroy(self.0.as_ptr()) }
    }
}

extern "C" {
    fn OptionString_destroy(this: *mut OptionStringRaw);
}
