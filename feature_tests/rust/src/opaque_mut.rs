#[repr(C)]
pub struct OpaqueMut {
    // Lives in the library's memory, so it is only ever reached through `&`, `&mut`
    // or `DiplomatBox`. Not `Send`, `Sync` or `Unpin`: the library does not say
    // whether that would be safe.
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl OpaqueMut {
    pub fn new() -> crate::DiplomatBox<crate::OpaqueMut> {
        // Safety: `Box` returns are never null, and the caller owns them.
        unsafe { crate::DiplomatBox::from_raw(OpaqueMut_new()) }
    }
}

extern "C" {
    fn OpaqueMut_new() -> *mut crate::OpaqueMut;
}

// Safety: `OpaqueMut_destroy` frees a `Box<OpaqueMut>` returned by the library.
unsafe impl crate::DiplomatOpaque for OpaqueMut {
    unsafe fn destroy(ptr: *mut Self) {
        OpaqueMut_destroy(ptr)
    }
}

extern "C" {
    fn OpaqueMut_destroy(this: *mut OpaqueMut);
}