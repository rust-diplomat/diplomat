# Feature Gates

Diplomat supports selectively disabling bindings based on a list of features. These features are separate from Cargo features. Enabled features can be specified through [config](../config.md), and are NOT tied into `Cargo.toml` features in any way.

So:

```rs
#[diplomat::attr(not(feature=some_feature), disable)]
mod ffi {}
```
Will only appear for backends which have `some_feature` enabled. You can do this through the config `features_enabled`:

```rs
#[diplomat::config(features_enabled=["this_feature", "some_feature"])]
struct Config;
```

Which will cause anything tagged with `some_feature` to generate across all backends.

## CLI

You can set a list of features that all backends can use (backend-specific configs can override this):

```
diplomat_tool cpp . --features-enabled=some,comma,separated,list
```