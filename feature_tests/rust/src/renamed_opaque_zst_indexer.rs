/// The type behind `RenamedOpaqueZSTIndexer` pointers at the ABI level.
#[repr(C)]
pub(crate) struct RenamedOpaqueZSTIndexerRaw {
    _private: [u8; 0],
}

pub struct RenamedOpaqueZSTIndexer(pub(crate) core::ptr::NonNull<RenamedOpaqueZSTIndexerRaw>);

impl RenamedOpaqueZSTIndexer {
    pub fn new() -> crate::RenamedOpaqueZSTIndexer {
        // Safety: `Box` returns are never null, and the wrapper takes ownership.
        unsafe {
            crate::RenamedOpaqueZSTIndexer(core::ptr::NonNull::new_unchecked(
                namespace_OpaqueZSTIndexer_new(),
            ))
        }
    }
}

extern "C" {
    fn namespace_OpaqueZSTIndexer_new() -> *mut crate::renamed_opaque_zst_indexer::RenamedOpaqueZSTIndexerRaw;
}

impl Drop for RenamedOpaqueZSTIndexer {
    fn drop(&mut self) {
        unsafe { namespace_OpaqueZSTIndexer_destroy(self.0.as_ptr()) }
    }
}

extern "C" {
    fn namespace_OpaqueZSTIndexer_destroy(this: *mut RenamedOpaqueZSTIndexerRaw);
}
