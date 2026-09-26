#[allow(unused_imports)]
use super::MyEnum;
#[allow(unused_imports)]
use super::MyZst;
use crate::ffi;

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
