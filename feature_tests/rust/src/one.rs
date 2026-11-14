#[repr(C)]
pub struct One {
    // Lives in the library's memory, so it is only ever reached through `&`, `&mut`
    // or `DiplomatBox`. Not `Send`, `Sync` or `Unpin`: the library does not say
    // whether that would be safe.
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

// Safety: `One_destroy` frees a `Box<One>` returned by the library.
unsafe impl crate::DiplomatOpaque for One {
    unsafe fn destroy(ptr: *mut Self) {
        One_destroy(ptr)
    }
}

extern "C" {
    fn One_destroy(this: *mut One);
}