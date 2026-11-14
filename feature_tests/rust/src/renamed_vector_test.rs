/// The type behind `RenamedVectorTest` pointers at the ABI level.
#[repr(C)]
pub(crate) struct RenamedVectorTestRaw {
    _private: [u8; 0],
}

pub struct RenamedVectorTest(pub(crate) core::ptr::NonNull<RenamedVectorTestRaw>);

impl RenamedVectorTest {
    pub fn new() -> crate::RenamedVectorTest {
        // Safety: `Box` returns are never null, and the wrapper takes ownership.
        unsafe {
            crate::RenamedVectorTest(core::ptr::NonNull::new_unchecked(
                namespace_VectorTest_new(),
            ))
        }
    }

    pub fn len(&self) -> usize {
        unsafe { namespace_VectorTest_len(self.0.as_ptr()) }
    }

    pub fn push(&mut self, value: f64) {
        unsafe { namespace_VectorTest_push(self.0.as_ptr(), value) }
    }
}

extern "C" {
    fn namespace_VectorTest_new() -> *mut crate::renamed_vector_test::RenamedVectorTestRaw;
    fn namespace_VectorTest_len(this: *const RenamedVectorTestRaw) -> usize;
    fn namespace_VectorTest_push(this: *mut RenamedVectorTestRaw, value: f64);
}

impl Drop for RenamedVectorTest {
    fn drop(&mut self) {
        unsafe { namespace_VectorTest_destroy(self.0.as_ptr()) }
    }
}

extern "C" {
    fn namespace_VectorTest_destroy(this: *mut RenamedVectorTestRaw);
}
