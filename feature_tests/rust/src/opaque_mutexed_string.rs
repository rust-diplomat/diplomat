/// The type behind `OpaqueMutexedString` pointers at the ABI level.
#[repr(C)]
pub(crate) struct OpaqueMutexedStringRaw {
    _private: [u8; 0],
}

pub struct OpaqueMutexedString(pub(crate) core::ptr::NonNull<OpaqueMutexedStringRaw>);

impl OpaqueMutexedString {
    pub fn from_usize(number: usize) -> crate::OpaqueMutexedString {
        // Safety: `Box` returns are never null, and the wrapper takes ownership.
        unsafe {
            crate::OpaqueMutexedString(core::ptr::NonNull::new_unchecked(
                OpaqueMutexedString_from_usize(number),
            ))
        }
    }

    pub fn change(&self, number: usize) {
        unsafe { OpaqueMutexedString_change(self.0.as_ptr(), number) }
    }

    pub fn get_len_and_add(&self, other: usize) -> usize {
        unsafe { OpaqueMutexedString_get_len_and_add(self.0.as_ptr(), other) }
    }

    pub fn wrapper(&self) -> crate::Utf16Wrap {
        // Safety: `Box` returns are never null, and the wrapper takes ownership.
        unsafe {
            crate::Utf16Wrap(core::ptr::NonNull::new_unchecked(
                OpaqueMutexedString_wrapper(self.0.as_ptr()),
            ))
        }
    }

    pub fn to_unsigned_from_unsigned(&self, input: u16) -> u16 {
        unsafe { OpaqueMutexedString_to_unsigned_from_unsigned(self.0.as_ptr(), input) }
    }
}

extern "C" {
    fn OpaqueMutexedString_from_usize(number: usize) -> *mut crate::opaque_mutexed_string::OpaqueMutexedStringRaw;
    fn OpaqueMutexedString_change(this: *const OpaqueMutexedStringRaw, number: usize);
    fn OpaqueMutexedString_get_len_and_add(this: *const OpaqueMutexedStringRaw, other: usize) -> usize;
    fn OpaqueMutexedString_wrapper(this: *const OpaqueMutexedStringRaw) -> *mut crate::utf16_wrap::Utf16WrapRaw;
    fn OpaqueMutexedString_to_unsigned_from_unsigned(this: *const OpaqueMutexedStringRaw, input: u16) -> u16;
}

impl Drop for OpaqueMutexedString {
    fn drop(&mut self) {
        unsafe { OpaqueMutexedString_destroy(self.0.as_ptr()) }
    }
}

extern "C" {
    fn OpaqueMutexedString_destroy(this: *mut OpaqueMutexedStringRaw);
}
