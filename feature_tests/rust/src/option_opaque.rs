#[repr(C)]
pub struct OptionOpaque {
    // Lives in the library's memory, so it is only ever reached through `&`, `&mut`
    // or `DiplomatBox`. Not `Send`, `Sync` or `Unpin`: the library does not say
    // whether that would be safe.
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl OptionOpaque {
    pub fn assert_integer(&self, i: i32) {
        unsafe { OptionOpaque_assert_integer(self, i) }
    }
}

extern "C" {
    fn OptionOpaque_assert_integer(this: *const OptionOpaque, i: i32);
}

// Safety: `OptionOpaque_destroy` frees a `Box<OptionOpaque>` returned by the library.
unsafe impl crate::DiplomatOpaque for OptionOpaque {
    unsafe fn destroy(ptr: *mut Self) {
        OptionOpaque_destroy(ptr)
    }
}

extern "C" {
    fn OptionOpaque_destroy(this: *mut OptionOpaque);
}