/// The type behind `Bar` pointers at the ABI level.
#[repr(C)]
pub(crate) struct BarRaw {
    _private: [u8; 0],
}

pub struct Bar(pub(crate) core::ptr::NonNull<BarRaw>);

impl Drop for Bar {
    fn drop(&mut self) {
        unsafe { Bar_destroy(self.0.as_ptr()) }
    }
}

extern "C" {
    fn Bar_destroy(this: *mut BarRaw);
}
