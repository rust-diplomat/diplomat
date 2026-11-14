#[repr(C)]
pub struct RenamedVectorTest {
    // Lives in the library's memory, so it is only ever reached through `&`, `&mut`
    // or `DiplomatBox`. Not `Send`, `Sync` or `Unpin`: the library does not say
    // whether that would be safe.
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl RenamedVectorTest {
    pub fn new() -> crate::DiplomatBox<crate::RenamedVectorTest> {
        // Safety: `Box` returns are never null, and the caller owns them.
        unsafe { crate::DiplomatBox::from_raw(namespace_VectorTest_new()) }
    }

    pub fn len(&self) -> usize {
        unsafe { namespace_VectorTest_len(self) }
    }

    pub fn push(&mut self, value: f64) {
        unsafe { namespace_VectorTest_push(self, value) }
    }
}

extern "C" {
    fn namespace_VectorTest_new() -> *mut crate::RenamedVectorTest;
    fn namespace_VectorTest_len(this: *const RenamedVectorTest) -> usize;
    fn namespace_VectorTest_push(this: *mut RenamedVectorTest, value: f64);
}

// Safety: `namespace_VectorTest_destroy` frees a `Box<RenamedVectorTest>` returned by the library.
unsafe impl crate::DiplomatOpaque for RenamedVectorTest {
    unsafe fn destroy(ptr: *mut Self) {
        namespace_VectorTest_destroy(ptr)
    }
}

extern "C" {
    fn namespace_VectorTest_destroy(this: *mut RenamedVectorTest);
}