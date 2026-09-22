use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;
#[allow(unused_imports)]
use crate::types::*;

pub struct Unnamespaced {
    pub(crate) inner: NonNull<ffi::Unnamespaced>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct UnnamespacedRef<'view> {
    pub(crate) inner: NonNull<ffi::Unnamespaced>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct UnnamespacedRefMut<'view> {
    pub(crate) inner: NonNull<ffi::Unnamespaced>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait UnnamespacedSharedArg: crate::private::UnnamespacedSharedSealed {}

#[doc(hidden)]
pub trait UnnamespacedMutArg: crate::private::UnnamespacedMutSealed {}

impl crate::private::UnnamespacedSharedSealed for Unnamespaced {
    fn __as_const_ptr(&self) -> *const ffi::Unnamespaced {
        self.inner.as_ptr()
    }
}

impl crate::private::UnnamespacedMutSealed for Unnamespaced {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Unnamespaced {
        self.inner.as_ptr()
    }
}

impl UnnamespacedSharedArg for Unnamespaced {}
impl UnnamespacedMutArg for Unnamespaced {}

impl<'view> crate::private::UnnamespacedSharedSealed for UnnamespacedRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::Unnamespaced {
        self.inner.as_ptr()
    }
}

impl<'view> UnnamespacedSharedArg for UnnamespacedRef<'view> {}

impl<'view> crate::private::UnnamespacedSharedSealed for UnnamespacedRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::Unnamespaced {
        self.inner.as_ptr()
    }
}

impl<'view> crate::private::UnnamespacedMutSealed for UnnamespacedRefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Unnamespaced {
        self.inner.as_ptr()
    }
}

impl<'view> UnnamespacedSharedArg for UnnamespacedRefMut<'view> {}
impl<'view> UnnamespacedMutArg for UnnamespacedRefMut<'view> {}

impl Drop for Unnamespaced {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::namespace_Unnamespaced_destroy(self.inner.as_ptr()) };
    }
}

impl fmt::Debug for Unnamespaced {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Unnamespaced")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for UnnamespacedRef<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("UnnamespacedRef")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for UnnamespacedRefMut<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("UnnamespacedRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl Unnamespaced {
    pub fn make(_e: RenamedAttrEnum) -> crate::Unnamespaced {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::namespace_Unnamespaced_make(_e) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Unnamespaced");
            crate::Unnamespaced {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn use_namespaced(&self, _n: &impl crate::AttrOpaque1RenamedSharedArg) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::namespace_Unnamespaced_use_namespaced(
                self.inner.as_ptr() as *const _,
                crate::private::AttrOpaque1RenamedSharedSealed::__as_const_ptr(_n),
            )
        };
    }
}

impl<'view> UnnamespacedRef<'view> {
    pub fn use_namespaced(&self, _n: &impl crate::AttrOpaque1RenamedSharedArg) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::namespace_Unnamespaced_use_namespaced(
                self.inner.as_ptr() as *const _,
                crate::private::AttrOpaque1RenamedSharedSealed::__as_const_ptr(_n),
            )
        };
    }
}

impl<'view> UnnamespacedRefMut<'view> {
    pub fn use_namespaced(&self, _n: &impl crate::AttrOpaque1RenamedSharedArg) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::namespace_Unnamespaced_use_namespaced(
                self.inner.as_ptr() as *const _,
                crate::private::AttrOpaque1RenamedSharedSealed::__as_const_ptr(_n),
            )
        };
    }
}
