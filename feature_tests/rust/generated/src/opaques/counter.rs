use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

use crate::ffi;
use crate::types::*;

/// Owner-side mutable value. The generated Rust API is expected to expose
/// Counter, CounterRef<'a>, and CounterRefMut<'a>.
pub struct Counter {
    pub(crate) inner: NonNull<ffi::Counter>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct CounterRef<'view> {
    pub(crate) inner: NonNull<ffi::Counter>,
    pub(crate) _borrow: PhantomData<&'view ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

pub struct CounterRefMut<'view> {
    pub(crate) inner: NonNull<ffi::Counter>,
    pub(crate) _borrow: PhantomData<&'view mut ()>,
    pub(crate) _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait CounterSharedArg: crate::private::CounterSharedSealed {}

#[doc(hidden)]
pub trait CounterMutArg: crate::private::CounterMutSealed {}

impl crate::private::CounterSharedSealed for Counter {
    fn __as_const_ptr(&self) -> *const ffi::Counter {
        self.inner.as_ptr()
    }
}
impl crate::private::CounterMutSealed for Counter {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Counter {
        self.inner.as_ptr()
    }
}
impl CounterSharedArg for Counter {}
impl CounterMutArg for Counter {}
impl<'view> crate::private::CounterSharedSealed for CounterRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::Counter {
        self.inner.as_ptr()
    }
}
impl<'view> CounterSharedArg for CounterRef<'view> {}
impl<'view> crate::private::CounterSharedSealed for CounterRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::Counter {
        self.inner.as_ptr()
    }
}
impl<'view> crate::private::CounterMutSealed for CounterRefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Counter {
        self.inner.as_ptr()
    }
}
impl<'view> CounterSharedArg for CounterRefMut<'view> {}
impl<'view> CounterMutArg for CounterRefMut<'view> {}

impl Drop for Counter {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::Counter_destroy(self.inner.as_ptr()) };
    }
}

