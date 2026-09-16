//! Safe Rust bindings generated over a Diplomat native ABI.

#![allow(clippy::needless_lifetimes)]
#![allow(clippy::new_without_default)]

mod ffi;

use core::marker::PhantomData;
use core::ptr::NonNull;
use std::rc::Rc;

mod private {
    pub trait BytesSharedSealed {
        fn __as_const_ptr(&self) -> *const crate::ffi::Bytes;
    }
    pub trait BytesMutSealed: BytesSharedSealed {
        fn __as_mut_ptr(&mut self) -> *mut crate::ffi::Bytes;
    }
    pub trait ChildSharedSealed {
        fn __as_const_ptr(&self) -> *const crate::ffi::Child;
    }
    pub trait ChildMutSealed: ChildSharedSealed {
        fn __as_mut_ptr(&mut self) -> *mut crate::ffi::Child;
    }
    pub trait CounterSharedSealed {
        fn __as_const_ptr(&self) -> *const crate::ffi::Counter;
    }
    pub trait CounterMutSealed: CounterSharedSealed {
        fn __as_mut_ptr(&mut self) -> *mut crate::ffi::Counter;
    }
    pub trait MessageSharedSealed {
        fn __as_const_ptr(&self) -> *const crate::ffi::Message;
    }
    pub trait MessageMutSealed: MessageSharedSealed {
        fn __as_mut_ptr(&mut self) -> *mut crate::ffi::Message;
    }
    pub trait NumbersSharedSealed {
        fn __as_const_ptr(&self) -> *const crate::ffi::Numbers;
    }
    pub trait NumbersMutSealed: NumbersSharedSealed {
        fn __as_mut_ptr(&mut self) -> *mut crate::ffi::Numbers;
    }
    pub trait SliceViewSharedSealed {
        fn __as_const_ptr(&self) -> *const crate::ffi::SliceView;
    }
    pub trait SliceViewMutSealed: SliceViewSharedSealed {
        fn __as_mut_ptr(&mut self) -> *mut crate::ffi::SliceView;
    }
    /// Reconstruct a shared slice from a provider-returned pointer/length pair.
    ///
    /// # Safety
    ///
    /// The caller must uphold the provider's validity, alignment, aliasing, and
    /// lifetime contract for `ptr`/`len` for the returned lifetime.
    pub(super) unsafe fn slice_from_raw_parts<'a, T>(ptr: *const T, len: usize) -> &'a [T] {
        if ptr.is_null() {
            debug_assert_eq!(len, 0, "provider returned a null slice with nonzero length");
            return &[];
        }
        core::slice::from_raw_parts(ptr, len)
    }

    /// Reconstruct an exclusive slice from a provider-returned pointer/length pair.
    ///
    /// # Safety
    ///
    /// The caller must uphold the provider's validity, alignment, uniqueness, and
    /// lifetime contract for `ptr`/`len` for the returned lifetime.
    pub(super) unsafe fn slice_from_raw_parts_mut<'a, T>(ptr: *mut T, len: usize) -> &'a mut [T] {
        if ptr.is_null() {
            debug_assert_eq!(len, 0, "provider returned a null slice with nonzero length");
            return &mut [];
        }
        core::slice::from_raw_parts_mut(ptr, len)
    }

    /// Reconstruct a validated `&str` from a provider-returned pointer/length pair.
    ///
    /// # Safety
    ///
    /// The caller must uphold the provider's validity, alignment, aliasing, UTF-8,
    /// and lifetime contract for `ptr`/`len` for the returned lifetime.
    pub(super) unsafe fn str_from_raw_parts<'a>(ptr: *const u8, len: usize) -> &'a str {
        core::str::from_utf8_unchecked(slice_from_raw_parts(ptr, len))
    }

