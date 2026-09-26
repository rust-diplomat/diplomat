use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;
#[allow(unused_imports)]
use crate::types::*;

/// Some example docs
/// Back to all docs
pub struct AttrOpaque1Renamed {
    pub(crate) inner: NonNull<ffi::AttrOpaque1Renamed>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct AttrOpaque1RenamedRef<'view> {
    pub(crate) inner: NonNull<ffi::AttrOpaque1Renamed>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct AttrOpaque1RenamedRefMut<'view> {
    pub(crate) inner: NonNull<ffi::AttrOpaque1Renamed>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait AttrOpaque1RenamedSharedArg: crate::private::AttrOpaque1RenamedSharedSealed {}

#[doc(hidden)]
pub trait AttrOpaque1RenamedMutArg: crate::private::AttrOpaque1RenamedMutSealed {}

impl crate::private::AttrOpaque1RenamedSharedSealed for AttrOpaque1Renamed {
    fn __as_const_ptr(&self) -> *const ffi::AttrOpaque1Renamed {
        self.inner.as_ptr()
    }
}

impl crate::private::AttrOpaque1RenamedMutSealed for AttrOpaque1Renamed {
    fn __as_mut_ptr(&mut self) -> *mut ffi::AttrOpaque1Renamed {
        self.inner.as_ptr()
    }
}

impl AttrOpaque1RenamedSharedArg for AttrOpaque1Renamed {}
impl AttrOpaque1RenamedMutArg for AttrOpaque1Renamed {}

impl<'view> crate::private::AttrOpaque1RenamedSharedSealed for AttrOpaque1RenamedRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::AttrOpaque1Renamed {
        self.inner.as_ptr()
    }
}

impl<'view> AttrOpaque1RenamedSharedArg for AttrOpaque1RenamedRef<'view> {}

impl<'view> crate::private::AttrOpaque1RenamedSharedSealed for AttrOpaque1RenamedRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::AttrOpaque1Renamed {
        self.inner.as_ptr()
    }
}

impl<'view> crate::private::AttrOpaque1RenamedMutSealed for AttrOpaque1RenamedRefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::AttrOpaque1Renamed {
        self.inner.as_ptr()
    }
}

impl<'view> AttrOpaque1RenamedSharedArg for AttrOpaque1RenamedRefMut<'view> {}
impl<'view> AttrOpaque1RenamedMutArg for AttrOpaque1RenamedRefMut<'view> {}

impl Drop for AttrOpaque1Renamed {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::namespace_AttrOpaque1_destroy(self.inner.as_ptr()) };
    }
}

impl fmt::Debug for AttrOpaque1Renamed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("AttrOpaque1Renamed")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for AttrOpaque1RenamedRef<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("AttrOpaque1RenamedRef")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for AttrOpaque1RenamedRefMut<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("AttrOpaque1RenamedRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl AttrOpaque1Renamed {
    /// More example docs
    pub fn totally_not_new() -> crate::AttrOpaque1Renamed {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::namespace_AttrOpaque1_new() };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null AttrOpaque1Renamed");
            crate::AttrOpaque1Renamed {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn mac_test() -> i32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::namespace_AttrOpaque1_mac_test() }
    }
    pub fn hello() -> i32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::namespace_AttrOpaque1_hello() }
    }
    pub fn method_renamed(&self) -> u8 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::namespace_AttrOpaque1_method(self.inner.as_ptr() as *const _) }
    }
    pub fn abirenamed(&self) -> u8 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::renamed_on_abi_only(self.inner.as_ptr() as *const _) }
    }
    pub fn use_unnamespaced(&self, _un: &impl crate::UnnamespacedSharedArg) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::namespace_AttrOpaque1_use_unnamespaced(
                self.inner.as_ptr() as *const _,
                crate::private::UnnamespacedSharedSealed::__as_const_ptr(_un),
            )
        };
    }
    pub fn use_namespaced(&self, _n: RenamedAttrEnum) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::namespace_AttrOpaque1_use_namespaced(self.inner.as_ptr() as *const _, _n) };
    }
}

impl<'view> AttrOpaque1RenamedRef<'view> {
    pub fn method_renamed(&self) -> u8 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::namespace_AttrOpaque1_method(self.inner.as_ptr() as *const _) }
    }
    pub fn abirenamed(&self) -> u8 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::renamed_on_abi_only(self.inner.as_ptr() as *const _) }
    }
    pub fn use_unnamespaced(&self, _un: &impl crate::UnnamespacedSharedArg) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::namespace_AttrOpaque1_use_unnamespaced(
                self.inner.as_ptr() as *const _,
                crate::private::UnnamespacedSharedSealed::__as_const_ptr(_un),
            )
        };
    }
    pub fn use_namespaced(&self, _n: RenamedAttrEnum) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::namespace_AttrOpaque1_use_namespaced(self.inner.as_ptr() as *const _, _n) };
    }
}

impl<'view> AttrOpaque1RenamedRefMut<'view> {
    pub fn method_renamed(&self) -> u8 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::namespace_AttrOpaque1_method(self.inner.as_ptr() as *const _) }
    }
    pub fn abirenamed(&self) -> u8 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::renamed_on_abi_only(self.inner.as_ptr() as *const _) }
    }
    pub fn use_unnamespaced(&self, _un: &impl crate::UnnamespacedSharedArg) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::namespace_AttrOpaque1_use_unnamespaced(
                self.inner.as_ptr() as *const _,
                crate::private::UnnamespacedSharedSealed::__as_const_ptr(_un),
            )
        };
    }
    pub fn use_namespaced(&self, _n: RenamedAttrEnum) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::namespace_AttrOpaque1_use_namespaced(self.inner.as_ptr() as *const _, _n) };
    }
}
