#[repr(C)]
pub struct OptionString {
    // Lives in the library's memory, so it is only ever reached through `&`, `&mut`
    // or `DiplomatBox`. Not `Send`, `Sync` or `Unpin`: the library does not say
    // whether that would be safe.
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

// Safety: `OptionString_destroy` frees a `Box<OptionString>` returned by the library.
unsafe impl crate::DiplomatOpaque for OptionString {
    unsafe fn destroy(ptr: *mut Self) {
        OptionString_destroy(ptr)
    }
}

extern "C" {
    fn OptionString_destroy(this: *mut OptionString);
}