# Configuring Markup
Diplomat takes the `-l` or `--library-config` option (in `diplomat_tool::gen` this is the `library_config` parameter). This represents a path to a `.toml` file that demo_gen will then read and convert into `DemoConfig`.

Here's a sample .toml file for configuration (with comments for clarity):

```toml
# If false, demo_gen will automatically search all methods for functions it can generate demonstration JS for.
# If true, demo_gen will look for any methods explicitly flagged with #[diplomat::demo(generate)] to perform generation.
explicit-generation=true # default = false (bool)

# This removes the rendering/ folder.
hide-default-renderer=true # default = false (bool)

# Adjusts all imports that demo_gen creates to a specific module. Setting this will not generate the js/ folder.
#
# So for instance, this setting will adjust imports to: `import { type } from "icu4x";
module-name="icu4x" # (string)

# Adjusts all imports that demo_gen creates to a relative path where Diplomat JS output should be. Setting this will not generate the js/ folder.
# 
# Setting this will adjust imports to: `import {type} from "../js/folder/here/index.mjs";
# 
# Intended to be a mutually exclusive setting with module-name, although you can set both simultaneously to import modules from a relative path. 
relative-js-path="../js/folder/here" # (string)
```