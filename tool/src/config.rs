use serde::{Serialize, Deserialize};

use crate::{demo_gen::DemoConfig, kotlin::KotlinConfig};

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
	pub kotlin_config : Option<KotlinConfig>,
	pub demo_gen_config : Option<DemoConfig>
}