    /// Take ownership of a provider-allocated `Box<[T]>` returned across the ABI.
    ///
    /// # Safety
    ///
    /// `ptr`/`len` must describe a `Box<[T]>` allocated by the provider, and the
    /// provider and consumer must share an allocator (Diplomat's owned-slice
    /// contract). Ownership transfers to the returned `Box`.
    pub(super) unsafe fn owned_slice_into_box<T>(ptr: *mut T, len: usize) -> Box<[T]> {
        if ptr.is_null() {
            debug_assert_eq!(len, 0, "provider returned a null slice with nonzero length");
            return Box::new([]);
        }
        Box::from_raw(core::ptr::slice_from_raw_parts_mut(ptr, len))
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Idle = 0,
    Active = 1,
}

/// Value struct with borrowed slice fields, returned by `SliceView::fields`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldView<'a> {
    pub bytes: &'a [u8],
    pub count: u32,
    _lifetimes: PhantomData<fn(&'a ()) -> &'a ()>,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Snapshot {
    pub value: u32,
    pub active: bool,
    pub mode: Mode,
}

/// Owned `Box<[u8]>` returns: the provider allocates and hands ownership across.
pub struct Bytes {
    inner: NonNull<ffi::Bytes>,
    _not_send_sync: PhantomData<Rc<()>>,
}

pub struct BytesRef<'view> {
    inner: NonNull<ffi::Bytes>,
    _borrow: PhantomData<&'view ()>,
    _not_send_sync: PhantomData<Rc<()>>,
}

pub struct BytesRefMut<'view> {
    inner: NonNull<ffi::Bytes>,
    _borrow: PhantomData<&'view mut ()>,
    _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait BytesSharedArg: private::BytesSharedSealed {}

#[doc(hidden)]
pub trait BytesMutArg: private::BytesMutSealed {}

impl private::BytesSharedSealed for Bytes {
    fn __as_const_ptr(&self) -> *const ffi::Bytes {
        self.inner.as_ptr()
    }
}
impl private::BytesMutSealed for Bytes {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Bytes {
        self.inner.as_ptr()
    }
}
impl BytesSharedArg for Bytes {}
impl BytesMutArg for Bytes {}
impl<'view> private::BytesSharedSealed for BytesRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::Bytes {
        self.inner.as_ptr()
    }
}
impl<'view> BytesSharedArg for BytesRef<'view> {}
impl<'view> private::BytesSharedSealed for BytesRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::Bytes {
        self.inner.as_ptr()
    }
}
impl<'view> private::BytesMutSealed for BytesRefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Bytes {
        self.inner.as_ptr()
    }
}
impl<'view> BytesSharedArg for BytesRefMut<'view> {}
impl<'view> BytesMutArg for BytesRefMut<'view> {}

impl Drop for Bytes {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::Bytes_destroy(self.inner.as_ptr()) };
    }
}

impl Bytes {
    pub fn make(len: u32) -> Box<[u8]> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Bytes_make(len) };
        unsafe { private::owned_slice_into_box(result.ptr, result.len) }
    }
    pub fn join(a: &[u8], b: &[u8]) -> Box<[u8]> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe {
            ffi::Bytes_join(
                ffi::DiplomatSlice::<u8> {
                    ptr: a.as_ptr(),
                    len: a.len(),
                },
                ffi::DiplomatSlice::<u8> {
                    ptr: b.as_ptr(),
                    len: b.len(),
                },
            )
        };
        unsafe { private::owned_slice_into_box(result.ptr, result.len) }
    }
}

impl<'view> BytesRef<'view> {}

impl<'view> BytesRefMut<'view> {}

/// A distinct owner-side opaque returned only as a borrow from Counter.
pub struct Child {
    inner: NonNull<ffi::Child>,
    _not_send_sync: PhantomData<Rc<()>>,
}

pub struct ChildRef<'view> {
    inner: NonNull<ffi::Child>,
    _borrow: PhantomData<&'view ()>,
    _not_send_sync: PhantomData<Rc<()>>,
}

pub struct ChildRefMut<'view> {
    inner: NonNull<ffi::Child>,
    _borrow: PhantomData<&'view mut ()>,
    _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait ChildSharedArg: private::ChildSharedSealed {}

#[doc(hidden)]
pub trait ChildMutArg: private::ChildMutSealed {}

