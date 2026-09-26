#[allow(unused_imports)]
use super::DefaultEnum;
#[allow(unused_imports)]
use super::MyStruct;
use crate::ffi;

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
