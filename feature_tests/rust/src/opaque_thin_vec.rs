/// The type behind `OpaqueThinVec` pointers at the ABI level.
#[repr(C)]
pub(crate) struct OpaqueThinVecRaw {
    _private: [u8; 0],
}

pub struct OpaqueThinVec(pub(crate) core::ptr::NonNull<OpaqueThinVecRaw>);

impl OpaqueThinVec {
    pub fn create(a: &[i32], b: &[f32], c: &[u8]) -> crate::OpaqueThinVec {
        // Safety: `Box` returns are never null, and the wrapper takes ownership.
        unsafe {
            crate::OpaqueThinVec(core::ptr::NonNull::new_unchecked(
                OpaqueThinVec_create(crate::DiplomatSlice::new(a), crate::DiplomatSlice::new(b), crate::DiplomatSlice::new(c)),
            ))
        }
    }

    pub fn len(&self) -> usize {
        unsafe { OpaqueThinVec_len(self.0.as_ptr()) }
    }
}

extern "C" {
    fn OpaqueThinVec_create(a: crate::DiplomatSlice<i32>, b: crate::DiplomatSlice<f32>, c: crate::DiplomatSlice<u8>) -> *mut crate::opaque_thin_vec::OpaqueThinVecRaw;
    fn OpaqueThinVec_len(this: *const OpaqueThinVecRaw) -> usize;
}

impl Drop for OpaqueThinVec {
    fn drop(&mut self) {
        unsafe { OpaqueThinVec_destroy(self.0.as_ptr()) }
    }
}

extern "C" {
    fn OpaqueThinVec_destroy(this: *mut OpaqueThinVecRaw);
}
