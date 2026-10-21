#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FixedDecimalFormatterOptions {
    pub grouping_strategy: crate::FixedDecimalGroupingStrategy,
    pub some_other_config: bool,
}

impl FixedDecimalFormatterOptions {
    pub fn default() -> crate::FixedDecimalFormatterOptions {
        unsafe { icu4x_FixedDecimalFormatterOptions_default_mv1() }
    }
}

extern "C" {
    fn icu4x_FixedDecimalFormatterOptions_default_mv1() -> crate::FixedDecimalFormatterOptions;
}
