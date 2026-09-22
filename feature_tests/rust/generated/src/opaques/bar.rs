use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

pub struct Bar<'b, 'a: 'b> {
    pub(crate) inner: NonNull<ffi::Bar>,
    pub(crate) _lifetimes: PhantomData<(*mut &'b (), *mut &'a ())>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct BarRef<'view, 'b, 'a: 'b> {
    pub(crate) inner: NonNull<ffi::Bar>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _lifetimes: PhantomData<(*mut &'b (), *mut &'a ())>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct BarRefMut<'view, 'b, 'a: 'b> {
    pub(crate) inner: NonNull<ffi::Bar>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _lifetimes: PhantomData<(*mut &'b (), *mut &'a ())>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait BarSharedArg: crate::private::BarSharedSealed {}

#[doc(hidden)]
pub trait BarMutArg: crate::private::BarMutSealed {}

impl<'b, 'a: 'b> crate::private::BarSharedSealed for Bar<'b, 'a> {
    fn __as_const_ptr(&self) -> *const ffi::Bar {
        self.inner.as_ptr()
    }
}

impl<'b, 'a: 'b> crate::private::BarMutSealed for Bar<'b, 'a> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Bar {
        self.inner.as_ptr()
    }
}

impl<'b, 'a: 'b> BarSharedArg for Bar<'b, 'a> {}
impl<'b, 'a: 'b> BarMutArg for Bar<'b, 'a> {}

impl<'view, 'b, 'a: 'b> crate::private::BarSharedSealed for BarRef<'view, 'b, 'a> {
    fn __as_const_ptr(&self) -> *const ffi::Bar {
        self.inner.as_ptr()
    }
}

impl<'view, 'b, 'a: 'b> BarSharedArg for BarRef<'view, 'b, 'a> {}

impl<'view, 'b, 'a: 'b> crate::private::BarSharedSealed for BarRefMut<'view, 'b, 'a> {
    fn __as_const_ptr(&self) -> *const ffi::Bar {
        self.inner.as_ptr()
    }
}

impl<'view, 'b, 'a: 'b> crate::private::BarMutSealed for BarRefMut<'view, 'b, 'a> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Bar {
        self.inner.as_ptr()
    }
}

impl<'view, 'b, 'a: 'b> BarSharedArg for BarRefMut<'view, 'b, 'a> {}
impl<'view, 'b, 'a: 'b> BarMutArg for BarRefMut<'view, 'b, 'a> {}

impl<'b, 'a: 'b> Drop for Bar<'b, 'a> {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::Bar_destroy(self.inner.as_ptr()) };
    }
}

impl<'b, 'a: 'b> fmt::Debug for Bar<'b, 'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Bar").field(&self.inner.as_ptr()).finish()
    }
}

impl<'view, 'b, 'a: 'b> fmt::Debug for BarRef<'view, 'b, 'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("BarRef").field(&self.inner.as_ptr()).finish()
    }
}

impl<'view, 'b, 'a: 'b> fmt::Debug for BarRefMut<'view, 'b, 'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("BarRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'b, 'a: 'b> Bar<'b, 'a> {
    pub fn foo(&'b self) -> crate::FooRef<'b, 'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Bar_foo(self.inner.as_ptr() as *const _) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Foo");
            crate::FooRef {
                inner,
                _borrow: PhantomData,
                _lifetimes: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
}

impl<'view, 'b, 'a: 'b> BarRef<'view, 'b, 'a> {
    pub fn foo(&'b self) -> crate::FooRef<'b, 'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Bar_foo(self.inner.as_ptr() as *const _) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Foo");
            crate::FooRef {
                inner,
                _borrow: PhantomData,
                _lifetimes: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
}

impl<'view, 'b, 'a: 'b> BarRefMut<'view, 'b, 'a> {
    pub fn foo(&'b self) -> crate::FooRef<'b, 'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Bar_foo(self.inner.as_ptr() as *const _) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Foo");
            crate::FooRef {
                inner,
                _borrow: PhantomData,
                _lifetimes: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
}