impl private::ChildSharedSealed for Child {
    fn __as_const_ptr(&self) -> *const ffi::Child {
        self.inner.as_ptr()
    }
}
impl private::ChildMutSealed for Child {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Child {
        self.inner.as_ptr()
    }
}
impl ChildSharedArg for Child {}
impl ChildMutArg for Child {}
impl<'view> private::ChildSharedSealed for ChildRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::Child {
        self.inner.as_ptr()
    }
}
impl<'view> ChildSharedArg for ChildRef<'view> {}
impl<'view> private::ChildSharedSealed for ChildRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::Child {
        self.inner.as_ptr()
    }
}
impl<'view> private::ChildMutSealed for ChildRefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Child {
        self.inner.as_ptr()
    }
}
impl<'view> ChildSharedArg for ChildRefMut<'view> {}
impl<'view> ChildMutArg for ChildRefMut<'view> {}

impl Drop for Child {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::Child_destroy(self.inner.as_ptr()) };
    }
}

impl Child {
    pub fn get(&self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Child_get(self.inner.as_ptr() as *const _) }
    }
    pub fn set(&mut self, value: u32) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Child_set(self.inner.as_ptr(), value) };
    }
    pub fn owner_identity(&self) -> usize {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Child_owner_identity(self.inner.as_ptr() as *const _) }
    }
}

impl<'view> ChildRef<'view> {
    pub fn get(&self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Child_get(self.inner.as_ptr() as *const _) }
    }
    pub fn owner_identity(&self) -> usize {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Child_owner_identity(self.inner.as_ptr() as *const _) }
    }
}

impl<'view> ChildRefMut<'view> {
    pub fn get(&self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Child_get(self.inner.as_ptr() as *const _) }
    }
    pub fn set(&mut self, value: u32) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Child_set(self.inner.as_ptr(), value) };
    }
    pub fn owner_identity(&self) -> usize {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Child_owner_identity(self.inner.as_ptr() as *const _) }
    }
}

/// Owner-side mutable value. The generated Rust API is expected to expose
/// Counter, CounterRef<'a>, and CounterRefMut<'a>.
pub struct Counter {
    inner: NonNull<ffi::Counter>,
    _not_send_sync: PhantomData<Rc<()>>,
}

pub struct CounterRef<'view> {
    inner: NonNull<ffi::Counter>,
    _borrow: PhantomData<&'view ()>,
    _not_send_sync: PhantomData<Rc<()>>,
}

pub struct CounterRefMut<'view> {
    inner: NonNull<ffi::Counter>,
    _borrow: PhantomData<&'view mut ()>,
    _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait CounterSharedArg: private::CounterSharedSealed {}

#[doc(hidden)]
pub trait CounterMutArg: private::CounterMutSealed {}

