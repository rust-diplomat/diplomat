/// The type behind `MyOpaqueEnum` pointers at the ABI level.
#[repr(C)]
pub(crate) struct MyOpaqueEnumRaw {
    _private: [u8; 0],
}

pub struct MyOpaqueEnum(pub(crate) core::ptr::NonNull<MyOpaqueEnumRaw>);

impl MyOpaqueEnum {
    pub fn new() -> crate::MyOpaqueEnum {
        // Safety: `Box` returns are never null, and the wrapper takes ownership.
        unsafe {
            crate::MyOpaqueEnum(core::ptr::NonNull::new_unchecked(
                MyOpaqueEnum_new(),
            ))
        }
    }
}

extern "C" {
    fn MyOpaqueEnum_new() -> *mut crate::my_opaque_enum::MyOpaqueEnumRaw;
}

impl Drop for MyOpaqueEnum {
    fn drop(&mut self) {
        unsafe { MyOpaqueEnum_destroy(self.0.as_ptr()) }
    }
}

extern "C" {
    fn MyOpaqueEnum_destroy(this: *mut MyOpaqueEnumRaw);
}
