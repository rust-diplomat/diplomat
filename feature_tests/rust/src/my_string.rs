#[repr(C)]
pub struct MyString {
    // Lives in the library's memory, so it is only ever reached through `&`, `&mut`
    // or `DiplomatBox`. Not `Send`, `Sync` or `Unpin`: the library does not say
    // whether that would be safe.
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl MyString {
    pub fn new(v: &[u8]) -> crate::DiplomatBox<crate::MyString> {
        // Safety: `Box` returns are never null, and the caller owns them.
        unsafe { crate::DiplomatBox::from_raw(MyString_new(crate::DiplomatSlice::new(v))) }
    }

    pub fn new_unsafe(v: &str) -> crate::DiplomatBox<crate::MyString> {
        // Safety: `Box` returns are never null, and the caller owns them.
        unsafe { crate::DiplomatBox::from_raw(MyString_new_unsafe(crate::DiplomatSlice::new(v.as_bytes()))) }
    }

    pub fn set_str(&mut self, new_str: &[u8]) {
        unsafe { MyString_set_str(self, crate::DiplomatSlice::new(new_str)) }
    }
}

extern "C" {
    fn MyString_new(v: crate::DiplomatSlice<u8>) -> *mut crate::MyString;
    fn MyString_new_unsafe(v: crate::DiplomatSlice<u8>) -> *mut crate::MyString;
    fn MyString_set_str(this: *mut MyString, new_str: crate::DiplomatSlice<u8>);
}

// Safety: `MyString_destroy` frees a `Box<MyString>` returned by the library.
unsafe impl crate::DiplomatOpaque for MyString {
    unsafe fn destroy(ptr: *mut Self) {
        MyString_destroy(ptr)
    }
}

extern "C" {
    fn MyString_destroy(this: *mut MyString);
}