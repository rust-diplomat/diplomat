#[repr(C)]
pub struct RefListParameter {
    // Lives in the library's memory, so it is only ever reached through `&`, `&mut`
    // or `DiplomatBox`. Not `Send`, `Sync` or `Unpin`: the library does not say
    // whether that would be safe.
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

// Safety: `RefListParameter_destroy` frees a `Box<RefListParameter>` returned by the library.
unsafe impl crate::DiplomatOpaque for RefListParameter {
    unsafe fn destroy(ptr: *mut Self) {
        RefListParameter_destroy(ptr)
    }
}

extern "C" {
    fn RefListParameter_destroy(this: *mut RefListParameter);
}