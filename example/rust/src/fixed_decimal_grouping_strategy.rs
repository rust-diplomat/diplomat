#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FixedDecimalGroupingStrategy {
    /// Auto grouping
    Auto = 0,
    /// No grouping
    Never = 1,
    /// Always group
    Always = 2,
    /// At least 2 groups
    Min2 = 3,
}

impl FixedDecimalGroupingStrategy {
    /// Whether this strategy can group digits at all.
    pub fn groups_digits(self) -> bool {
        unsafe { icu4x_FixedDecimalGroupingStrategy_groups_digits_mv1(self) }
    }
}

extern "C" {
    fn icu4x_FixedDecimalGroupingStrategy_groups_digits_mv1(
        this: crate::FixedDecimalGroupingStrategy,
    ) -> bool;
}
