use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

pub struct RefList<'a> {
    pub(crate) inner: NonNull<ffi::RefList>,
    pub(crate) _lifetimes: PhantomData<*mut &'a ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct RefListRef<'view, 'a> {
    pub(crate) inner: NonNull<ffi::RefList>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _lifetimes: PhantomData<*mut &'a ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct RefListRefMut<'view, 'a> {
    pub(crate) inner: NonNull<ffi::RefList>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _lifetimes: PhantomData<*mut &'a ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait RefListSharedArg: crate::private::RefListSharedSealed {}

#[doc(hidden)]
pub trait RefListMutArg: crate::private::RefListMutSealed {}

impl<'a> crate::private::RefListSharedSealed for RefList<'a> {
    fn __as_const_ptr(&self) -> *const ffi::RefList {
        self.inner.as_ptr()
    }
}
impl<'a> crate::private::RefListMutSealed for RefList<'a> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::RefList {
        self.inner.as_ptr()
    }
}
impl<'a> RefListSharedArg for RefList<'a> {}
impl<'a> RefListMutArg for RefList<'a> {}
impl<'view, 'a> crate::private::RefListSharedSealed for RefListRef<'view, 'a> {
    fn __as_const_ptr(&self) -> *const ffi::RefList {
        self.inner.as_ptr()
    }
}
impl<'view, 'a> RefListSharedArg for RefListRef<'view, 'a> {}
impl<'view, 'a> crate::private::RefListSharedSealed for RefListRefMut<'view, 'a> {
    fn __as_const_ptr(&self) -> *const ffi::RefList {
        self.inner.as_ptr()
    }
}
impl<'view, 'a> crate::private::RefListMutSealed for RefListRefMut<'view, 'a> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::RefList {
        self.inner.as_ptr()
    }
}
impl<'view, 'a> RefListSharedArg for RefListRefMut<'view, 'a> {}
impl<'view, 'a> RefListMutArg for RefListRefMut<'view, 'a> {}

impl<'a> Drop for RefList<'a> {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::RefList_destroy(self.inner.as_ptr()) };
    }
}

impl<'a> fmt::Debug for RefList<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RefList")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view, 'a> fmt::Debug for RefListRef<'view, 'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RefListRef")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view, 'a> fmt::Debug for RefListRefMut<'view, 'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RefListRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'b> RefList<'b> {
    pub fn node(data: &'b impl super::RefListParameterSharedArg) -> super::RefList<'b> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe {
            ffi::RefList_node(crate::private::RefListParameterSharedSealed::__as_const_ptr(data))
        };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null RefList");
            super::RefList {
                inner,
                _lifetimes: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
}

impl<'view, 'b> RefListRef<'view, 'b> {}

impl<'view, 'b> RefListRefMut<'view, 'b> {}
