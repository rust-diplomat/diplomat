use diplomat_core::ast::{attrs::DiplomatBackendConfigAttr, Attrs};
use serde::{Deserialize, Serialize};
use toml::value::Table;

use crate::{demo_gen::DemoConfig, kotlin::KotlinConfig};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct SharedConfig {
    pub lib_name: Option<String>,
}

impl SharedConfig {
    /// Quick and dirty way to tell [`set_overrides`] whether or not to copy an override from a specific language over.
    pub fn overrides_shared(name : &str) -> bool {
        match name {
            "lib_name" => true,
            _ => false
        }
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    pub shared_config: SharedConfig,
    #[serde(rename = "kotlin")]
    pub kotlin_config: Option<KotlinConfig>,
    #[serde(rename = "demo_gen")]
    pub demo_gen_config: Option<DemoConfig>,
}

fn merge_subconfig(base: &mut Table, other: Table) {
    for (key, val) in other {
        match val {
            toml::Value::Table(t) => {
                if let Some(b) = base.get_mut(&key) {
                    if b.is_table() {
                        merge_subconfig(b.as_table_mut().unwrap(), t);
                        continue;
                    }
                }
                base.insert(key, toml::Value::Table(t));
            }
            _ => {
                base.insert(key, val);
            }
        };
    }
}

pub fn merge_config(base: &mut Table, other: Table) {
    for (key, val) in other {
        if !base.contains_key(&key) {
            base.insert(key, val);
            continue;
        }

        match val {
            // If this is a table, go into the table for the base:
            toml::Value::Table(t) => {
                merge_subconfig(
                    base.get_mut(&key)
                        .unwrap()
                        .as_table_mut()
                        .expect("Expected a table of values in base config."),
                    t
                );
            }
            // Otherwise just overwrite whatever's already there:
            _ => {
                base.insert(key, val);
            }
        }
    }
}

/// Returns a constructed table from a list of `key=value` pairs. Assumes that whatever is passing us these pairs has already parsed for `key=value` and found no issue there.
///
/// Also returns a list of errors for outputting error information.
pub fn table_from_values(values: Vec<(String, String)>) -> (Table, Vec<String>) {
    let mut errors = Vec::new();

    let mut out_table = Table::new();
    for (key, value) in values {
        // Check if this is a value that can be parsed correctly, otherwise we need to add string quotes:
        let val_str = if toml::from_str::<toml::Value>(&value).is_err() {
            format!(r#""{}""#, value)
        } else {
            value.clone()
        };

        let some_table = toml::from_str::<Table>(&format!("{key}={val_str}"));
        if let Ok(t) = some_table {
            merge_config(&mut out_table, t);
        } else {
            errors.push(format!(
                "Could not read {key}={value}: {}",
                some_table.unwrap_err()
            ));
        }
    }

    (out_table, errors)
}

pub(crate) fn table_from_attrs(
    config_attrs: Vec<DiplomatBackendConfigAttr>,
) -> (Table, Vec<String>) {
    let mut values = Vec::new();

    for config in config_attrs {
        for key_value in config.key_value_pairs {
            // Coerce the two into something table_from_values understands:
            values.push((key_value.key, key_value.value));
        }
    }

    table_from_values(values)
}

pub(crate) fn find_top_level_attr(module_items: Vec<syn::Item>) -> Vec<DiplomatBackendConfigAttr> {
    let attrs = module_items
        .iter()
        .filter_map(|i| match i {
            syn::Item::Struct(s) => Some(s.attrs.clone()),
            syn::Item::Impl(i) => Some(i.attrs.clone()),
            syn::Item::Mod(m) => Some(m.attrs.clone()),
            _ => None,
        })
        .filter_map(|attrs| {
            let attrs = Attrs::from(attrs.as_slice());
            if !attrs.config_attrs.is_empty() {
                return Some(attrs.config_attrs);
            }
            None
        });

    let mut out_config = Vec::new();

    for mut a in attrs {
        out_config.append(&mut a);
    }

    out_config
}

/// Intended for use in [`crate::gen`]. Given a specific language's attributes, return a new table that overrides any base values from [`SharedConfig`].
pub(crate) fn set_overrides(parent_table : &Table, language : &str) -> Table {
    let mut overridden_table = parent_table.clone();

    if let Some(v) = parent_table.get(language) {
        let t = v.as_table().expect("Expected language config to be table.");
        // Right now just I'm just assuming all of `SharedConfig` will always be top-level values (never nested in sub-tables). If that changes, feel free to change this.
        for (key, val) in t {
            if SharedConfig::overrides_shared(&key) {
                overridden_table.insert(key.clone(), val.clone());
            }
        }
    }

    overridden_table
}