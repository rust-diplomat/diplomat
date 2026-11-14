#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenamedStructWithAttrs {
    pub a: bool,
    pub b: u32,
}

impl RenamedStructWithAttrs {
    pub fn new_fallible(a: bool, b: u32) -> Result<crate::RenamedStructWithAttrs, ()> {
        unsafe { namespace_StructWithAttrs_new_fallible(a, b) }
            .into_result()
    }

    pub fn c(self) -> u32 {
        unsafe { namespace_StructWithAttrs_c(self) }
    }

    pub fn deprecated(self) {
        unsafe { namespace_StructWithAttrs_deprecated(self) }
    }
}

extern "C" {
    fn namespace_StructWithAttrs_new_fallible(a: bool, b: u32) -> crate::DiplomatResult<crate::RenamedStructWithAttrs, ()>;
    fn namespace_StructWithAttrs_c(this: crate::RenamedStructWithAttrs) -> u32;
    fn namespace_StructWithAttrs_deprecated(this: crate::RenamedStructWithAttrs);
}
