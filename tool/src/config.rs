use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    str,
};

use quote::ToTokens;
use serde::{Deserialize, Serialize};
use syn::{
    parse::{Parse, ParseStream},
    Expr, Ident, Token,
};
use toml::{value::Table, Value};

use crate::{cpp::CppConfig, demo_gen::DemoConfig, js::JsConfig, kotlin::KotlinConfig};
use diplomat_core::hir::LoweringConfig;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct SharedConfig {
    pub lib_name: Option<String>,
    /// Whether or not callbacks support references in parameters. This is unsafe: you need to be careful to not
    /// retain these references on the foreign side.
    pub unsafe_references_in_callbacks: Option<bool>,
    /// The folder to pull custom bindings from. Defaults to the lib.rs folder.
    pub custom_binding_location: PathBuf,
}

impl SharedConfig {
    // / Quick and dirty way to tell [`set_overrides`] whether or not to copy an override from a specific language over.
    pub fn overrides_shared(name: &str) -> bool {
        // Expect the first item in the iterator to be the name of the language, so we eliminate that:
        let name: String = name.split(".").skip(1).collect();
        matches!(name.as_str(), "lib_name" | "unsafe_references_in_callbacks")
    }

    pub fn set(&mut self, key: &str, value: Value) {
        match key {
            "lib_name" => {
                if value.is_str() {
                    self.lib_name = value.as_str().map(|v| v.to_string())
                } else {
                    panic!("Config key `lib_name` must be a string");
                }
            }
            "unsafe_references_in_callbacks" => {
                if value.is_bool() {
                    self.unsafe_references_in_callbacks = value.as_bool()
                } else {
                    panic!("Config key `unsafe_references_in_callbacks` must be a boolean");
                }
            }
            "custom_binding_location" => {
                if value.is_str() {
                    self.custom_binding_location = PathBuf::from(value.as_str().unwrap())
                } else {
                    panic!("Config key `custom_binding_location` must be a string");
                }
            }
            _ => (),
        }
    }

    pub fn lowering_config(&self) -> LoweringConfig {
        let mut cfg = LoweringConfig::default();
        if let Some(refs) = self.unsafe_references_in_callbacks {
            cfg.unsafe_references_in_callbacks = refs;
        }
        cfg
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    pub shared_config: SharedConfig,
    #[serde(rename = "kotlin")]
    pub kotlin_config: KotlinConfig,
    #[serde(rename = "demo_gen")]
    pub demo_gen_config: DemoConfig,
    #[serde(rename = "js")]
    pub js_config: JsConfig,
    #[serde(rename = "cpp")]
    pub cpp_config: CppConfig,
    /// Any language can override what's in [`SharedConfig`]. This is a structure that holds information about those specific overrides. [`Config`] will update [`SharedConfig`] based on the current language.
    #[serde(skip)]
    pub language_overrides: HashMap<String, Value>,
}

impl Config {
    pub fn set(&mut self, key: &str, value: Value) {
        if key.starts_with("kotlin.") {
            if SharedConfig::overrides_shared(key) {
                self.language_overrides.insert(key.to_string(), value);
            } else {
                self.kotlin_config.set(&key.replace("kotlin.", ""), value);
            }
        } else if key.starts_with("demo_gen.") {
            if SharedConfig::overrides_shared(key) {
                self.language_overrides.insert(key.to_string(), value);
            } else {
                self.demo_gen_config
                    .set(&key.replace("demo_gen.", ""), value);
            }
        } else if key.starts_with("nanobind.") {
            if SharedConfig::overrides_shared(key) {
                self.language_overrides.insert(key.to_string(), value);
            } // nanobind doesn't have any other config setting
        } else if key.starts_with("js.") {
            if SharedConfig::overrides_shared(key) {
                self.language_overrides.insert(key.to_string(), value);
            } else {
                self.js_config.set(&key.replace("js.", ""), value);
            }
        } else if key.starts_with("cpp.") {
            if SharedConfig::overrides_shared(key) {
                self.language_overrides.insert(key.to_string(), value);
            } else {
                self.cpp_config.set(&key.replace("cpp.", ""), value);
            }
        } else {
            self.shared_config.set(key, value)
        }
    }

    pub fn get_overridden(self, target_language: &str) -> Self {
        let mut out = self.clone();

        // Look for a match of language_name.some_value in a potential key.
        let m = format!("{target_language}.");
        for (k, v) in out.language_overrides.iter() {
            if k.starts_with(&m) {
                out.shared_config.set(&k.replace(&m, ""), v.clone());
            }
        }
        out
    }

