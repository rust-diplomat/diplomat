#[repr(C)]
pub struct Foo {
    // Lives in the library's memory, so it is only ever reached through `&`, `&mut`
    // or `DiplomatBox`. Not `Send`, `Sync` or `Unpin`: the library does not say
    // whether that would be safe.
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

// Safety: `Foo_destroy` frees a `Box<Foo>` returned by the library.
unsafe impl crate::DiplomatOpaque for Foo {
    unsafe fn destroy(ptr: *mut Self) {
        Foo_destroy(ptr)
    }
}

extern "C" {
    fn Foo_destroy(this: *mut Foo);
}