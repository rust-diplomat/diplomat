use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;

pub struct One<'a> {
    pub(crate) inner: NonNull<ffi::One>,
    pub(crate) _lifetimes: PhantomData<*mut &'a ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct OneRef<'view, 'a> {
    pub(crate) inner: NonNull<ffi::One>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _lifetimes: PhantomData<*mut &'a ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct OneRefMut<'view, 'a> {
    pub(crate) inner: NonNull<ffi::One>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _lifetimes: PhantomData<*mut &'a ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait OneSharedArg: crate::private::OneSharedSealed {}

#[doc(hidden)]
pub trait OneMutArg: crate::private::OneMutSealed {}

impl<'a> crate::private::OneSharedSealed for One<'a> {
    fn __as_const_ptr(&self) -> *const ffi::One {
        self.inner.as_ptr()
    }
}
impl<'a> crate::private::OneMutSealed for One<'a> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::One {
        self.inner.as_ptr()
    }
}
impl<'a> OneSharedArg for One<'a> {}
impl<'a> OneMutArg for One<'a> {}
impl<'view, 'a> crate::private::OneSharedSealed for OneRef<'view, 'a> {
    fn __as_const_ptr(&self) -> *const ffi::One {
        self.inner.as_ptr()
    }
}
impl<'view, 'a> OneSharedArg for OneRef<'view, 'a> {}
impl<'view, 'a> crate::private::OneSharedSealed for OneRefMut<'view, 'a> {
    fn __as_const_ptr(&self) -> *const ffi::One {
        self.inner.as_ptr()
    }
}
impl<'view, 'a> crate::private::OneMutSealed for OneRefMut<'view, 'a> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::One {
        self.inner.as_ptr()
    }
}
impl<'view, 'a> OneSharedArg for OneRefMut<'view, 'a> {}
impl<'view, 'a> OneMutArg for OneRefMut<'view, 'a> {}

impl<'a> Drop for One<'a> {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::One_destroy(self.inner.as_ptr()) };
    }
}

impl<'a> fmt::Debug for One<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("One").field(&self.inner.as_ptr()).finish()
    }
}

impl<'view, 'a> fmt::Debug for OneRef<'view, 'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OneRef").field(&self.inner.as_ptr()).finish()
    }
}

impl<'view, 'a> fmt::Debug for OneRefMut<'view, 'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OneRefMut")
            .field(&self.inner.as_ptr())
            .finish()
    }
}

