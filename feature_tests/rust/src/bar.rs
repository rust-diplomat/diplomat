#[repr(C)]
pub struct Bar {
    // Lives in the library's memory, so it is only ever reached through `&`, `&mut`
    // or `DiplomatBox`. Not `Send`, `Sync` or `Unpin`: the library does not say
    // whether that would be safe.
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

// Safety: `Bar_destroy` frees a `Box<Bar>` returned by the library.
unsafe impl crate::DiplomatOpaque for Bar {
    unsafe fn destroy(ptr: *mut Self) {
        Bar_destroy(ptr)
    }
}

extern "C" {
    fn Bar_destroy(this: *mut Bar);
}