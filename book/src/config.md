# Configuring Diplomat

Some Diplomat backends have configurable parameters to change the behavior of their output.

For instance, Kotlin requires a `domain` parameter and `lib_name` parameter. These two parameters are used to generate a package name and folder structure on generation.

## Configuration Structures
Documentation on what configuration options are available in [`config.rs` on GitHub](https://github.com/rust-diplomat/diplomat/tree/main/tool/src/config.rs).

Note that every configuration option uses `snake_case` for consistency, and that the `SharedConfig` struct is flattened. So for setting `--config shared_config.lib_name="some_value"`, this would instead be `--config lib_name="some_value"`.

## Configuration Interfaces

Configuration information can be set in four ways:

### `config.toml`

By default, Diplomat scans for a `config.toml` in the folder where `diplomat_tool` is being run. You can change the location of this folder with the `--config_file` parameter.

The structure of `config.toml` is as follows:

```toml
# Top level table specifies Shared Config settings that apply to all backends:
lib-name = "MyLibrary"

[kotlin]
# Individual tables can override Shared Config settings:
lib-name = "LibraryNameOverride"
# Along with backend specific settings:
domain = "org.myOrganization"

[demo_gen]
explicit-generation = true

[other-library-name]
some-value = 100
```

### `diplomat-tool` CLI
When running `diplomat-tool`, you may pass in the `--config` flag for each option you wish to set:

```
./diplomat-tool kotlin ./kotlin-folder --config lib_name="MyLibrary" --config kotlin.domain = "org.myOrganization"
```

`diplomat-tool` flags take priority over `config.toml`.

### `diplomat_tool::gen`

If you call `diplomat_tool::gen` manually, then you have the option of setting configuration yourself, with the `diplomat_tool::config::Config` struct.

See [Backend Structures](#backend-structures) for more on these structures.

### `#[diplomat::config(...)]`

In `lib.rs`, any top-level `mod`ule, `struct`, or `impl` block can use the `#[diplomat::config]` attribute:

```rust
#[diplomat::config(lib_name="MyLibrary")]
struct SomeConfig;

#[diplomat::config(kotlin.domain="org.myOrganization")]
mod kotlin_specific_mod;

#[diplomat::config(...)]
impl SomeConfig {

}
```

Due to a quirk of how Diplomat reads these attributes, `#[diplomat::config]` has priority over all other methods of setting configuration.