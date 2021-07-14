/// As part of the macro expansion and code generation process, Diplomat
/// generates a simplified version of the Rust AST that captures special
/// types such as opaque structs, [`Box`], and [`Result`] with utilities
/// for handling such types.
mod methods;
pub use methods::{Method, Param};

mod modules;
pub use modules::{File, Module};

mod structs;
pub use structs::{OpaqueStruct, Struct};

mod types;
pub use types::{CustomType, PrimitiveType, TypeName};

mod utils;
