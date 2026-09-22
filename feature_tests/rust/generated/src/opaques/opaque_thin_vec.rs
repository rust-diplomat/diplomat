use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

pub struct OpaqueThinVec {
    pub(crate) inner: NonNull<ffi::OpaqueThinVec>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct OpaqueThinVecRef<'view> {
    pub(crate) inner: NonNull<ffi::OpaqueThinVec>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct OpaqueThinVecRefMut<'view> {
    pub(crate) inner: NonNull<ffi::OpaqueThinVec>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait OpaqueThinVecSharedArg: crate::private::OpaqueThinVecSharedSealed {}

#[doc(hidden)]
pub trait OpaqueThinVecMutArg: crate::private::OpaqueThinVecMutSealed {}

impl crate::private::OpaqueThinVecSharedSealed for OpaqueThinVec {
    fn __as_const_ptr(&self) -> *const ffi::OpaqueThinVec {
        self.inner.as_ptr()
    }
}

impl crate::private::OpaqueThinVecMutSealed for OpaqueThinVec {
    fn __as_mut_ptr(&mut self) -> *mut ffi::OpaqueThinVec {
        self.inner.as_ptr()
    }
}

impl OpaqueThinVecSharedArg for OpaqueThinVec {}
impl OpaqueThinVecMutArg for OpaqueThinVec {}

impl<'view> crate::private::OpaqueThinVecSharedSealed for OpaqueThinVecRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::OpaqueThinVec {
        self.inner.as_ptr()
    }
}

impl<'view> OpaqueThinVecSharedArg for OpaqueThinVecRef<'view> {}

impl<'view> crate::private::OpaqueThinVecSharedSealed for OpaqueThinVecRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::OpaqueThinVec {
        self.inner.as_ptr()
    }
}

impl<'view> crate::private::OpaqueThinVecMutSealed for OpaqueThinVecRefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::OpaqueThinVec {
        self.inner.as_ptr()
    }
}

impl<'view> OpaqueThinVecSharedArg for OpaqueThinVecRefMut<'view> {}
impl<'view> OpaqueThinVecMutArg for OpaqueThinVecRefMut<'view> {}

impl Drop for OpaqueThinVec {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::OpaqueThinVec_destroy(self.inner.as_ptr()) };
    }
}

impl fmt::Debug for OpaqueThinVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OpaqueThinVec")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for OpaqueThinVecRef<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OpaqueThinVecRef")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view> fmt::Debug for OpaqueThinVecRefMut<'view> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OpaqueThinVecRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl OpaqueThinVec {
    pub fn create(a: &[i32], b: &[f32], c: &[u8]) -> crate::OpaqueThinVec {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe {
            ffi::OpaqueThinVec_create(
                ffi::DiplomatSlice::from(a),
                ffi::DiplomatSlice::from(b),
                ffi::DiplomatSlice::from(c),
            )
        };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null OpaqueThinVec");
            crate::OpaqueThinVec {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn iter<'a>(&'a self) -> crate::OpaqueThinIter<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OpaqueThinVec_iter(self.inner.as_ptr() as *const _) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null OpaqueThinIter");
            crate::OpaqueThinIter {
                inner,
                _lifetimes: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn len(&self) -> usize {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::OpaqueThinVec_len(self.inner.as_ptr() as *const _) }
    }
    pub fn get<'a>(&'a self, idx: usize) -> Option<crate::OpaqueThinRef<'a>> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OpaqueThinVec_get(self.inner.as_ptr() as *const _, idx) };
        NonNull::new(result as *mut _).map(|inner| crate::OpaqueThinRef {
            inner,
            _borrow: PhantomData,
            _not_send_sync: PhantomData,
        })
    }
    pub fn first<'a>(&'a self) -> Option<crate::OpaqueThinRef<'a>> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OpaqueThinVec_first(self.inner.as_ptr() as *const _) };
        NonNull::new(result as *mut _).map(|inner| crate::OpaqueThinRef {
            inner,
            _borrow: PhantomData,
            _not_send_sync: PhantomData,
        })
    }
}

impl<'view> OpaqueThinVecRef<'view> {
    pub fn iter<'a>(&'a self) -> crate::OpaqueThinIter<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OpaqueThinVec_iter(self.inner.as_ptr() as *const _) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null OpaqueThinIter");
            crate::OpaqueThinIter {
                inner,
                _lifetimes: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn len(&self) -> usize {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::OpaqueThinVec_len(self.inner.as_ptr() as *const _) }
    }
    pub fn get<'a>(&'a self, idx: usize) -> Option<crate::OpaqueThinRef<'a>> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OpaqueThinVec_get(self.inner.as_ptr() as *const _, idx) };
        NonNull::new(result as *mut _).map(|inner| crate::OpaqueThinRef {
            inner,
            _borrow: PhantomData,
            _not_send_sync: PhantomData,
        })
    }
    pub fn first<'a>(&'a self) -> Option<crate::OpaqueThinRef<'a>> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OpaqueThinVec_first(self.inner.as_ptr() as *const _) };
        NonNull::new(result as *mut _).map(|inner| crate::OpaqueThinRef {
            inner,
            _borrow: PhantomData,
            _not_send_sync: PhantomData,
        })
    }
}

impl<'view> OpaqueThinVecRefMut<'view> {
    pub fn iter<'a>(&'a self) -> crate::OpaqueThinIter<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OpaqueThinVec_iter(self.inner.as_ptr() as *const _) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null OpaqueThinIter");
            crate::OpaqueThinIter {
                inner,
                _lifetimes: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn len(&self) -> usize {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::OpaqueThinVec_len(self.inner.as_ptr() as *const _) }
    }
    pub fn get<'a>(&'a self, idx: usize) -> Option<crate::OpaqueThinRef<'a>> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OpaqueThinVec_get(self.inner.as_ptr() as *const _, idx) };
        NonNull::new(result as *mut _).map(|inner| crate::OpaqueThinRef {
            inner,
            _borrow: PhantomData,
            _not_send_sync: PhantomData,
        })
    }
    pub fn first<'a>(&'a self) -> Option<crate::OpaqueThinRef<'a>> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::OpaqueThinVec_first(self.inner.as_ptr() as *const _) };
        NonNull::new(result as *mut _).map(|inner| crate::OpaqueThinRef {
            inner,
            _borrow: PhantomData,
            _not_send_sync: PhantomData,
        })
    }
}
