//! Generated enums and value structs.

use core::marker::PhantomData;

use crate::ffi;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenamedAttrEnum {
    A = 0,
    B = 1,
    Renamed = 2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenamedDeprecatedEnum {
    A = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FFIError {
    FFI = 0,
    User = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnimportedEnum {
    A = 0,
    B = 1,
    C = 2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OptionEnum {
    Foo = 0,
    Bar = 1,
    Baz = 2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorEnum {
    Foo = 0,
    Bar = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContiguousEnum {
    C = 0,
    D = 1,
    E = 2,
    F = 3,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DefaultEnum {
    A = 0,
    B = 1,
}

impl DefaultEnum {
    pub fn new() -> DefaultEnum {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::DefaultEnum_new() }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MyEnum {
    A = -2,
    B = -1,
    C = 0,
    D = 1,
    /// EEEEEEE
    E = 2,
    F = 3,
}

impl MyEnum {
    pub fn into_value(self) -> i8 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::MyEnum_into_value(self) }
    }
    pub fn get_a() -> MyEnum {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::MyEnum_get_a() }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RenamedDeprecatedStruct {}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RenamedRenamedCachedIncludeZST {}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RenamedStructWithAttrs {
    pub a: bool,
    pub b: u32,
}

impl RenamedStructWithAttrs {
    pub fn new_fallible(a: bool, b: u32) -> Result<RenamedStructWithAttrs, ()> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::namespace_StructWithAttrs_new_fallible(a, b) };
        result.into()
    }
    pub fn c(self) -> u32 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::namespace_StructWithAttrs_c(self) }
    }
    pub fn deprecated(self) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::namespace_StructWithAttrs_deprecated(self) };
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImportedStruct {
    pub foo: UnimportedEnum,
    pub count: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BorrowedFieldsReturning<'a> {
    pub bytes: &'a [u8],
    pub(crate) _lifetimes: PhantomData<*mut &'a ()>,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CachedIncludeZST {}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ErrorStruct {
    pub i: i32,
    pub j: i32,
}

impl ErrorStruct {
    pub fn returns_result_option(is_some: bool) -> Result<Option<ErrorStruct>, ()> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::ErrorStruct_returns_result_option(is_some) };
        match Result::from(result) {
            Ok(result) => Ok(result.into()),
            Err(result) => Err(result),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CyclicStructB {
    pub field: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MyZst {}

/// Testing JS-specific layout/padding behavior
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScalarPairWithPadding {
    pub first: u8,
    pub second: u32,
}

impl ScalarPairWithPadding {
    pub fn assert_value(self) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::ScalarPairWithPadding_assert_value(self) };
    }
}
