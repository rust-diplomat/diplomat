#[repr(C)]
pub struct MyOpaqueEnum {
    // Lives in the library's memory, so it is only ever reached through `&`, `&mut`
    // or `DiplomatBox`. Not `Send`, `Sync` or `Unpin`: the library does not say
    // whether that would be safe.
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl MyOpaqueEnum {
    pub fn new() -> crate::DiplomatBox<crate::MyOpaqueEnum> {
        // Safety: `Box` returns are never null, and the caller owns them.
        unsafe { crate::DiplomatBox::from_raw(MyOpaqueEnum_new()) }
    }
}

extern "C" {
    fn MyOpaqueEnum_new() -> *mut crate::MyOpaqueEnum;
}

// Safety: `MyOpaqueEnum_destroy` frees a `Box<MyOpaqueEnum>` returned by the library.
unsafe impl crate::DiplomatOpaque for MyOpaqueEnum {
    unsafe fn destroy(ptr: *mut Self) {
        MyOpaqueEnum_destroy(ptr)
    }
}

extern "C" {
    fn MyOpaqueEnum_destroy(this: *mut MyOpaqueEnum);
}