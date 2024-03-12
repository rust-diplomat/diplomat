use super::{Ident, Path, TypeName};

#[cfg_attr(feature = "displaydoc", derive(displaydoc::Display))]
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ValidityError {
    /// A return type contains elided lifetimes.
    #[cfg_attr(
        feature = "displaydoc",
        displaydoc("A return type contains elided lifetimes, which aren't yet supported: {sub_type} in {full_type}")
    )]
    LifetimeElisionInReturn {
        full_type: TypeName,
        sub_type: TypeName,
    },
    /// An alias or submodule was found instead of a custom type.
    #[cfg_attr(
        feature = "displaydoc",
        displaydoc("An alias or submodule was found instead of a custom type with the name {0}.")
    )]
    PathTypeNameConflict(Ident),
}
