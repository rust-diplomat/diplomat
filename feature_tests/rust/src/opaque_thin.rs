/// The type behind `OpaqueThin` pointers at the ABI level.
#[repr(C)]
pub(crate) struct OpaqueThinRaw {
    _private: [u8; 0],
}

pub struct OpaqueThin(pub(crate) core::ptr::NonNull<OpaqueThinRaw>);

impl OpaqueThin {
    pub fn a(&self) -> i32 {
        unsafe { OpaqueThin_a(self.0.as_ptr()) }
    }

    pub fn b(&self) -> f32 {
        unsafe { OpaqueThin_b(self.0.as_ptr()) }
    }
}

extern "C" {
    fn OpaqueThin_a(this: *const OpaqueThinRaw) -> i32;
    fn OpaqueThin_b(this: *const OpaqueThinRaw) -> f32;
}

impl Drop for OpaqueThin {
    fn drop(&mut self) {
        unsafe { OpaqueThin_destroy(self.0.as_ptr()) }
    }
}

extern "C" {
    fn OpaqueThin_destroy(this: *mut OpaqueThinRaw);
}
