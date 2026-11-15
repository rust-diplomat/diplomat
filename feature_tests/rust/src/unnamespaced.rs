/// The type behind `Unnamespaced` pointers at the ABI level.
#[repr(C)]
pub(crate) struct UnnamespacedRaw {
    _private: [u8; 0],
}

pub struct Unnamespaced(pub(crate) core::ptr::NonNull<UnnamespacedRaw>);

impl Unnamespaced {
    pub fn make(_e: crate::RenamedAttrEnum) -> crate::Unnamespaced {
        // Safety: `Box` returns are never null, and the wrapper takes ownership.
        unsafe {
            crate::Unnamespaced(core::ptr::NonNull::new_unchecked(
                namespace_Unnamespaced_make(_e),
            ))
        }
    }

    pub fn use_namespaced(&self, _n: &crate::AttrOpaque1Renamed) {
        unsafe { namespace_Unnamespaced_use_namespaced(self.0.as_ptr(), _n.0.as_ptr()) }
    }
}

extern "C" {
    fn namespace_Unnamespaced_make(_e: crate::RenamedAttrEnum) -> *mut crate::unnamespaced::UnnamespacedRaw;
    fn namespace_Unnamespaced_use_namespaced(this: *const UnnamespacedRaw, _n: *const crate::attr_opaque1_renamed::AttrOpaque1RenamedRaw);
}

impl Drop for Unnamespaced {
    fn drop(&mut self) {
        unsafe { namespace_Unnamespaced_destroy(self.0.as_ptr()) }
    }
}

extern "C" {
    fn namespace_Unnamespaced_destroy(this: *mut UnnamespacedRaw);
}