impl Counter {
    pub fn new() -> super::Counter {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_new() };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Counter");
            super::Counter {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn maybe_new(present: bool) -> Option<super::Counter> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_maybe_new(present) };
        NonNull::new(result as *mut _).map(|inner| super::Counter {
            inner,
            _not_send_sync: PhantomData,
        })
    }
    pub fn get(&self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Counter_get(self.inner.as_ptr() as *const _) }
    }
    pub fn identity(&self) -> usize {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Counter_identity(self.inner.as_ptr() as *const _) }
    }
    pub fn increment(&mut self) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Counter_increment(self.inner.as_ptr()) };
    }
    /// `Option` over a primitive, in the parameter and in the return, gated on
    /// `option`. Lowering is what checks the flag, and it only consults it for
    /// `Option<struct/enum/primitive>` — a nullable owned opaque is a different
    /// lowering path that this flag does not govern.
    pub fn add(&mut self, amount: Option<u32>) -> Option<u32> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result =
            unsafe { ffi::Counter_add(self.inner.as_ptr(), ffi::DiplomatOption::from(amount)) };
        result.into()
    }
    pub fn snapshot(&self) -> Snapshot {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Counter_snapshot(self.inner.as_ptr() as *const _) }
    }
    /// `Option<Struct>`, the struct arm of the same `option` lowering check.
    pub fn maybe_snapshot(&self, present: bool) -> Option<Snapshot> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result =
            unsafe { ffi::Counter_maybe_snapshot(self.inner.as_ptr() as *const _, present) };
        result.into()
    }
    pub fn view<'a>(&'a self) -> super::CounterRef<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_view(self.inner.as_ptr() as *const _) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Counter");
            super::CounterRef {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn view_mut<'a>(&'a mut self) -> super::CounterRefMut<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_view_mut(self.inner.as_ptr()) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Counter");
            super::CounterRefMut {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn same_identity(&self, other: &impl super::CounterSharedArg) -> bool {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::Counter_same_identity(
                self.inner.as_ptr() as *const _,
                crate::private::CounterSharedSealed::__as_const_ptr(other),
            )
        }
    }
    pub fn copy_value_from(&mut self, other: &impl super::CounterSharedArg) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::Counter_copy_value_from(
                self.inner.as_ptr(),
                crate::private::CounterSharedSealed::__as_const_ptr(other),
            )
        }
    }
    pub fn exchange_values(&mut self, other: &mut impl super::CounterMutArg) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::Counter_exchange_values(
                self.inner.as_ptr(),
                crate::private::CounterMutSealed::__as_mut_ptr(other),
            )
        };
    }
    pub fn child<'a>(&'a self) -> super::ChildRef<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_child(self.inner.as_ptr() as *const _) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Child");
            super::ChildRef {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn child_short<'short, 'long: 'short>(&'long self) -> super::ChildRef<'short> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_child_short(self.inner.as_ptr() as *const _) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Child");
            super::ChildRef {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn child_mut<'a>(&'a mut self) -> super::ChildRefMut<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_child_mut(self.inner.as_ptr()) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Child");
            super::ChildRefMut {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn maybe_child<'a>(&'a self, present: bool) -> Option<super::ChildRef<'a>> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_maybe_child(self.inner.as_ptr() as *const _, present) };
        NonNull::new(result as *mut _).map(|inner| super::ChildRef {
            inner,
            _borrow: PhantomData,
            _not_send_sync: PhantomData,
        })
    }
    pub fn reset_drop_count() {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Counter_reset_drop_count() };
    }
    pub fn drop_count() -> usize {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Counter_drop_count() }
    }
    /// An explicit named-constructor name becomes the generated function name.
    ///
    /// The `cfg` guard is load-bearing rather than redundant: `attr(auto, ...)` only
    /// checks support through `BackendAttrSupport::check_string`, whose keys are the
    /// flag names (`named_constructors`), while the attribute path it is called with
    /// is singular (`named_constructor`). That lookup misses, so the `auto` gate is
    /// skipped and this attribute would be applied even with the flag off. Guarding
    /// the method explicitly is what makes `named_constructors` load-bearing here.
    pub fn with_value(value: u32) -> super::Counter {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_new_named(value) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Counter");
            super::Counter {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    /// A plain `#[diplomat::attr(auto, constructor)]`, gated on `constructors`.
    /// Rust lowers this to an ordinary associated function, so the flag's promise is
    /// only that such a method is accepted and emitted at all.
    pub fn from_value(value: u32) -> super::Counter {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_from_value(value) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Counter");
            super::Counter {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    /// A fallible call whose error is a custom enum, gated on `custom_errors`.
    pub fn try_from_value(value: u32) -> Result<super::Counter, ValueError> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_try_from_value(value) };
        match Result::from(result) {
            Ok(result) => Ok({
                let inner = NonNull::new(result as *mut _)
                    .expect("Diplomat ABI returned null for non-null Counter");
                super::Counter {
                    inner,
                    _not_send_sync: PhantomData,
                }
            }),
            Err(result) => Err(result),
        }
    }
    /// A fallible call whose error owns a native allocation, gated on the same flag.
    pub fn take(&mut self, n: u32) -> Result<u32, super::AllocationFailure> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_take(self.inner.as_ptr(), n) };
        match Result::from(result) {
            Ok(result) => Ok(result),
            Err(result) => Err({
                let inner = NonNull::new(result as *mut _)
                    .expect("Diplomat ABI returned null for non-null AllocationFailure");
                super::AllocationFailure {
                    inner,
                    _not_send_sync: PhantomData,
                }
            }),
        }
    }
}

impl<'view> CounterRef<'view> {
    pub fn get(&self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Counter_get(self.inner.as_ptr() as *const _) }
    }
    pub fn identity(&self) -> usize {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Counter_identity(self.inner.as_ptr() as *const _) }
    }
    pub fn snapshot(&self) -> Snapshot {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Counter_snapshot(self.inner.as_ptr() as *const _) }
    }
    /// `Option<Struct>`, the struct arm of the same `option` lowering check.
    pub fn maybe_snapshot(&self, present: bool) -> Option<Snapshot> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result =
            unsafe { ffi::Counter_maybe_snapshot(self.inner.as_ptr() as *const _, present) };
        result.into()
    }
    pub fn view<'a>(&'a self) -> super::CounterRef<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_view(self.inner.as_ptr() as *const _) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Counter");
            super::CounterRef {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn same_identity(&self, other: &impl super::CounterSharedArg) -> bool {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::Counter_same_identity(
                self.inner.as_ptr() as *const _,
                crate::private::CounterSharedSealed::__as_const_ptr(other),
            )
        }
    }
    pub fn child<'a>(&'a self) -> super::ChildRef<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_child(self.inner.as_ptr() as *const _) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Child");
            super::ChildRef {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn child_short<'short, 'long: 'short>(&'long self) -> super::ChildRef<'short> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_child_short(self.inner.as_ptr() as *const _) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Child");
            super::ChildRef {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn maybe_child<'a>(&'a self, present: bool) -> Option<super::ChildRef<'a>> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_maybe_child(self.inner.as_ptr() as *const _, present) };
        NonNull::new(result as *mut _).map(|inner| super::ChildRef {
            inner,
            _borrow: PhantomData,
            _not_send_sync: PhantomData,
        })
    }
}