impl private::CounterSharedSealed for Counter {
    fn __as_const_ptr(&self) -> *const ffi::Counter {
        self.inner.as_ptr()
    }
}
impl private::CounterMutSealed for Counter {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Counter {
        self.inner.as_ptr()
    }
}
impl CounterSharedArg for Counter {}
impl CounterMutArg for Counter {}
impl<'view> private::CounterSharedSealed for CounterRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::Counter {
        self.inner.as_ptr()
    }
}
impl<'view> CounterSharedArg for CounterRef<'view> {}
impl<'view> private::CounterSharedSealed for CounterRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::Counter {
        self.inner.as_ptr()
    }
}
impl<'view> private::CounterMutSealed for CounterRefMut<'view> {
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
    pub fn new() -> Counter {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_new() };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Counter");
            Counter {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn maybe_new(present: bool) -> Option<Counter> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_maybe_new(present) };
        NonNull::new(result as *mut _).map(|inner| Counter {
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
    pub fn add(&mut self, amount: Option<u32>) -> Option<u32> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe {
            ffi::Counter_add(
                self.inner.as_ptr(),
                ffi::DiplomatOption::from_option(amount),
            )
        };
        unsafe { result.into_option() }
    }
    pub fn snapshot(&self) -> Snapshot {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Counter_snapshot(self.inner.as_ptr() as *const _) }
    }
    pub fn maybe_snapshot(&self, present: bool) -> Option<Snapshot> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result =
            unsafe { ffi::Counter_maybe_snapshot(self.inner.as_ptr() as *const _, present) };
        unsafe { result.into_option() }
    }
    pub fn view<'a>(&'a self) -> CounterRef<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_view(self.inner.as_ptr() as *const _) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Counter");
            CounterRef {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn view_mut<'a>(&'a mut self) -> CounterRefMut<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_view_mut(self.inner.as_ptr()) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Counter");
            CounterRefMut {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn same_identity(&self, other: &impl CounterSharedArg) -> bool {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::Counter_same_identity(
                self.inner.as_ptr() as *const _,
                private::CounterSharedSealed::__as_const_ptr(other),
            )
        }
    }
    pub fn copy_value_from(&mut self, other: &impl CounterSharedArg) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::Counter_copy_value_from(
                self.inner.as_ptr(),
                private::CounterSharedSealed::__as_const_ptr(other),
            )
        }
    }
    pub fn exchange_values(&mut self, other: &mut impl CounterMutArg) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::Counter_exchange_values(
                self.inner.as_ptr(),
                private::CounterMutSealed::__as_mut_ptr(other),
            )
        };
    }
    pub fn child<'a>(&'a self) -> ChildRef<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_child(self.inner.as_ptr() as *const _) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Child");
            ChildRef {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn child_short<'short, 'long: 'short>(&'long self) -> ChildRef<'short> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_child_short(self.inner.as_ptr() as *const _) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Child");
            ChildRef {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn child_mut<'a>(&'a mut self) -> ChildRefMut<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_child_mut(self.inner.as_ptr()) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Child");
            ChildRefMut {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn maybe_child<'a>(&'a self, present: bool) -> Option<ChildRef<'a>> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_maybe_child(self.inner.as_ptr() as *const _, present) };
        NonNull::new(result as *mut _).map(|inner| ChildRef {
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
    pub fn maybe_snapshot(&self, present: bool) -> Option<Snapshot> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result =
            unsafe { ffi::Counter_maybe_snapshot(self.inner.as_ptr() as *const _, present) };
        unsafe { result.into_option() }
    }
    pub fn view<'a>(&'a self) -> CounterRef<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_view(self.inner.as_ptr() as *const _) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Counter");
            CounterRef {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn same_identity(&self, other: &impl CounterSharedArg) -> bool {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::Counter_same_identity(
                self.inner.as_ptr() as *const _,
                private::CounterSharedSealed::__as_const_ptr(other),
            )
        }
    }
    pub fn child<'a>(&'a self) -> ChildRef<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_child(self.inner.as_ptr() as *const _) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Child");
            ChildRef {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn child_short<'short, 'long: 'short>(&'long self) -> ChildRef<'short> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_child_short(self.inner.as_ptr() as *const _) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Child");
            ChildRef {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn maybe_child<'a>(&'a self, present: bool) -> Option<ChildRef<'a>> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_maybe_child(self.inner.as_ptr() as *const _, present) };
        NonNull::new(result as *mut _).map(|inner| ChildRef {
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
    pub fn add(&mut self, amount: Option<u32>) -> Option<u32> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe {
            ffi::Counter_add(
                self.inner.as_ptr(),
                ffi::DiplomatOption::from_option(amount),
            )
        };
        unsafe { result.into_option() }
    }
    pub fn snapshot(&self) -> Snapshot {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Counter_snapshot(self.inner.as_ptr() as *const _) }
    }
    pub fn maybe_snapshot(&self, present: bool) -> Option<Snapshot> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result =
            unsafe { ffi::Counter_maybe_snapshot(self.inner.as_ptr() as *const _, present) };
        unsafe { result.into_option() }
    }
    pub fn view<'a>(&'a self) -> CounterRef<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_view(self.inner.as_ptr() as *const _) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Counter");
            CounterRef {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn view_mut<'a>(&'a mut self) -> CounterRefMut<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_view_mut(self.inner.as_ptr()) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Counter");
            CounterRefMut {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn same_identity(&self, other: &impl CounterSharedArg) -> bool {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::Counter_same_identity(
                self.inner.as_ptr() as *const _,
                private::CounterSharedSealed::__as_const_ptr(other),
            )
        }
    }
    pub fn copy_value_from(&mut self, other: &impl CounterSharedArg) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::Counter_copy_value_from(
                self.inner.as_ptr(),
                private::CounterSharedSealed::__as_const_ptr(other),
            )
        }
    }
    pub fn exchange_values(&mut self, other: &mut impl CounterMutArg) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::Counter_exchange_values(
                self.inner.as_ptr(),
                private::CounterMutSealed::__as_mut_ptr(other),
            )
        };
    }
    pub fn child<'a>(&'a self) -> ChildRef<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_child(self.inner.as_ptr() as *const _) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Child");
            ChildRef {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn child_short<'short, 'long: 'short>(&'long self) -> ChildRef<'short> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_child_short(self.inner.as_ptr() as *const _) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Child");
            ChildRef {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn child_mut<'a>(&'a mut self) -> ChildRefMut<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_child_mut(self.inner.as_ptr()) };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Child");
            ChildRefMut {
                inner,
                _borrow: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn maybe_child<'a>(&'a self, present: bool) -> Option<ChildRef<'a>> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Counter_maybe_child(self.inner.as_ptr() as *const _, present) };
        NonNull::new(result as *mut _).map(|inner| ChildRef {
            inner,
            _borrow: PhantomData,
            _not_send_sync: PhantomData,
        })
    }
}

