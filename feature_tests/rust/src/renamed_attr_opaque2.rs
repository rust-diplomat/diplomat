#[repr(C)]
pub struct RenamedAttrOpaque2 {
    // Lives in the library's memory, so it is only ever reached through `&`, `&mut`
    // or `DiplomatBox`. Not `Send`, `Sync` or `Unpin`: the library does not say
    // whether that would be safe.
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

// Safety: `namespace_AttrOpaque2_destroy` frees a `Box<RenamedAttrOpaque2>` returned by the library.
unsafe impl crate::DiplomatOpaque for RenamedAttrOpaque2 {
    unsafe fn destroy(ptr: *mut Self) {
        namespace_AttrOpaque2_destroy(ptr)
    }
}

extern "C" {
    fn namespace_AttrOpaque2_destroy(this: *mut RenamedAttrOpaque2);
}