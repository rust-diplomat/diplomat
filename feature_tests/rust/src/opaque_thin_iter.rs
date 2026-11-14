#[repr(C)]
pub struct OpaqueThinIter {
    // Lives in the library's memory, so it is only ever reached through `&`, `&mut`
    // or `DiplomatBox`. Not `Send`, `Sync` or `Unpin`: the library does not say
    // whether that would be safe.
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

// Safety: `OpaqueThinIter_destroy` frees a `Box<OpaqueThinIter>` returned by the library.
unsafe impl crate::DiplomatOpaque for OpaqueThinIter {
    unsafe fn destroy(ptr: *mut Self) {
        OpaqueThinIter_destroy(ptr)
    }
}

extern "C" {
    fn OpaqueThinIter_destroy(this: *mut OpaqueThinIter);
}