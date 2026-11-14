/// The type behind `OpaqueThinIter` pointers at the ABI level.
#[repr(C)]
pub(crate) struct OpaqueThinIterRaw {
    _private: [u8; 0],
}

pub struct OpaqueThinIter(pub(crate) core::ptr::NonNull<OpaqueThinIterRaw>);

impl Drop for OpaqueThinIter {
    fn drop(&mut self) {
        unsafe { OpaqueThinIter_destroy(self.0.as_ptr()) }
    }
}

extern "C" {
    fn OpaqueThinIter_destroy(this: *mut OpaqueThinIterRaw);
}
