/// An error from lowering the AST to the HIR.
pub enum LoweringError {
    /// The purpose of having this is that translating to the HIR has enormous
    /// potential for really detailed error handling and giving suggestions.
    ///
    /// Unfortunately, working out what the error enum should look like to enable
    /// this is really hard. The plan is that once the lowering code is completely
    /// written, we ctrl+F for `"LoweringError::Other"` in the lowering code, and turn every
    /// instance into an specialized enum variant, generalizing where possible
    /// without losing any information.
    Other(String),
}
