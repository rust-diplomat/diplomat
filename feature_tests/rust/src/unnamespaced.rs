#[repr(C)]
pub struct Unnamespaced {
    // Lives in the library's memory, so it is only ever reached through `&`, `&mut`
    // or `DiplomatBox`. Not `Send`, `Sync` or `Unpin`: the library does not say
    // whether that would be safe.
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl Unnamespaced {
    pub fn make(_e: crate::RenamedAttrEnum) -> crate::DiplomatBox<crate::Unnamespaced> {
        // Safety: `Box` returns are never null, and the caller owns them.
        unsafe { crate::DiplomatBox::from_raw(namespace_Unnamespaced_make(_e)) }
    }

    pub fn use_namespaced(&self, _n: &crate::AttrOpaque1Renamed) {
        unsafe { namespace_Unnamespaced_use_namespaced(self, _n) }
    }
}

extern "C" {
    fn namespace_Unnamespaced_make(_e: crate::RenamedAttrEnum) -> *mut crate::Unnamespaced;
    fn namespace_Unnamespaced_use_namespaced(this: *const Unnamespaced, _n: *const crate::AttrOpaque1Renamed);
}

// Safety: `namespace_Unnamespaced_destroy` frees a `Box<Unnamespaced>` returned by the library.
unsafe impl crate::DiplomatOpaque for Unnamespaced {
    unsafe fn destroy(ptr: *mut Self) {
        namespace_Unnamespaced_destroy(ptr)
    }
}

extern "C" {
    fn namespace_Unnamespaced_destroy(this: *mut Unnamespaced);
}