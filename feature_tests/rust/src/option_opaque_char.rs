#[repr(C)]
pub struct OptionOpaqueChar {
    // Lives in the library's memory, so it is only ever reached through `&`, `&mut`
    // or `DiplomatBox`. Not `Send`, `Sync` or `Unpin`: the library does not say
    // whether that would be safe.
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

// Safety: `OptionOpaqueChar_destroy` frees a `Box<OptionOpaqueChar>` returned by the library.
unsafe impl crate::DiplomatOpaque for OptionOpaqueChar {
    unsafe fn destroy(ptr: *mut Self) {
        OptionOpaqueChar_destroy(ptr)
    }
}

extern "C" {
    fn OptionOpaqueChar_destroy(this: *mut OptionOpaqueChar);
}