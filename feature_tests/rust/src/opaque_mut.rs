/// The type behind `OpaqueMut` pointers at the ABI level.
#[repr(C)]
pub(crate) struct OpaqueMutRaw {
    _private: [u8; 0],
}

pub struct OpaqueMut(pub(crate) core::ptr::NonNull<OpaqueMutRaw>);

impl OpaqueMut {
    pub fn new() -> crate::OpaqueMut {
        // Safety: `Box` returns are never null, and the wrapper takes ownership.
        unsafe {
            crate::OpaqueMut(core::ptr::NonNull::new_unchecked(
                OpaqueMut_new(),
            ))
        }
    }
}

extern "C" {
    fn OpaqueMut_new() -> *mut crate::opaque_mut::OpaqueMutRaw;
}

impl Drop for OpaqueMut {
    fn drop(&mut self) {
        unsafe { OpaqueMut_destroy(self.0.as_ptr()) }
    }
}

extern "C" {
    fn OpaqueMut_destroy(this: *mut OpaqueMutRaw);
}