impl<'view> CounterRefMut<'view> {
    pub fn get(&self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Counter_get(self.inner.as_ptr() as *const _) }
    }
    pub fn identity(&self) -> usize {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Counter_identity(self.inner.as_ptr() as *const _) }
    }
    pub fn increment(&mut self) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Counter_increment(self.inner.as_ptr()) };
    }
    /// `Option` over a primitive, in the parameter and in the return, gated on
    /// `option`. Lowering is what checks the flag, and it only consults it for
    /// `Option<struct/enum/primitive>` — a nullable owned opaque is a different
    /// lowering path that this flag does not govern.
    pub fn add(&mut self, amount: Option<u32>) -> Option<u32> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result =
            unsafe { ffi::Counter_add(self.inner.as_ptr(), ffi::DiplomatOption::from(amount)) };
        result.into()
    }
    pub fn snapshot(&self) -> Snapshot {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Counter_snapshot(self.inner.as_ptr() as *const _) }
    }
    /// `Option<Struct>`, the struct arm of the same `option` lowering check.
    pub fn maybe_snapshot(&self, present: bool) -> Option<Snapshot> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result =
            unsafe { ffi::Counter_maybe_snapshot(self.inner.as_ptr() as *const _, present) };
        result.into()
    }
    pub fn view<'a>(&'a self) -> super::CounterRef<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_view(self.inner.as_ptr() as *const _) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Counter");
            super::CounterRef {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn view_mut<'a>(&'a mut self) -> super::CounterRefMut<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_view_mut(self.inner.as_ptr()) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Counter");
            super::CounterRefMut {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn same_identity(&self, other: &impl super::CounterSharedArg) -> bool {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::Counter_same_identity(
                self.inner.as_ptr() as *const _,
                crate::private::CounterSharedSealed::__as_const_ptr(other),
            )
        }
    }
    pub fn copy_value_from(&mut self, other: &impl super::CounterSharedArg) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::Counter_copy_value_from(
                self.inner.as_ptr(),
                crate::private::CounterSharedSealed::__as_const_ptr(other),
            )
        }
    }
    pub fn exchange_values(&mut self, other: &mut impl super::CounterMutArg) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::Counter_exchange_values(
                self.inner.as_ptr(),
                crate::private::CounterMutSealed::__as_mut_ptr(other),
            )
        };
    }
    pub fn child<'a>(&'a self) -> super::ChildRef<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_child(self.inner.as_ptr() as *const _) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Child");
            super::ChildRef {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn child_short<'short, 'long: 'short>(&'long self) -> super::ChildRef<'short> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_child_short(self.inner.as_ptr() as *const _) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Child");
            super::ChildRef {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn child_mut<'a>(&'a mut self) -> super::ChildRefMut<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_child_mut(self.inner.as_ptr()) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Child");
            super::ChildRefMut {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn maybe_child<'a>(&'a self, present: bool) -> Option<super::ChildRef<'a>> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_maybe_child(self.inner.as_ptr() as *const _, present) };
        NonNull::new(result as *mut _).map(|inner| super::ChildRef {
            inner,
            _borrow: PhantomData,
            _not_send_sync: PhantomData,
        })
    }
    /// A fallible call whose error owns a native allocation, gated on the same flag.
    pub fn take(&mut self, n: u32) -> Result<u32, super::AllocationFailure> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_take(self.inner.as_ptr(), n) };
        match Result::from(result) {
            Ok(result) => Ok(result),
            Err(result) => Err({
                let inner = NonNull::new(result as *mut _)
                    .expect("Diplomat ABI returned null for non-null AllocationFailure");
                super::AllocationFailure {
                    inner,
                    _not_send_sync: PhantomData,
                }
            }),
        }
    }
}
