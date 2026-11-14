#[repr(C)]
pub struct Opaque {
    // Lives in the library's memory, so it is only ever reached through `&`, `&mut`
    // or `DiplomatBox`. Not `Send`, `Sync` or `Unpin`: the library does not say
    // whether that would be safe.
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl Opaque {
    pub fn new() -> crate::DiplomatBox<crate::Opaque> {
        // Safety: `Box` returns are never null, and the caller owns them.
        unsafe { crate::DiplomatBox::from_raw(Opaque_new()) }
    }

    pub fn from_str(input: &str) -> crate::DiplomatBox<crate::Opaque> {
        // Safety: `Box` returns are never null, and the caller owns them.
        unsafe { crate::DiplomatBox::from_raw(Opaque_from_str(crate::DiplomatSlice::new(input.as_bytes()))) }
    }

    pub fn returns_usize() -> usize {
        unsafe { Opaque_returns_usize() }
    }

    pub fn returns_imported() -> crate::ImportedStruct {
        unsafe { Opaque_returns_imported() }
    }
}

extern "C" {
    fn Opaque_new() -> *mut crate::Opaque;
    fn Opaque_from_str(input: crate::DiplomatSlice<u8>) -> *mut crate::Opaque;
    fn Opaque_returns_usize() -> usize;
    fn Opaque_returns_imported() -> crate::ImportedStruct;
}

// Safety: `Opaque_destroy` frees a `Box<Opaque>` returned by the library.
unsafe impl crate::DiplomatOpaque for Opaque {
    unsafe fn destroy(ptr: *mut Self) {
        Opaque_destroy(ptr)
    }
}

extern "C" {
    fn Opaque_destroy(this: *mut Opaque);
}