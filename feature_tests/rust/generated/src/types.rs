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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BorrowingOptionStruct<'a> {
    pub a: Option<&'a [u8]>,
    pub(crate) _lifetimes: PhantomData<*mut &'a ()>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OptionInputStruct {
    pub a: Option<u8>,
    pub b: Option<char>,
    pub c: Option<OptionEnum>,
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
            Ok(result) => Ok(result.into_option()),
            Err(result) => Err(result),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ErrorWithChar {
    pub c: char,
}

/// Testing JS-specific layout/padding behavior
/// Also being used to test CPP backends taking structs with primitive values.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BigStructWithStuff {
    pub first: u8,
    pub second: u16,
    pub third: u16,
    pub fourth: ScalarPairWithPadding,
    pub fifth: u8,
}

impl BigStructWithStuff {
    pub fn assert_value(self, extra_val: u16) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::BigStructWithStuff_assert_value(self, extra_val) };
    }
    pub fn assert_slice(slice: &[BigStructWithStuff], second_value: u16) {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::BigStructWithStuff_assert_slice(ffi::DiplomatSlice::from(slice), second_value)
        };
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CyclicStructA {
    pub a: CyclicStructB,
}

impl CyclicStructA {
    pub fn get_b() -> CyclicStructB {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::CyclicStructA_get_b() }
    }
    pub fn cyclic_out(self) -> String {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        crate::private::with_write(|write| {
            unsafe { ffi::CyclicStructA_cyclic_out(self, write) };
        })
        .1
    }
    pub fn nested_slice(sl: &[CyclicStructA]) -> u8 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::CyclicStructA_nested_slice(ffi::DiplomatSlice::from(sl)) }
    }
    pub fn double_cyclic_out(self, cyclic_struct_a: CyclicStructA) -> String {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        crate::private::with_write(|write| {
            unsafe { ffi::CyclicStructA_double_cyclic_out(self, cyclic_struct_a, write) };
        })
        .1
    }
    pub fn getter_out(self) -> String {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        crate::private::with_write(|write| {
            unsafe { ffi::CyclicStructA_getter_out(self, write) };
        })
        .1
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CyclicStructB {
    pub field: u8,
}

impl CyclicStructB {
    pub fn get_a() -> CyclicStructA {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::CyclicStructB_get_a() }
    }
    pub fn get_a_option() -> Option<CyclicStructA> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::CyclicStructB_get_a_option() };
        result.into()
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CyclicStructC {
    pub a: CyclicStructA,
}

impl CyclicStructC {
    pub fn takes_nested_parameters(c: CyclicStructC) -> CyclicStructC {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe { ffi::CyclicStructC_takes_nested_parameters(c) }
    }
    pub fn cyclic_out(self) -> String {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        crate::private::with_write(|write| {
            unsafe { ffi::CyclicStructC_cyclic_out(self, write) };
        })
        .1
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MyStruct {
    pub a: u8,
    pub b: bool,
    pub c: u8,
    pub d: u64,
    pub e: i32,
    pub f: char,
    pub g: MyEnum,
}

impl MyStruct {
    pub fn new() -> MyStruct {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::MyStruct_new() };
        MyStruct {
            a: result.a,
            b: result.b,
            c: result.c,
            d: result.d,
            e: result.e,
            f: crate::private::char_from_u32(result.f),
            g: result.g,
        }
    }
    pub fn into_a(self) -> u8 {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        unsafe {
            ffi::MyStruct_into_a(ffi::MyStruct {
                a: self.a,
                b: self.b,
                c: self.c,
                d: self.d,
                e: self.e,
                f: self.f as u32,
                g: self.g,
            })
        }
    }
    pub fn returns_zst_result() -> Result<(), MyZst> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::MyStruct_returns_zst_result() };
        result.into()
    }
    pub fn fails_zst_result() -> Result<(), MyZst> {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::MyStruct_fails_zst_result() };
        result.into()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MyStructContainingAnOption {
    pub a: Option<MyStruct>,
    pub b: Option<DefaultEnum>,
}

impl MyStructContainingAnOption {
    pub fn new() -> MyStructContainingAnOption {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::MyStructContainingAnOption_new() };
        MyStructContainingAnOption {
            a: result.a.into_option().map(|__v| MyStruct {
                a: __v.a,
                b: __v.b,
                c: __v.c,
                d: __v.d,
                e: __v.e,
                f: crate::private::char_from_u32(__v.f),
                g: __v.g,
            }),
            b: result.b.into_option(),
        }
    }
    pub fn filled() -> MyStructContainingAnOption {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::MyStructContainingAnOption_filled() };
        MyStructContainingAnOption {
            a: result.a.into_option().map(|__v| MyStruct {
                a: __v.a,
                b: __v.b,
                c: __v.c,
                d: __v.d,
                e: __v.e,
                f: crate::private::char_from_u32(__v.f),
                g: __v.g,
            }),
            b: result.b.into_option(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MyZst {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NestedCharField {
    pub ch: char,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NestedConvertingFields {
    pub char_inner: NestedCharField,
    pub option_inner: NestedOptionField,
}

impl NestedConvertingFields {
    pub fn new() -> NestedConvertingFields {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe { ffi::NestedConvertingFields_new() };
        NestedConvertingFields {
            char_inner: NestedCharField {
                ch: crate::private::char_from_u32(result.char_inner.ch),
            },
            option_inner: NestedOptionField {
                value: result.option_inner.value.into_option(),
            },
        }
    }
    pub fn round_trip(value: NestedConvertingFields) -> NestedConvertingFields {
        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.
        let result = unsafe {
            ffi::NestedConvertingFields_round_trip(ffi::NestedConvertingFields {
                char_inner: ffi::NestedCharField {
                    ch: value.char_inner.ch as u32,
                },
                option_inner: ffi::NestedOptionField {
                    value: ffi::DiplomatOption::from(value.option_inner.value),
                },
            })
        };
        NestedConvertingFields {
            char_inner: NestedCharField {
                ch: crate::private::char_from_u32(result.char_inner.ch),
            },
            option_inner: NestedOptionField {
                value: result.option_inner.value.into_option(),
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NestedOptionField {
    pub value: Option<u8>,
}

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
