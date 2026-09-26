use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

/// Storing a slice must keep the opaque's lifetime on the input. Eliding it
/// to `&[u8]` would let `Slot<'static>` store a temporary.
pub struct Slot<'a> {
    pub(crate) inner: NonNull<ffi::Slot>,
    pub(crate) _lifetimes: PhantomData<*mut &'a ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct SlotRef<'view, 'a> {
    pub(crate) inner: NonNull<ffi::Slot>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _lifetimes: PhantomData<*mut &'a ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct SlotRefMut<'view, 'a> {
    pub(crate) inner: NonNull<ffi::Slot>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _lifetimes: PhantomData<*mut &'a ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait SlotSharedArg<'a>: crate::private::SlotSharedSealed<'a> {}

#[doc(hidden)]
pub trait SlotMutArg<'a>: crate::private::SlotMutSealed<'a> {}

impl<'a> crate::private::SlotSharedSealed<'a> for Slot<'a> {
    fn __as_const_ptr(&self) -> *const ffi::Slot {
        self.inner.as_ptr()
    }
    fn __type_lifetime(&self) -> core::marker::PhantomData<*mut &'a ()> {
        core::marker::PhantomData
    }
}

impl<'a> crate::private::SlotMutSealed<'a> for Slot<'a> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Slot {
        self.inner.as_ptr()
    }
    fn __type_lifetime(&self) -> core::marker::PhantomData<*mut &'a ()> {
        core::marker::PhantomData
    }
}

impl<'a> SlotSharedArg<'a> for Slot<'a> {}
impl<'a> SlotMutArg<'a> for Slot<'a> {}

impl<'view, 'a> crate::private::SlotSharedSealed<'a> for SlotRef<'view, 'a> {
    fn __as_const_ptr(&self) -> *const ffi::Slot {
        self.inner.as_ptr()
    }
    fn __type_lifetime(&self) -> core::marker::PhantomData<*mut &'a ()> {
        core::marker::PhantomData
    }
}

impl<'view, 'a> SlotSharedArg<'a> for SlotRef<'view, 'a> {}

impl<'view, 'a> crate::private::SlotSharedSealed<'a> for SlotRefMut<'view, 'a> {
    fn __as_const_ptr(&self) -> *const ffi::Slot {
        self.inner.as_ptr()
    }
    fn __type_lifetime(&self) -> core::marker::PhantomData<*mut &'a ()> {
        core::marker::PhantomData
    }
}

impl<'view, 'a> crate::private::SlotMutSealed<'a> for SlotRefMut<'view, 'a> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Slot {
        self.inner.as_ptr()
    }
    fn __type_lifetime(&self) -> core::marker::PhantomData<*mut &'a ()> {
        core::marker::PhantomData
    }
}

impl<'view, 'a> SlotSharedArg<'a> for SlotRefMut<'view, 'a> {}
impl<'view, 'a> SlotMutArg<'a> for SlotRefMut<'view, 'a> {}

impl<'a> Drop for Slot<'a> {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::Slot_destroy(self.inner.as_ptr()) };
    }
}

impl<'a> fmt::Debug for Slot<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Slot").field(&self.inner.as_ptr()).finish()
    }
}

impl<'view, 'a> fmt::Debug for SlotRef<'view, 'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SlotRef")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'view, 'a> fmt::Debug for SlotRefMut<'view, 'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SlotRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'a> Slot<'a> {
    pub fn new(initial: &'a [u8]) -> crate::Slot<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Slot_new(ffi::DiplomatSlice::from(initial)) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Slot");
            crate::Slot {
                inner,
                _lifetimes: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn store(&mut self, value: &'a [u8]) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Slot_store(self.inner.as_ptr(), ffi::DiplomatSlice::from(value)) };
    }
    pub fn get<'anon_0>(&'anon_0 self) -> &'a [u8] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Slot_get(self.inner.as_ptr() as *const _) };
        result.into()
    }
}

impl<'view, 'a> SlotRef<'view, 'a> {
    pub fn get<'anon_0>(&'anon_0 self) -> &'a [u8] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Slot_get(self.inner.as_ptr() as *const _) };
        result.into()
    }
}

impl<'view, 'a> SlotRefMut<'view, 'a> {
    pub fn store(&mut self, value: &'a [u8]) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Slot_store(self.inner.as_ptr(), ffi::DiplomatSlice::from(value)) };
    }
    pub fn get<'anon_0>(&'anon_0 self) -> &'a [u8] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Slot_get(self.inner.as_ptr() as *const _) };
        result.into()
    }
}
