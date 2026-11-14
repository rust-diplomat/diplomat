/// The type behind `Utf16Wrap` pointers at the ABI level.
#[repr(C)]
pub(crate) struct Utf16WrapRaw {
    _private: [u8; 0],
}

pub struct Utf16Wrap(pub(crate) core::ptr::NonNull<Utf16WrapRaw>);

impl Utf16Wrap {
    pub fn from_utf16(input: &[u16]) -> crate::Utf16Wrap {
        // Safety: `Box` returns are never null, and the wrapper takes ownership.
        unsafe {
            crate::Utf16Wrap(core::ptr::NonNull::new_unchecked(
                Utf16Wrap_from_utf16(crate::DiplomatSlice::new(input)),
            ))
        }
    }
}

extern "C" {
    fn Utf16Wrap_from_utf16(input: crate::DiplomatSlice<u16>) -> *mut crate::utf16_wrap::Utf16WrapRaw;
}

impl Drop for Utf16Wrap {
    fn drop(&mut self) {
        unsafe { Utf16Wrap_destroy(self.0.as_ptr()) }
    }
}

extern "C" {
    fn Utf16Wrap_destroy(this: *mut Utf16WrapRaw);
}
