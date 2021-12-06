use super::{Path, TypeName};

#[cfg_attr(feature = "displaydoc", derive(displaydoc::Display))]
pub enum ValidityError {
    #[cfg_attr(
        feature = "displaydoc",
        doc = "An opaque type crossed the FFI boundary as a value: {0:?}"
    )]
    OpaqueAsValue(TypeName),
    #[cfg_attr(
        feature = "displaydoc",
        doc = "A non-opaque zero-sized struct or enum has been defined: {0:?}"
    )]
    NonOpaqueZST(Path),
}
