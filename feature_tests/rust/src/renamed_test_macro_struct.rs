#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenamedTestMacroStruct {
    pub a: usize,
}

impl RenamedTestMacroStruct {
    pub fn test_func() -> usize {
        unsafe { namespace_TestMacroStruct_test_func() }
    }

    pub fn test_meta() -> crate::RenamedTestMacroStruct {
        unsafe { namespace_TestMacroStruct_test_meta() }
    }
}

extern "C" {
    fn namespace_TestMacroStruct_test_func() -> usize;
    fn namespace_TestMacroStruct_test_meta() -> crate::RenamedTestMacroStruct;
}