/// Opaque owning a string, exercising `DiplomatStr`/`&str` borrows.
pub struct Message {
    inner: NonNull<ffi::Message>,
    _not_send_sync: PhantomData<Rc<()>>,
}

pub struct MessageRef<'view> {
    inner: NonNull<ffi::Message>,
    _borrow: PhantomData<&'view ()>,
    _not_send_sync: PhantomData<Rc<()>>,
}

pub struct MessageRefMut<'view> {
    inner: NonNull<ffi::Message>,
    _borrow: PhantomData<&'view mut ()>,
    _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait MessageSharedArg: private::MessageSharedSealed {}

#[doc(hidden)]
pub trait MessageMutArg: private::MessageMutSealed {}

impl private::MessageSharedSealed for Message {
    fn __as_const_ptr(&self) -> *const ffi::Message {
        self.inner.as_ptr()
    }
}
impl private::MessageMutSealed for Message {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Message {
        self.inner.as_ptr()
    }
}
impl MessageSharedArg for Message {}
impl MessageMutArg for Message {}
impl<'view> private::MessageSharedSealed for MessageRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::Message {
        self.inner.as_ptr()
    }
}
impl<'view> MessageSharedArg for MessageRef<'view> {}
impl<'view> private::MessageSharedSealed for MessageRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::Message {
        self.inner.as_ptr()
    }
}
impl<'view> private::MessageMutSealed for MessageRefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Message {
        self.inner.as_ptr()
    }
}
impl<'view> MessageSharedArg for MessageRefMut<'view> {}
impl<'view> MessageMutArg for MessageRefMut<'view> {}

impl Drop for Message {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::Message_destroy(self.inner.as_ptr()) };
    }
}

impl Message {
    pub fn new(v: &[u8]) -> Message {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe {
            ffi::Message_new(ffi::DiplomatSlice::<u8> {
                ptr: v.as_ptr(),
                len: v.len(),
            })
        };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Message");
            Message {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn bytes<'a>(&'a self) -> &'a [u8] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Message_bytes(self.inner.as_ptr() as *const _) };
        unsafe { private::slice_from_raw_parts(result.ptr, result.len) }
    }
    pub fn text<'a>(&'a self) -> &'a str {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Message_text(self.inner.as_ptr() as *const _) };
        unsafe { private::str_from_raw_parts(result.ptr, result.len) }
    }
}

