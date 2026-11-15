/// The type behind `RefList` pointers at the ABI level.
#[repr(C)]
pub(crate) struct RefListRaw {
    _private: [u8; 0],
}

pub struct RefList(pub(crate) core::ptr::NonNull<RefListRaw>);

impl Drop for RefList {
    fn drop(&mut self) {
        unsafe { RefList_destroy(self.0.as_ptr()) }
    }
}

extern "C" {
    fn RefList_destroy(this: *mut RefListRaw);
}
