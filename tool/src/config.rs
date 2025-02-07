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
    pub demo_gen_config: Option<DemoConfig>
}

fn merge_subconfig(base : &mut Table, other : Table) {
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
            },
            _ => {
                base.insert(key, val);
            }
        };
    }
}

pub fn merge_config(base : &mut Table, other : Table) {
    for (key, subconfig) in other {
        if !base.contains_key(&key) {
            base.insert(key, subconfig);
            continue;
        }

        match subconfig {
            toml::Value::Table(t) => {
                merge_subconfig(base.get_mut(&key).unwrap().as_table_mut().expect("Expected a table of values in base config."), t);
            },
            _ => panic!("Expected a table of values for configuration, got {subconfig:?}")
        }
    }
}

pub fn table_from_values(values : Vec<String>) -> (Table, Vec<String>) {
    let mut errors = Vec::new();

    let mut out_table = Table::new();
    for v in values {
        let split_opt = v.split_once("=");
        if let Some((key, val)) = split_opt {
            
            // Check if this is a value that can be parsed correctly, otherwise we need to add string quotes:
            let val_str = if let Err(_) = toml::from_str::<toml::Value>(&v) {
                format!(r#""{}""#, val)
            } else {
                val.to_string()
            };

            let some_table = toml::from_str::<Table>(&format!("{key}={val_str}"));
            if let Ok(t) = some_table {
                merge_config(&mut out_table, t);
            } else {
                errors.push(format!("Could not read {v}: {}", some_table.unwrap_err()));
            }
        } else {
            errors.push(format!("Could not read {v}, expected ="));
        }
    }
    
    (out_table, errors)
}