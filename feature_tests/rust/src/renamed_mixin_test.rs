#[repr(C)]
pub struct RenamedMixinTest {
    // Lives in the library's memory, so it is only ever reached through `&`, `&mut`
    // or `DiplomatBox`. Not `Send`, `Sync` or `Unpin`: the library does not say
    // whether that would be safe.
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

// Safety: `namespace_MixinTest_destroy` frees a `Box<RenamedMixinTest>` returned by the library.
unsafe impl crate::DiplomatOpaque for RenamedMixinTest {
    unsafe fn destroy(ptr: *mut Self) {
        namespace_MixinTest_destroy(ptr)
    }
}

extern "C" {
    fn namespace_MixinTest_destroy(this: *mut RenamedMixinTest);
}