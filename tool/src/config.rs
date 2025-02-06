use serde::{Deserialize, Serialize};

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