impl<'o> One<'o> {
    pub fn transitivity<
        'a,
        'b: 'a,
        'c: 'b + 'a,
        'd: 'c + 'b + 'a,
        'e: 'x + 'd + 'c + 'b + 'a,
        'x,
        'anon_0,
    >(
        hold: &'x impl super::OneSharedArg,
        nohold: &'anon_0 impl super::OneSharedArg,
    ) -> super::One<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe {
            ffi::One_transitivity(
                crate::private::OneSharedSealed::__as_const_ptr(hold),
                crate::private::OneSharedSealed::__as_const_ptr(nohold),
            )
        };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null One");
            super::One {
                inner,
                _lifetimes: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn cycle<'a: 'b + 'c, 'b: 'c + 'a, 'c: 'a + 'b, 'x, 'anon_0>(
        hold: &'anon_0 impl super::TwoSharedArg,
        nohold: &'x impl super::OneSharedArg,
    ) -> super::One<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe {
            ffi::One_cycle(
                crate::private::TwoSharedSealed::__as_const_ptr(hold),
                crate::private::OneSharedSealed::__as_const_ptr(nohold),
            )
        };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null One");
            super::One {
                inner,
                _lifetimes: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn many_dependents<
        'a: 'b + 'x,
        'b: 'a + 'x,
        'c: 'a + 'b + 'x,
        'd: 'x + 'b + 'a,
        'x,
        'y: 'x,
        'anon_0,
    >(
        a: &'x impl super::OneSharedArg,
        b: &'b impl super::OneSharedArg,
        c: &'anon_0 impl super::TwoSharedArg,
        d: &'x impl super::TwoSharedArg,
        nohold: &'x impl super::TwoSharedArg,
    ) -> super::One<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe {
            ffi::One_many_dependents(
                crate::private::OneSharedSealed::__as_const_ptr(a),
                crate::private::OneSharedSealed::__as_const_ptr(b),
                crate::private::TwoSharedSealed::__as_const_ptr(c),
                crate::private::TwoSharedSealed::__as_const_ptr(d),
                crate::private::TwoSharedSealed::__as_const_ptr(nohold),
            )
        };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null One");
            super::One {
                inner,
                _lifetimes: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn return_outlives_param<'short, 'long: 'short, 'anon_0>(
        hold: &'anon_0 impl super::TwoSharedArg,
        nohold: &'short impl super::OneSharedArg,
    ) -> super::One<'long> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe {
            ffi::One_return_outlives_param(
                crate::private::TwoSharedSealed::__as_const_ptr(hold),
                crate::private::OneSharedSealed::__as_const_ptr(nohold),
            )
        };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null One");
            super::One {
                inner,
                _lifetimes: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn diamond_top<
        'top,
        'left: 'top,
        'right: 'top,
        'bottom: 'right + 'top + 'left,
        'anon_0,
        'anon_1,
        'anon_2,
        'anon_3,
    >(
        top: &'anon_0 impl super::OneSharedArg,
        left: &'anon_1 impl super::OneSharedArg,
        right: &'anon_2 impl super::OneSharedArg,
        bottom: &'anon_3 impl super::OneSharedArg,
    ) -> super::One<'top> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe {
            ffi::One_diamond_top(
                crate::private::OneSharedSealed::__as_const_ptr(top),
                crate::private::OneSharedSealed::__as_const_ptr(left),
                crate::private::OneSharedSealed::__as_const_ptr(right),
                crate::private::OneSharedSealed::__as_const_ptr(bottom),
            )
        };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null One");
            super::One {
                inner,
                _lifetimes: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn diamond_left<
        'top,
        'left: 'top,
        'right: 'top,
        'bottom: 'right + 'top + 'left,
        'anon_0,
        'anon_1,
        'anon_2,
        'anon_3,
    >(
        top: &'anon_0 impl super::OneSharedArg,
        left: &'anon_1 impl super::OneSharedArg,
        right: &'anon_2 impl super::OneSharedArg,
        bottom: &'anon_3 impl super::OneSharedArg,
    ) -> super::One<'left> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe {
            ffi::One_diamond_left(
                crate::private::OneSharedSealed::__as_const_ptr(top),
                crate::private::OneSharedSealed::__as_const_ptr(left),
                crate::private::OneSharedSealed::__as_const_ptr(right),
                crate::private::OneSharedSealed::__as_const_ptr(bottom),
            )
        };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null One");
            super::One {
                inner,
                _lifetimes: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn diamond_right<
        'top,
        'left: 'top,
        'right: 'top,
        'bottom: 'right + 'top + 'left,
        'anon_0,
        'anon_1,
        'anon_2,
        'anon_3,
    >(
        top: &'anon_0 impl super::OneSharedArg,
        left: &'anon_1 impl super::OneSharedArg,
        right: &'anon_2 impl super::OneSharedArg,
        bottom: &'anon_3 impl super::OneSharedArg,
    ) -> super::One<'right> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe {
            ffi::One_diamond_right(
                crate::private::OneSharedSealed::__as_const_ptr(top),
                crate::private::OneSharedSealed::__as_const_ptr(left),
                crate::private::OneSharedSealed::__as_const_ptr(right),
                crate::private::OneSharedSealed::__as_const_ptr(bottom),
            )
        };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null One");
            super::One {
                inner,
                _lifetimes: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn diamond_bottom<
        'top,
        'left: 'top,
        'right: 'top,
        'bottom: 'right + 'top + 'left,
        'anon_0,
        'anon_1,
        'anon_2,
        'anon_3,
    >(
        top: &'anon_0 impl super::OneSharedArg,
        left: &'anon_1 impl super::OneSharedArg,
        right: &'anon_2 impl super::OneSharedArg,
        bottom: &'anon_3 impl super::OneSharedArg,
    ) -> super::One<'bottom> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe {
            ffi::One_diamond_bottom(
                crate::private::OneSharedSealed::__as_const_ptr(top),
                crate::private::OneSharedSealed::__as_const_ptr(left),
                crate::private::OneSharedSealed::__as_const_ptr(right),
                crate::private::OneSharedSealed::__as_const_ptr(bottom),
            )
        };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null One");
            super::One {
                inner,
                _lifetimes: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn diamond_and_nested_types<
        'a,
        'b: 'y + 'a,
        'c: 'b + 'y + 'a,
        'd: 'c + 'b + 'y + 'a,
        'x,
        'y,
        'anon_0,
        'anon_1,
        'anon_2,
        'anon_3,
    >(
        a: &'anon_0 impl super::OneSharedArg,
        b: &'y impl super::OneSharedArg,
        c: &'anon_1 impl super::OneSharedArg,
        d: &'anon_2 impl super::OneSharedArg,
        nohold: &'anon_3 impl super::OneSharedArg,
    ) -> super::One<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe {
            ffi::One_diamond_and_nested_types(
                crate::private::OneSharedSealed::__as_const_ptr(a),
                crate::private::OneSharedSealed::__as_const_ptr(b),
                crate::private::OneSharedSealed::__as_const_ptr(c),
                crate::private::OneSharedSealed::__as_const_ptr(d),
                crate::private::OneSharedSealed::__as_const_ptr(nohold),
            )
        };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null One");
            super::One {
                inner,
                _lifetimes: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn implicit_bounds<
        'a,
        'b: 'a,
        'c: 'b + 'a,
        'd: 'c + 'b + 'a,
        'x: 'd + 'c + 'b + 'a,
        'y,
        'anon_0,
        'anon_1,
    >(
        explicit_hold: &'d impl super::OneSharedArg,
        implicit_hold: &'anon_0 impl super::OneSharedArg,
        nohold: &'anon_1 impl super::OneSharedArg,
    ) -> super::One<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe {
            ffi::One_implicit_bounds(
                crate::private::OneSharedSealed::__as_const_ptr(explicit_hold),
                crate::private::OneSharedSealed::__as_const_ptr(implicit_hold),
                crate::private::OneSharedSealed::__as_const_ptr(nohold),
            )
        };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null One");
            super::One {
                inner,
                _lifetimes: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn implicit_bounds_deep<'a, 'b: 'a, 'c: 'b + 'a, 'd: 'c + 'b + 'a, 'x>(
        explicit_: &'a impl super::OneSharedArg,
        implicit_1: &'b impl super::OneSharedArg,
        implicit_2: &'c impl super::OneSharedArg,
        nohold: &'x impl super::OneSharedArg,
    ) -> super::One<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe {
            ffi::One_implicit_bounds_deep(
                crate::private::OneSharedSealed::__as_const_ptr(explicit_),
                crate::private::OneSharedSealed::__as_const_ptr(implicit_1),
                crate::private::OneSharedSealed::__as_const_ptr(implicit_2),
                crate::private::OneSharedSealed::__as_const_ptr(nohold),
            )
        };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null One");
            super::One {
                inner,
                _lifetimes: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
}

impl<'view, 'o> OneRef<'view, 'o> {}

impl<'view, 'o> OneRefMut<'view, 'o> {}