impl<'view> MessageRef<'view> {
    pub fn bytes<'a>(&'a self) -> &'a [u8] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Message_bytes(self.inner.as_ptr() as *const _) };
        unsafe { private::slice_from_raw_parts(result.ptr, result.len) }
    }
    pub fn text<'a>(&'a self) -> &'a str {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Message_text(self.inner.as_ptr() as *const _) };
        unsafe { private::str_from_raw_parts(result.ptr, result.len) }
    }
}

impl<'view> MessageRefMut<'view> {
    pub fn bytes<'a>(&'a self) -> &'a [u8] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Message_bytes(self.inner.as_ptr() as *const _) };
        unsafe { private::slice_from_raw_parts(result.ptr, result.len) }
    }
    pub fn text<'a>(&'a self) -> &'a str {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Message_text(self.inner.as_ptr() as *const _) };
        unsafe { private::str_from_raw_parts(result.ptr, result.len) }
    }
}

/// Opaque with a primitive slice, exercising borrowed slice inputs/outputs.
pub struct Numbers {
    inner: NonNull<ffi::Numbers>,
    _not_send_sync: PhantomData<Rc<()>>,
}

pub struct NumbersRef<'view> {
    inner: NonNull<ffi::Numbers>,
    _borrow: PhantomData<&'view ()>,
    _not_send_sync: PhantomData<Rc<()>>,
}

pub struct NumbersRefMut<'view> {
    inner: NonNull<ffi::Numbers>,
    _borrow: PhantomData<&'view mut ()>,
    _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait NumbersSharedArg: private::NumbersSharedSealed {}

#[doc(hidden)]
pub trait NumbersMutArg: private::NumbersMutSealed {}

impl private::NumbersSharedSealed for Numbers {
    fn __as_const_ptr(&self) -> *const ffi::Numbers {
        self.inner.as_ptr()
    }
}
impl private::NumbersMutSealed for Numbers {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Numbers {
        self.inner.as_ptr()
    }
}
impl NumbersSharedArg for Numbers {}
impl NumbersMutArg for Numbers {}
impl<'view> private::NumbersSharedSealed for NumbersRef<'view> {
    fn __as_const_ptr(&self) -> *const ffi::Numbers {
        self.inner.as_ptr()
    }
}
impl<'view> NumbersSharedArg for NumbersRef<'view> {}
impl<'view> private::NumbersSharedSealed for NumbersRefMut<'view> {
    fn __as_const_ptr(&self) -> *const ffi::Numbers {
        self.inner.as_ptr()
    }
}
impl<'view> private::NumbersMutSealed for NumbersRefMut<'view> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::Numbers {
        self.inner.as_ptr()
    }
}
impl<'view> NumbersSharedArg for NumbersRefMut<'view> {}
impl<'view> NumbersMutArg for NumbersRefMut<'view> {}

impl Drop for Numbers {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::Numbers_destroy(self.inner.as_ptr()) };
    }
}

impl Numbers {
    pub fn new(values: &[u32]) -> Numbers {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe {
            ffi::Numbers_new(ffi::DiplomatSlice::<u32> {
                ptr: values.as_ptr(),
                len: values.len(),
            })
        };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null Numbers");
            Numbers {
                inner,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn values<'a>(&'a self) -> &'a [u32] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Numbers_values(self.inner.as_ptr() as *const _) };
        unsafe { private::slice_from_raw_parts(result.ptr, result.len) }
    }
    pub fn values_mut<'a>(&'a mut self) -> &'a mut [u32] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Numbers_values_mut(self.inner.as_ptr()) };
        unsafe { private::slice_from_raw_parts_mut(result.ptr, result.len) }
    }
    pub fn sum(&self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Numbers_sum(self.inner.as_ptr() as *const _) }
    }
    pub fn total<'a>(fields: FieldView<'a>) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::Numbers_total(ffi::FieldView {
                bytes: ffi::DiplomatSlice::<u8> {
                    ptr: fields.bytes.as_ptr(),
                    len: fields.bytes.len(),
                },
                count: fields.count,
            })
        }
    }
    pub fn fill(&self, out: &mut [u32]) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::Numbers_fill(
                self.inner.as_ptr() as *const _,
                ffi::DiplomatSliceMut::<u32> {
                    ptr: out.as_mut_ptr(),
                    len: out.len(),
                },
            )
        };
    }
}

