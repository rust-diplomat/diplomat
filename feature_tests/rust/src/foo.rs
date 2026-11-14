/// The type behind `Foo` pointers at the ABI level.
#[repr(C)]
pub(crate) struct FooRaw {
    _private: [u8; 0],
}

pub struct Foo(pub(crate) core::ptr::NonNull<FooRaw>);

impl Drop for Foo {
    fn drop(&mut self) {
        unsafe { Foo_destroy(self.0.as_ptr()) }
    }
}

extern "C" {
    fn Foo_destroy(this: *mut FooRaw);
}
