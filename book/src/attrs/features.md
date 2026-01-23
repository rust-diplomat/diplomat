# Feature Gates

Diplomat supports selectively disabling bindings based on a list of features. These features are separate from Cargo features. Enabled features can be specified through [config](../config.md), although Diplomat treats ALL features as enabled by default.

So:

```rs
#[diplomat::attr(not(feature=some_feature), disable)]
mod ffi {}
```

Will be enabled by default across all backends.

If you set the `features_enabled` config across all backends:

```rs
#[diplomat::config(features_enabled=["this_feature", "other_feature"])]
struct Config;
```

Then `some_feature` will no longer generate across all backends unless explicitly enabled.

## CLI

You can set a list of features that all backends can use (backend-specific configs can override this):

```
diplomat_tool cpp . --features-enabled=some,comma,separated,list
```

OR

```
diplomat_tool cpp . --all-features-enabled
```