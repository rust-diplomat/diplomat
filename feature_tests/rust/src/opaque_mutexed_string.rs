#[repr(C)]
pub struct OpaqueMutexedString {
    // Lives in the library's memory, so it is only ever reached through `&`, `&mut`
    // or `DiplomatBox`. Not `Send`, `Sync` or `Unpin`: the library does not say
    // whether that would be safe.
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl OpaqueMutexedString {
    pub fn from_usize(number: usize) -> crate::DiplomatBox<crate::OpaqueMutexedString> {
        // Safety: `Box` returns are never null, and the caller owns them.
        unsafe { crate::DiplomatBox::from_raw(OpaqueMutexedString_from_usize(number)) }
    }

    pub fn change(&self, number: usize) {
        unsafe { OpaqueMutexedString_change(self, number) }
    }

    pub fn borrow<'a>(&'a self) -> &'a crate::OpaqueMutexedString {
        // Safety: Borrowed returns are never null, and live as long as the signature says.
        unsafe { &*OpaqueMutexedString_borrow(self) }
    }

    pub fn borrow_other<'a>(other: &'a crate::OpaqueMutexedString) -> &'a crate::OpaqueMutexedString {
        // Safety: Borrowed returns are never null, and live as long as the signature says.
        unsafe { &*OpaqueMutexedString_borrow_other(other) }
    }

    pub fn borrow_self_or_other<'a>(&'a self, other: &'a crate::OpaqueMutexedString) -> &'a crate::OpaqueMutexedString {
        // Safety: Borrowed returns are never null, and live as long as the signature says.
        unsafe { &*OpaqueMutexedString_borrow_self_or_other(self, other) }
    }

    pub fn get_len_and_add(&self, other: usize) -> usize {
        unsafe { OpaqueMutexedString_get_len_and_add(self, other) }
    }

    pub fn wrapper(&self) -> crate::DiplomatBox<crate::Utf16Wrap> {
        // Safety: `Box` returns are never null, and the caller owns them.
        unsafe { crate::DiplomatBox::from_raw(OpaqueMutexedString_wrapper(self)) }
    }

    pub fn to_unsigned_from_unsigned(&self, input: u16) -> u16 {
        unsafe { OpaqueMutexedString_to_unsigned_from_unsigned(self, input) }
    }
}

extern "C" {
    fn OpaqueMutexedString_from_usize(number: usize) -> *mut crate::OpaqueMutexedString;
    fn OpaqueMutexedString_change(this: *const OpaqueMutexedString, number: usize);
    fn OpaqueMutexedString_borrow(this: *const OpaqueMutexedString) -> *const crate::OpaqueMutexedString;
    fn OpaqueMutexedString_borrow_other(other: *const crate::OpaqueMutexedString) -> *const crate::OpaqueMutexedString;
    fn OpaqueMutexedString_borrow_self_or_other(this: *const OpaqueMutexedString, other: *const crate::OpaqueMutexedString) -> *const crate::OpaqueMutexedString;
    fn OpaqueMutexedString_get_len_and_add(this: *const OpaqueMutexedString, other: usize) -> usize;
    fn OpaqueMutexedString_wrapper(this: *const OpaqueMutexedString) -> *mut crate::Utf16Wrap;
    fn OpaqueMutexedString_to_unsigned_from_unsigned(this: *const OpaqueMutexedString, input: u16) -> u16;
}

// Safety: `OpaqueMutexedString_destroy` frees a `Box<OpaqueMutexedString>` returned by the library.
unsafe impl crate::DiplomatOpaque for OpaqueMutexedString {
    unsafe fn destroy(ptr: *mut Self) {
        OpaqueMutexedString_destroy(ptr)
    }
}

extern "C" {
    fn OpaqueMutexedString_destroy(this: *mut OpaqueMutexedString);
}