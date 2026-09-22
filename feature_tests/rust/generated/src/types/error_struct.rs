use crate::ffi;

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
