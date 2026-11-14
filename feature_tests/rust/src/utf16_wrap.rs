#[repr(C)]
pub struct Utf16Wrap {
    // Lives in the library's memory, so it is only ever reached through `&`, `&mut`
    // or `DiplomatBox`. Not `Send`, `Sync` or `Unpin`: the library does not say
    // whether that would be safe.
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl Utf16Wrap {
    pub fn from_utf16(input: &[u16]) -> crate::DiplomatBox<crate::Utf16Wrap> {
        // Safety: `Box` returns are never null, and the caller owns them.
        unsafe { crate::DiplomatBox::from_raw(Utf16Wrap_from_utf16(crate::DiplomatSlice::new(input))) }
    }
}

extern "C" {
    fn Utf16Wrap_from_utf16(input: crate::DiplomatSlice<u16>) -> *mut crate::Utf16Wrap;
}

// Safety: `Utf16Wrap_destroy` frees a `Box<Utf16Wrap>` returned by the library.
unsafe impl crate::DiplomatOpaque for Utf16Wrap {
    unsafe fn destroy(ptr: *mut Self) {
        Utf16Wrap_destroy(ptr)
    }
}

extern "C" {
    fn Utf16Wrap_destroy(this: *mut Utf16Wrap);
}