impl<'view> NumbersRef<'view> {
    pub fn values<'a>(&'a self) -> &'a [u32] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Numbers_values(self.inner.as_ptr() as *const _) };
        unsafe { private::slice_from_raw_parts(result.ptr, result.len) }
    }
    pub fn sum(&self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Numbers_sum(self.inner.as_ptr() as *const _) }
    }
    pub fn fill(&self, out: &mut [u32]) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::Numbers_fill(
                self.inner.as_ptr() as *const _,
                ffi::DiplomatSliceMut::<u32> {
                    ptr: out.as_mut_ptr(),
                    len: out.len(),
                },
            )
        };
    }
}

impl<'view> NumbersRefMut<'view> {
    pub fn values<'a>(&'a self) -> &'a [u32] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Numbers_values(self.inner.as_ptr() as *const _) };
        unsafe { private::slice_from_raw_parts(result.ptr, result.len) }
    }
    pub fn values_mut<'a>(&'a mut self) -> &'a mut [u32] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::Numbers_values_mut(self.inner.as_ptr()) };
        unsafe { private::slice_from_raw_parts_mut(result.ptr, result.len) }
    }
    pub fn sum(&self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::Numbers_sum(self.inner.as_ptr() as *const _) }
    }
    pub fn fill(&self, out: &mut [u32]) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::Numbers_fill(
                self.inner.as_ptr() as *const _,
                ffi::DiplomatSliceMut::<u32> {
                    ptr: out.as_mut_ptr(),
                    len: out.len(),
                },
            )
        };
    }
}

/// Owned opaque whose type-level lifetime is tied to the caller's buffer.
pub struct SliceView<'a> {
    inner: NonNull<ffi::SliceView>,
    _lifetimes: PhantomData<fn(&'a ()) -> &'a ()>,
    _not_send_sync: PhantomData<Rc<()>>,
}

pub struct SliceViewRef<'view, 'a> {
    inner: NonNull<ffi::SliceView>,
    _borrow: PhantomData<&'view ()>,
    _lifetimes: PhantomData<fn(&'a ()) -> &'a ()>,
    _not_send_sync: PhantomData<Rc<()>>,
}

pub struct SliceViewRefMut<'view, 'a> {
    inner: NonNull<ffi::SliceView>,
    _borrow: PhantomData<&'view mut ()>,
    _lifetimes: PhantomData<fn(&'a ()) -> &'a ()>,
    _not_send_sync: PhantomData<Rc<()>>,
}

#[doc(hidden)]
pub trait SliceViewSharedArg: private::SliceViewSharedSealed {}

#[doc(hidden)]
pub trait SliceViewMutArg: private::SliceViewMutSealed {}

impl<'a> private::SliceViewSharedSealed for SliceView<'a> {
    fn __as_const_ptr(&self) -> *const ffi::SliceView {
        self.inner.as_ptr()
    }
}
impl<'a> private::SliceViewMutSealed for SliceView<'a> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::SliceView {
        self.inner.as_ptr()
    }
}
impl<'a> SliceViewSharedArg for SliceView<'a> {}
impl<'a> SliceViewMutArg for SliceView<'a> {}
impl<'view, 'a> private::SliceViewSharedSealed for SliceViewRef<'view, 'a> {
    fn __as_const_ptr(&self) -> *const ffi::SliceView {
        self.inner.as_ptr()
    }
}
impl<'view, 'a> SliceViewSharedArg for SliceViewRef<'view, 'a> {}
impl<'view, 'a> private::SliceViewSharedSealed for SliceViewRefMut<'view, 'a> {
    fn __as_const_ptr(&self) -> *const ffi::SliceView {
        self.inner.as_ptr()
    }
}
impl<'view, 'a> private::SliceViewMutSealed for SliceViewRefMut<'view, 'a> {
    fn __as_mut_ptr(&mut self) -> *mut ffi::SliceView {
        self.inner.as_ptr()
    }
}
impl<'view, 'a> SliceViewSharedArg for SliceViewRefMut<'view, 'a> {}
impl<'view, 'a> SliceViewMutArg for SliceViewRefMut<'view, 'a> {}

impl<'a> Drop for SliceView<'a> {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.
        unsafe { ffi::SliceView_destroy(self.inner.as_ptr()) };
    }
}

