//! Sealed capability traits and unsafe ABI reconstruction helpers.
//!
//! This module is private, so downstream crates cannot name the sealed traits and
//! therefore cannot implement the public capability traits themselves. The helpers
//! may be unused for a given provider, hence the `dead_code` allow.
#![allow(dead_code)]

pub trait AttrOpaque1RenamedSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::AttrOpaque1Renamed;
}
pub trait AttrOpaque1RenamedMutSealed: AttrOpaque1RenamedSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::AttrOpaque1Renamed;
}
pub trait RenamedAttrOpaque2SharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::RenamedAttrOpaque2;
}
pub trait RenamedAttrOpaque2MutSealed: RenamedAttrOpaque2SharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::RenamedAttrOpaque2;
}
pub trait RenamedDeprecatedOpaqueSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::RenamedDeprecatedOpaque;
}
pub trait RenamedDeprecatedOpaqueMutSealed: RenamedDeprecatedOpaqueSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::RenamedDeprecatedOpaque;
}
pub trait RenamedMixinTestSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::RenamedMixinTest;
}
pub trait RenamedMixinTestMutSealed: RenamedMixinTestSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::RenamedMixinTest;
}
pub trait RenamedNestedSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::RenamedNested;
}
pub trait RenamedNestedMutSealed: RenamedNestedSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::RenamedNested;
}
pub trait RenamedNested2SharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::RenamedNested2;
}
pub trait RenamedNested2MutSealed: RenamedNested2SharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::RenamedNested2;
}
pub trait RenamedOpaqueZSTIndexerSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::RenamedOpaqueZSTIndexer;
}
pub trait RenamedOpaqueZSTIndexerMutSealed: RenamedOpaqueZSTIndexerSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::RenamedOpaqueZSTIndexer;
}
pub trait RenamedTestOpaqueSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::RenamedTestOpaque;
}
pub trait RenamedTestOpaqueMutSealed: RenamedTestOpaqueSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::RenamedTestOpaque;
}
pub trait UnnamespacedSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::Unnamespaced;
}
pub trait UnnamespacedMutSealed: UnnamespacedSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::Unnamespaced;
}
pub trait RenamedVectorTestSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::RenamedVectorTest;
}
pub trait RenamedVectorTestMutSealed: RenamedVectorTestSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::RenamedVectorTest;
}
pub trait BarSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::Bar;
}
pub trait BarMutSealed: BarSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::Bar;
}
pub trait FooSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::Foo;
}
pub trait FooMutSealed: FooSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::Foo;
}
pub trait OneSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::One;
}
pub trait OneMutSealed: OneSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::One;
}
pub trait OpaqueThinSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::OpaqueThin;
}
pub trait OpaqueThinMutSealed: OpaqueThinSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::OpaqueThin;
}
pub trait OpaqueThinIterSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::OpaqueThinIter;
}
pub trait OpaqueThinIterMutSealed: OpaqueThinIterSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::OpaqueThinIter;
}
pub trait OpaqueThinVecSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::OpaqueThinVec;
}
pub trait OpaqueThinVecMutSealed: OpaqueThinVecSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::OpaqueThinVec;
}
pub trait TwoSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::Two;
}
pub trait TwoMutSealed: TwoSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::Two;
}
pub trait OptionOpaqueSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::OptionOpaque;
}
pub trait OptionOpaqueMutSealed: OptionOpaqueSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::OptionOpaque;
}
pub trait OptionOpaqueCharSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::OptionOpaqueChar;
}
pub trait OptionOpaqueCharMutSealed: OptionOpaqueCharSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::OptionOpaqueChar;
}
pub trait OptionStringSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::OptionString;
}
pub trait OptionStringMutSealed: OptionStringSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::OptionString;
}
pub trait ResultOpaqueSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::ResultOpaque;
}
pub trait ResultOpaqueMutSealed: ResultOpaqueSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::ResultOpaque;
}
pub trait RefListSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::RefList;
}
pub trait RefListMutSealed: RefListSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::RefList;
}
pub trait RefListParameterSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::RefListParameter;
}
pub trait RefListParameterMutSealed: RefListParameterSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::RefListParameter;
}
pub trait Float64VecSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::Float64Vec;
}
pub trait Float64VecMutSealed: Float64VecSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::Float64Vec;
}
pub trait MyStringSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::MyString;
}
pub trait MyStringMutSealed: MyStringSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::MyString;
}
pub trait OwnedSliceReturnSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::OwnedSliceReturn;
}
pub trait OwnedSliceReturnMutSealed: OwnedSliceReturnSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::OwnedSliceReturn;
}
pub trait MyOpaqueEnumSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::MyOpaqueEnum;
}
pub trait MyOpaqueEnumMutSealed: MyOpaqueEnumSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::MyOpaqueEnum;
}
pub trait OpaqueSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::Opaque;
}
pub trait OpaqueMutSealed: OpaqueSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::Opaque;
}
pub trait OpaqueMutSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::OpaqueMut;
}
pub trait OpaqueMutMutSealed: OpaqueMutSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::OpaqueMut;
}
pub trait OpaqueMutexedStringSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::OpaqueMutexedString;
}
pub trait OpaqueMutexedStringMutSealed: OpaqueMutexedStringSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::OpaqueMutexedString;
}
pub trait PrimitiveStructVecSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::PrimitiveStructVec;
}
pub trait PrimitiveStructVecMutSealed: PrimitiveStructVecSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::PrimitiveStructVec;
}
pub trait Utf16WrapSharedSealed {
    fn __as_const_ptr(&self) -> *const crate::ffi::Utf16Wrap;
}
pub trait Utf16WrapMutSealed: Utf16WrapSharedSealed {
    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::Utf16Wrap;
}

/// Reconstruct a validated `&str` from a provider-returned UTF-8 slice.
///
/// The runtime can turn a `DiplomatSlice` back into a `&[T]`, but
/// `DiplomatUtf8StrSlice`'s field is private, so this is the one conversion the
/// runtime cannot express and the generated crate keeps locally.
///
/// # Safety
///
/// The caller must uphold the provider's validity, alignment, aliasing, UTF-8,
/// and lifetime contract for the slice for the returned lifetime. The ABI type
/// cannot express UTF-8 validity, so the provider must send valid UTF-8.
pub(crate) unsafe fn utf8_str_from_slice<'a>(
    slice: diplomat_runtime::DiplomatSlice<'a, u8>,
) -> &'a str {
    core::str::from_utf8_unchecked(<&[u8]>::from(slice))
}
