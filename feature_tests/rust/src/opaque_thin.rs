#[repr(C)]
pub struct OpaqueThin {
    // Lives in the library's memory, so it is only ever reached through `&`, `&mut`
    // or `DiplomatBox`. Not `Send`, `Sync` or `Unpin`: the library does not say
    // whether that would be safe.
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl OpaqueThin {
    pub fn a(&self) -> i32 {
        unsafe { OpaqueThin_a(self) }
    }

    pub fn b(&self) -> f32 {
        unsafe { OpaqueThin_b(self) }
    }
}

extern "C" {
    fn OpaqueThin_a(this: *const OpaqueThin) -> i32;
    fn OpaqueThin_b(this: *const OpaqueThin) -> f32;
}

// Safety: `OpaqueThin_destroy` frees a `Box<OpaqueThin>` returned by the library.
unsafe impl crate::DiplomatOpaque for OpaqueThin {
    unsafe fn destroy(ptr: *mut Self) {
        OpaqueThin_destroy(ptr)
    }
}

extern "C" {
    fn OpaqueThin_destroy(this: *mut OpaqueThin);
}