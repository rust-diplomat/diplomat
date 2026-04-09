# Including Macros

Diplomat is able to prepend the contents of any file to a `#[diplomat::bridge]` module. This is useful for being able to include and use macros across different modules:


## `src/shared_macro.rs`
```rs
#[diplomat::macro_rules]
#[macro_export]
macro_rules! shared_macro {
    /* ... */
}
```

## `src/macro_use.rs`

```rs
// Grab shared_macro definition:
use super::*;

#[diplomat::bridge]
#[diplomat::include("src/shared_macro.rs")]
mod ffi {
    super::shared_macro!();
}
```

## Usage
`#[diplomat::bridge]` is evaluated by proc_macro individually, so every module that uses `#[diplomat::bridge]` that wants to use the same macros must each have a separate `#[diplomat::include]` declaration.

`#[diplomat::include(...)]` evaluates a path always relative to `Cargo.toml`. This is detected automatically when building your library, but may need to be set when using `diplomat_tool` through the `manifest_dir` [config](../config.md).