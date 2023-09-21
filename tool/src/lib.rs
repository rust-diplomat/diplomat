// Enable once https://github.com/rust-lang/rust/issues/89554 is stable
// #![deny(non_exhaustive_omitted_patterns)] // diplomat_core uses non_exhaustive a lot; we should never miss its patterns

pub mod c;
pub mod c2;
pub mod common;
pub mod cpp;
pub mod cpp2;
mod docs_util;
pub mod dotnet;
pub mod js;
mod layout;
mod util;
