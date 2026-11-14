/// Some example docs
/// Back to all docs
#[repr(C)]
pub struct AttrOpaque1Renamed {
    // Lives in the library's memory, so it is only ever reached through `&`, `&mut`
    // or `DiplomatBox`. Not `Send`, `Sync` or `Unpin`: the library does not say
    // whether that would be safe.
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl AttrOpaque1Renamed {
    /// More example docs
    pub fn new() -> crate::DiplomatBox<crate::AttrOpaque1Renamed> {
        // Safety: `Box` returns are never null, and the caller owns them.
        unsafe { crate::DiplomatBox::from_raw(namespace_AttrOpaque1_new()) }
    }

    pub fn mac_test() -> i32 {
        unsafe { namespace_AttrOpaque1_mac_test() }
    }

    pub fn hello() -> i32 {
        unsafe { namespace_AttrOpaque1_hello() }
    }

    pub fn method(&self) -> u8 {
        unsafe { namespace_AttrOpaque1_method(self) }
    }

    pub fn abirenamed(&self) -> u8 {
        unsafe { renamed_on_abi_only(self) }
    }

    pub fn use_unnamespaced(&self, _un: &crate::Unnamespaced) {
        unsafe { namespace_AttrOpaque1_use_unnamespaced(self, _un) }
    }

    pub fn use_namespaced(&self, _n: crate::RenamedAttrEnum) {
        unsafe { namespace_AttrOpaque1_use_namespaced(self, _n) }
    }
}

extern "C" {
    fn namespace_AttrOpaque1_new() -> *mut crate::AttrOpaque1Renamed;
    fn namespace_AttrOpaque1_mac_test() -> i32;
    fn namespace_AttrOpaque1_hello() -> i32;
    fn namespace_AttrOpaque1_method(this: *const AttrOpaque1Renamed) -> u8;
    fn renamed_on_abi_only(this: *const AttrOpaque1Renamed) -> u8;
    fn namespace_AttrOpaque1_use_unnamespaced(this: *const AttrOpaque1Renamed, _un: *const crate::Unnamespaced);
    fn namespace_AttrOpaque1_use_namespaced(this: *const AttrOpaque1Renamed, _n: crate::RenamedAttrEnum);
}

// Safety: `namespace_AttrOpaque1_destroy` frees a `Box<AttrOpaque1Renamed>` returned by the library.
unsafe impl crate::DiplomatOpaque for AttrOpaque1Renamed {
    unsafe fn destroy(ptr: *mut Self) {
        namespace_AttrOpaque1_destroy(ptr)
    }
}

extern "C" {
    fn namespace_AttrOpaque1_destroy(this: *mut AttrOpaque1Renamed);
}