impl<'a> SliceView<'a> {
    pub fn wrap(data: &'a [u8]) -> SliceView<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe {
            ffi::SliceView_wrap(ffi::DiplomatSlice::<u8> {
                ptr: data.as_ptr(),
                len: data.len(),
            })
        };
        {
            let inner = NonNull::new(result as *mut _)
                .expect("Diplomat ABI returned null for non-null SliceView");
            SliceView {
                inner,
                _lifetimes: PhantomData,
                _not_send_sync: PhantomData,
            }
        }
    }
    pub fn len(&self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::SliceView_len(self.inner.as_ptr() as *const _) }
    }
    pub fn is_empty(&self) -> bool {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::SliceView_is_empty(self.inner.as_ptr() as *const _) }
    }
    pub fn get(&self, index: u32) -> u8 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::SliceView_get(self.inner.as_ptr() as *const _, index) }
    }
    pub fn sum(&self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::SliceView_sum(self.inner.as_ptr() as *const _) }
    }
    pub fn data<'b>(&'b self) -> &'b [u8] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::SliceView_data(self.inner.as_ptr() as *const _) };
        unsafe { private::slice_from_raw_parts(result.ptr, result.len) }
    }
    pub fn fields<'anon_0>(&'anon_0 self) -> FieldView<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::SliceView_fields(self.inner.as_ptr() as *const _) };
        FieldView {
            bytes: unsafe { private::slice_from_raw_parts(result.bytes.ptr, result.bytes.len) },
            count: result.count,
            _lifetimes: PhantomData,
        }
    }
}

impl<'view, 'a> SliceViewRef<'view, 'a> {
    pub fn len(&self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::SliceView_len(self.inner.as_ptr() as *const _) }
    }
    pub fn is_empty(&self) -> bool {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::SliceView_is_empty(self.inner.as_ptr() as *const _) }
    }
    pub fn get(&self, index: u32) -> u8 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::SliceView_get(self.inner.as_ptr() as *const _, index) }
    }
    pub fn sum(&self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::SliceView_sum(self.inner.as_ptr() as *const _) }
    }
    pub fn data<'b>(&'b self) -> &'b [u8] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::SliceView_data(self.inner.as_ptr() as *const _) };
        unsafe { private::slice_from_raw_parts(result.ptr, result.len) }
    }
    pub fn fields<'anon_0>(&'anon_0 self) -> FieldView<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::SliceView_fields(self.inner.as_ptr() as *const _) };
        FieldView {
            bytes: unsafe { private::slice_from_raw_parts(result.bytes.ptr, result.bytes.len) },
            count: result.count,
            _lifetimes: PhantomData,
        }
    }
}

impl<'view, 'a> SliceViewRefMut<'view, 'a> {
    pub fn len(&self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::SliceView_len(self.inner.as_ptr() as *const _) }
    }
    pub fn is_empty(&self) -> bool {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::SliceView_is_empty(self.inner.as_ptr() as *const _) }
    }
    pub fn get(&self, index: u32) -> u8 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::SliceView_get(self.inner.as_ptr() as *const _, index) }
    }
    pub fn sum(&self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::SliceView_sum(self.inner.as_ptr() as *const _) }
    }
    pub fn data<'b>(&'b self) -> &'b [u8] {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::SliceView_data(self.inner.as_ptr() as *const _) };
        unsafe { private::slice_from_raw_parts(result.ptr, result.len) }
    }
    pub fn fields<'anon_0>(&'anon_0 self) -> FieldView<'a> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::SliceView_fields(self.inner.as_ptr() as *const _) };
        FieldView {
            bytes: unsafe { private::slice_from_raw_parts(result.bytes.ptr, result.bytes.len) },
            count: result.count,
            _lifetimes: PhantomData,
        }
    }
}
