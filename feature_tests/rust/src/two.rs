#[repr(C)]
pub struct Two {
    // Lives in the library's memory, so it is only ever reached through `&`, `&mut`
    // or `DiplomatBox`. Not `Send`, `Sync` or `Unpin`: the library does not say
    // whether that would be safe.
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

// Safety: `Two_destroy` frees a `Box<Two>` returned by the library.
unsafe impl crate::DiplomatOpaque for Two {
    unsafe fn destroy(ptr: *mut Self) {
        Two_destroy(ptr)
    }
}

extern "C" {
    fn Two_destroy(this: *mut Two);
}