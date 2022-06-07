/// As part of the macro expansion and code generation process, Diplomat
/// generates a simplified version of the Rust AST that captures special
/// types such as opaque structs, [`Box`], and [`Result`] with utilities
/// for handling such types.
mod methods;
pub use methods::{Method, Param, SelfParam};

mod modules;
pub use modules::{File, Module};

mod structs;
pub use structs::{OpaqueStruct, Struct};

mod enums;
pub use enums::Enum;

mod types;
pub use types::{
    CustomType, Lifetime, LifetimeDef, LifetimeOrigin, ModSymbol, Mutability, PathType,
    PrimitiveType, TypeName,
};

mod paths;
pub use paths::Path;

mod idents;
pub use idents::Ident;

mod docs;
pub use docs::{Docs, DocsUrlGenerator};

mod validity;
pub use validity::ValidityError;
