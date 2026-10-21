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
