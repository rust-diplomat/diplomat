/// The type behind `AttrOpaque1Renamed` pointers at the ABI level.
#[repr(C)]
pub(crate) struct AttrOpaque1RenamedRaw {
    _private: [u8; 0],
}

/// Some example docs
/// Back to all docs
pub struct AttrOpaque1Renamed(pub(crate) core::ptr::NonNull<AttrOpaque1RenamedRaw>);

impl AttrOpaque1Renamed {
    /// More example docs
    pub fn new() -> crate::AttrOpaque1Renamed {
        // Safety: `Box` returns are never null, and the wrapper takes ownership.
        unsafe {
            crate::AttrOpaque1Renamed(core::ptr::NonNull::new_unchecked(
                namespace_AttrOpaque1_new(),
            ))
        }
    }

    pub fn mac_test() -> i32 {
        unsafe { namespace_AttrOpaque1_mac_test() }
    }

    pub fn hello() -> i32 {
        unsafe { namespace_AttrOpaque1_hello() }
    }

    pub fn method(&self) -> u8 {
        unsafe { namespace_AttrOpaque1_method(self.0.as_ptr()) }
    }

    pub fn abirenamed(&self) -> u8 {
        unsafe { renamed_on_abi_only(self.0.as_ptr()) }
    }

    pub fn use_unnamespaced(&self, _un: &crate::Unnamespaced) {
        unsafe { namespace_AttrOpaque1_use_unnamespaced(self.0.as_ptr(), _un.0.as_ptr()) }
    }

    pub fn use_namespaced(&self, _n: crate::RenamedAttrEnum) {
        unsafe { namespace_AttrOpaque1_use_namespaced(self.0.as_ptr(), _n) }
    }
}

extern "C" {
    fn namespace_AttrOpaque1_new() -> *mut crate::attr_opaque1_renamed::AttrOpaque1RenamedRaw;
    fn namespace_AttrOpaque1_mac_test() -> i32;
    fn namespace_AttrOpaque1_hello() -> i32;
    fn namespace_AttrOpaque1_method(this: *const AttrOpaque1RenamedRaw) -> u8;
    fn renamed_on_abi_only(this: *const AttrOpaque1RenamedRaw) -> u8;
    fn namespace_AttrOpaque1_use_unnamespaced(this: *const AttrOpaque1RenamedRaw, _un: *const crate::unnamespaced::UnnamespacedRaw);
    fn namespace_AttrOpaque1_use_namespaced(this: *const AttrOpaque1RenamedRaw, _n: crate::RenamedAttrEnum);
}

impl Drop for AttrOpaque1Renamed {
    fn drop(&mut self) {
        unsafe { namespace_AttrOpaque1_destroy(self.0.as_ptr()) }
    }
}

extern "C" {
    fn namespace_AttrOpaque1_destroy(this: *mut AttrOpaque1RenamedRaw);
}
