//! Generated enums and value structs.

use core::marker::PhantomData;

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
pub struct RenamedDeprecatedStruct {}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RenamedRenamedCachedIncludeZST {}

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
pub struct MyZst {}
