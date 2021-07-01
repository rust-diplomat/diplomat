/// As part of the macro expansion and code generation process, Diplomat
/// generates a simplified version of the Rust AST that captures special
/// types such as opaque structs, [`Box`], and [`Result`] with utilities
/// for handling such types.
mod methods;
pub use methods::*;

mod modules;
pub use modules::*;

mod structs;
pub use structs::*;

mod types;
pub use types::*;
