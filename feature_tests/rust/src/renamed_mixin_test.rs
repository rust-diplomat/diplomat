/// The type behind `RenamedMixinTest` pointers at the ABI level.
#[repr(C)]
pub(crate) struct RenamedMixinTestRaw {
    _private: [u8; 0],
}

pub struct RenamedMixinTest(pub(crate) core::ptr::NonNull<RenamedMixinTestRaw>);

impl Drop for RenamedMixinTest {
    fn drop(&mut self) {
        unsafe { namespace_MixinTest_destroy(self.0.as_ptr()) }
    }
}

extern "C" {
    fn namespace_MixinTest_destroy(this: *mut RenamedMixinTestRaw);
}
