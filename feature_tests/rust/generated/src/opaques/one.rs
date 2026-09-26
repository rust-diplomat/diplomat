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
pub trait OneSharedArg<'a>: crate::private::OneSharedSealed<'a> {}

#[doc(hidden)]
pub trait OneMutArg<'a>: crate::private::OneMutSealed<'a> {}

impl<'a> crate::private::OneSharedSealed<'a> for One<'a> {
    fn __as_const_ptr(&self) -> *const ffi::One {
        self.inner.as_ptr()
    }
    fn __type_lifetime(&self) -> core::marker::PhantomData<*mut &'a ()> {
        core::marker::PhantomData
    }
}

impl<'a> crate::private::OneMutSealed<'a> for One<'a> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::One {
        self.inner.as_ptr()
    }
    fn __type_lifetime(&self) -> core::marker::PhantomData<*mut &'a ()> {
        core::marker::PhantomData
    }
}

impl<'a> OneSharedArg<'a> for One<'a> {}
impl<'a> OneMutArg<'a> for One<'a> {}

impl<'view, 'a> crate::private::OneSharedSealed<'a> for OneRef<'view, 'a> {
    fn __as_const_ptr(&self) -> *const ffi::One {
        self.inner.as_ptr()
    }
    fn __type_lifetime(&self) -> core::marker::PhantomData<*mut &'a ()> {
        core::marker::PhantomData
    }
}

impl<'view, 'a> OneSharedArg<'a> for OneRef<'view, 'a> {}

impl<'view, 'a> crate::private::OneSharedSealed<'a> for OneRefMut<'view, 'a> {
    fn __as_const_ptr(&self) -> *const ffi::One {
        self.inner.as_ptr()
    }
    fn __type_lifetime(&self) -> core::marker::PhantomData<*mut &'a ()> {
        core::marker::PhantomData
    }
}

impl<'view, 'a> crate::private::OneMutSealed<'a> for OneRefMut<'view, 'a> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::One {
        self.inner.as_ptr()
    }
    fn __type_lifetime(&self) -> core::marker::PhantomData<*mut &'a ()> {
        core::marker::PhantomData
    }
}

impl<'view, 'a> OneSharedArg<'a> for OneRefMut<'view, 'a> {}
impl<'view, 'a> OneMutArg<'a> for OneRefMut<'view, 'a> {}

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
        hold: &'x impl crate::OneSharedArg<'e>,
        nohold: &'anon_0 impl crate::OneSharedArg<'x>,
    ) -> crate::One<'a> {
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
            crate::One {
                inner,
                _lifetimes: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn cycle<'a: 'b + 'c, 'b: 'c + 'a, 'c: 'a + 'b, 'x, 'anon_0>(
        hold: &'anon_0 impl crate::TwoSharedArg<'x, 'b>,
        nohold: &'x impl crate::OneSharedArg<'x>,
    ) -> crate::One<'a> {
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
            crate::One {
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
        a: &'x impl crate::OneSharedArg<'a>,
        b: &'b impl crate::OneSharedArg<'a>,
        c: &'anon_0 impl crate::TwoSharedArg<'x, 'c>,
        d: &'x impl crate::TwoSharedArg<'d, 'y>,
        nohold: &'x impl crate::TwoSharedArg<'x, 'y>,
    ) -> crate::One<'a> {
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
            crate::One {
                inner,
                _lifetimes: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn return_outlives_param<'short, 'long: 'short, 'anon_0>(
        hold: &'anon_0 impl crate::TwoSharedArg<'long, 'short>,
        nohold: &'short impl crate::OneSharedArg<'short>,
    ) -> crate::One<'long> {
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
            crate::One {
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
        top: &'anon_0 impl crate::OneSharedArg<'top>,
        left: &'anon_1 impl crate::OneSharedArg<'left>,
        right: &'anon_2 impl crate::OneSharedArg<'right>,
        bottom: &'anon_3 impl crate::OneSharedArg<'bottom>,
    ) -> crate::One<'top> {
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
            crate::One {
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
        top: &'anon_0 impl crate::OneSharedArg<'top>,
        left: &'anon_1 impl crate::OneSharedArg<'left>,
        right: &'anon_2 impl crate::OneSharedArg<'right>,
        bottom: &'anon_3 impl crate::OneSharedArg<'bottom>,
    ) -> crate::One<'left> {
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
            crate::One {
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
        top: &'anon_0 impl crate::OneSharedArg<'top>,
        left: &'anon_1 impl crate::OneSharedArg<'left>,
        right: &'anon_2 impl crate::OneSharedArg<'right>,
        bottom: &'anon_3 impl crate::OneSharedArg<'bottom>,
    ) -> crate::One<'right> {
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
            crate::One {
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
        top: &'anon_0 impl crate::OneSharedArg<'top>,
        left: &'anon_1 impl crate::OneSharedArg<'left>,
        right: &'anon_2 impl crate::OneSharedArg<'right>,
        bottom: &'anon_3 impl crate::OneSharedArg<'bottom>,
    ) -> crate::One<'bottom> {
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
            crate::One {
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
        a: &'anon_0 impl crate::OneSharedArg<'a>,
        b: &'y impl crate::OneSharedArg<'b>,
        c: &'anon_1 impl crate::OneSharedArg<'c>,
        d: &'anon_2 impl crate::OneSharedArg<'d>,
        nohold: &'anon_3 impl crate::OneSharedArg<'x>,
    ) -> crate::One<'a> {
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
            crate::One {
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
        explicit_hold: &'d impl crate::OneSharedArg<'x>,
        implicit_hold: &'anon_0 impl crate::OneSharedArg<'x>,
        nohold: &'anon_1 impl crate::OneSharedArg<'y>,
    ) -> crate::One<'a> {
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
            crate::One {
                inner,
                _lifetimes: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn implicit_bounds_deep<'a, 'b: 'a, 'c: 'b + 'a, 'd: 'c + 'b + 'a, 'x>(
        explicit_: &'a impl crate::OneSharedArg<'b>,
        implicit_1: &'b impl crate::OneSharedArg<'c>,
        implicit_2: &'c impl crate::OneSharedArg<'d>,
        nohold: &'x impl crate::OneSharedArg<'x>,
    ) -> crate::One<'a> {
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
            crate::One {
                inner,
                _lifetimes: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
}
