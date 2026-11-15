/// The type behind `MyString` pointers at the ABI level.
#[repr(C)]
pub(crate) struct MyStringRaw {
    _private: [u8; 0],
}

pub struct MyString(pub(crate) core::ptr::NonNull<MyStringRaw>);

impl MyString {
    pub fn new(v: &[u8]) -> crate::MyString {
        // Safety: `Box` returns are never null, and the wrapper takes ownership.
        unsafe {
            crate::MyString(core::ptr::NonNull::new_unchecked(
                MyString_new(crate::DiplomatSlice::new(v)),
            ))
        }
    }

    pub fn new_unsafe(v: &str) -> crate::MyString {
        // Safety: `Box` returns are never null, and the wrapper takes ownership.
        unsafe {
            crate::MyString(core::ptr::NonNull::new_unchecked(
                MyString_new_unsafe(crate::DiplomatSlice::new(v.as_bytes())),
            ))
        }
    }

    pub fn set_str(&mut self, new_str: &[u8]) {
        unsafe { MyString_set_str(self.0.as_ptr(), crate::DiplomatSlice::new(new_str)) }
    }
}

extern "C" {
    fn MyString_new(v: crate::DiplomatSlice<u8>) -> *mut crate::my_string::MyStringRaw;
    fn MyString_new_unsafe(v: crate::DiplomatSlice<u8>) -> *mut crate::my_string::MyStringRaw;
    fn MyString_set_str(this: *mut MyStringRaw, new_str: crate::DiplomatSlice<u8>);
}

impl Drop for MyString {
    fn drop(&mut self) {
        unsafe { MyString_destroy(self.0.as_ptr()) }
    }
}

extern "C" {
    fn MyString_destroy(this: *mut MyStringRaw);
}
