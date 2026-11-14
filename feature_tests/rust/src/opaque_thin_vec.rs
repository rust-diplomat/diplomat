#[repr(C)]
pub struct OpaqueThinVec {
    // Lives in the library's memory, so it is only ever reached through `&`, `&mut`
    // or `DiplomatBox`. Not `Send`, `Sync` or `Unpin`: the library does not say
    // whether that would be safe.
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl OpaqueThinVec {
    pub fn create(a: &[i32], b: &[f32], c: &[u8]) -> crate::DiplomatBox<crate::OpaqueThinVec> {
        // Safety: `Box` returns are never null, and the caller owns them.
        unsafe { crate::DiplomatBox::from_raw(OpaqueThinVec_create(crate::DiplomatSlice::new(a), crate::DiplomatSlice::new(b), crate::DiplomatSlice::new(c))) }
    }

    pub fn len(&self) -> usize {
        unsafe { OpaqueThinVec_len(self) }
    }
}

extern "C" {
    fn OpaqueThinVec_create(a: crate::DiplomatSlice<i32>, b: crate::DiplomatSlice<f32>, c: crate::DiplomatSlice<u8>) -> *mut crate::OpaqueThinVec;
    fn OpaqueThinVec_len(this: *const OpaqueThinVec) -> usize;
}

// Safety: `OpaqueThinVec_destroy` frees a `Box<OpaqueThinVec>` returned by the library.
unsafe impl crate::DiplomatOpaque for OpaqueThinVec {
    unsafe fn destroy(ptr: *mut Self) {
        OpaqueThinVec_destroy(ptr)
    }
}

extern "C" {
    fn OpaqueThinVec_destroy(this: *mut OpaqueThinVec);
}