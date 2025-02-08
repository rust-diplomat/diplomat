use diplomat_core::ast::Attrs;
use serde::{Deserialize, Serialize};
use toml::value::Table;

use crate::{demo_gen::DemoConfig, kotlin::KotlinConfig};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct SharedConfig {
    pub lib_name: Option<String>,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    pub shared_config: SharedConfig,
    #[serde(rename = "kotlin")]
    pub kotlin_config: Option<KotlinConfig>,
    #[serde(rename = "demo-gen")]
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
                    t,
                );
            }
            // Otherwise just overwrite whatever's already there:
            _ => {
                base.insert(key, val);
            },
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
            errors.push(format!("Could not read {key}={value}: {}", some_table.unwrap_err()));
        }
    }

    (out_table, errors)
}


pub(crate) fn table_from_attrs(attrs : Attrs) -> (Table, Vec<String>) {
    let mut values = Vec::new();

    for config in attrs.config_attrs {
        for key_value in config.key_value_pairs {
            // Coerce the two into something table_from_values understands:
            values.push((key_value.key, key_value.value));
        }
    }

    table_from_values(values)
}