    /// Given a filepath, read TOML formatted config settings from it (and modify the current Config struct from the read)
    pub fn read_file(&mut self, path: &Path) -> Result<(), String> {
        let config_table: Table = if path.exists() {
            let file_buf = std::fs::read(path).map_err(|e| e.to_string())?;
            let s = str::from_utf8(&file_buf).map_err(|_| "Config file is not UTF8".to_string())?;
            toml::from_str(s).map_err(|_| "Config file is not valid TOML".to_string())?
        } else {
            Table::default()
        };

        for (key, value) in config_table {
            // Quick way to take config.toml from kebab to snake case.
            // This technically means that someone could also just as easily do CamelCase and have it translated,
            // but I'm not sure I want to bother writing validation code for such a scenario.
            let key = heck::AsSnakeCase(key).to_string();
            if let toml::Value::Table(t) = value {
                for (subkey, subvalue) in t {
                    let subkey = heck::AsSnakeCase(subkey).to_string();
                    self.set(&format!("{key}.{subkey}"), subvalue);
                }
            } else {
                self.set(&key, value);
            }
        }
        Ok(())
    }

    /// Given a vector of strings with the format `config.setting = value`, modify the `Config` struct appropriately.
    pub fn read_cli_settings(&mut self, settings: Vec<String>) {
        for c in settings {
            let split = c.split_once("=");
            if let Some((key, value)) = split {
                self.set(key, toml_value_from_str(value));
            } else {
                eprintln!("Could not read {c}, expected =");
            }
        }
    }
}

pub fn toml_value_from_str(string: &str) -> toml::Value {
    let try_parse = toml::from_str::<toml::Value>(string);

    // If there's an error parsing (because clap will not parse quotes, for example), we just treat what we're passed as a string:
    // toml from_str
    if let Ok(out) = try_parse {
        out
    } else {
        toml::Value::String(string.to_string())
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
#[non_exhaustive]
pub(crate) struct DiplomatBackendConfigAttr {
    pub key_value_pairs: Vec<DiplomatBackendConfigKeyValue>,
}

impl Parse for DiplomatBackendConfigAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let list = input.parse_terminated(DiplomatBackendConfigKeyValue::parse, Token![,])?;
        let vec = list.into_iter().collect();
        Ok(Self {
            key_value_pairs: vec,
        })
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
#[non_exhaustive]
pub(crate) struct DiplomatBackendConfigKeyValue {
    pub key: String,
    pub value: String,
}

impl Parse for DiplomatBackendConfigKeyValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut key_str: Vec<String> = Vec::new();

        loop {
            let i: Ident = input.parse()?;

            key_str.push(i.to_string());

            if input.peek(Token![.]) {
                let _period: Token![.] = input.parse()?;
            } else {
                break;
            }
        }

        let _equals: Token![=] = input.parse()?;

        let val_expr: Expr = input.parse()?;

        let value = val_expr.to_token_stream().to_string();

        Ok(Self {
            key: key_str.join("."),
            value,
        })
    }
}

pub(crate) fn find_top_level_attr(module_items: Vec<syn::Item>) -> Vec<DiplomatBackendConfigAttr> {
    let path = syn::parse_str("diplomat::config").unwrap();

    let attrs = module_items
        .iter()
        .filter_map(|i| match i {
            syn::Item::Struct(s) => Some(s.attrs.clone()),
            syn::Item::Impl(i) => Some(i.attrs.clone()),
            syn::Item::Mod(m) => Some(m.attrs.clone()),
            _ => None,
        })
        .filter_map(|attrs| {
            let attributes_vec = attrs
                .iter()
                .filter_map(|attribute| {
                    if attribute.path() == &path {
                        Some(
                            syn::parse2::<DiplomatBackendConfigAttr>(
                                attribute
                                    .parse_args()
                                    .expect("Failed to parse malformed diplomat::config"),
                            )
                            .expect("Could not parse DiplomatBackendConfig attribute."),
                        )
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            if !attributes_vec.is_empty() {
                Some(attributes_vec)
            } else {
                None
            }
        });

    let mut out_config = Vec::new();

    for mut a in attrs {
        out_config.append(&mut a);
    }

    out_config
}

#[cfg(test)]
mod test {
    use toml::Value;

    #[test]
    fn test_toml_parse() {
        let t = "true";
        assert!(toml::from_str::<Value>(t).is_err());
        assert_eq!(super::toml_value_from_str(t), Value::String(t.to_string()));
    }
}
