#[repr(C)]
pub struct RenamedOpaqueZSTIndexer {
    // Lives in the library's memory, so it is only ever reached through `&`, `&mut`
    // or `DiplomatBox`. Not `Send`, `Sync` or `Unpin`: the library does not say
    // whether that would be safe.
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl RenamedOpaqueZSTIndexer {
    pub fn new() -> crate::DiplomatBox<crate::RenamedOpaqueZSTIndexer> {
        // Safety: `Box` returns are never null, and the caller owns them.
        unsafe { crate::DiplomatBox::from_raw(namespace_OpaqueZSTIndexer_new()) }
    }
}

extern "C" {
    fn namespace_OpaqueZSTIndexer_new() -> *mut crate::RenamedOpaqueZSTIndexer;
}

// Safety: `namespace_OpaqueZSTIndexer_destroy` frees a `Box<RenamedOpaqueZSTIndexer>` returned by the library.
unsafe impl crate::DiplomatOpaque for RenamedOpaqueZSTIndexer {
    unsafe fn destroy(ptr: *mut Self) {
        namespace_OpaqueZSTIndexer_destroy(ptr)
    }
}

extern "C" {
    fn namespace_OpaqueZSTIndexer_destroy(this: *mut RenamedOpaqueZSTIndexer);
}