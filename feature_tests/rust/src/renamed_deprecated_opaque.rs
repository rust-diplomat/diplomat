#[repr(C)]
pub struct RenamedDeprecatedOpaque {
    // Lives in the library's memory, so it is only ever reached through `&`, `&mut`
    // or `DiplomatBox`. Not `Send`, `Sync` or `Unpin`: the library does not say
    // whether that would be safe.
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

// Safety: `namespace_DeprecatedOpaque_destroy` frees a `Box<RenamedDeprecatedOpaque>` returned by the library.
unsafe impl crate::DiplomatOpaque for RenamedDeprecatedOpaque {
    unsafe fn destroy(ptr: *mut Self) {
        namespace_DeprecatedOpaque_destroy(ptr)
    }
}

extern "C" {
    fn namespace_DeprecatedOpaque_destroy(this: *mut RenamedDeprecatedOpaque);
}