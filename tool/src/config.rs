use serde::{Serialize, Deserialize};

use crate::{demo_gen::DemoConfig, kotlin::KotlinConfig};

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
	#[serde(rename="kotlin")]
	pub kotlin_config : Option<KotlinConfig>,
	#[serde(rename="demo-gen")]
	pub demo_gen_config : Option<DemoConfig>
}