use serde::{Serialize, Deserialize};

use crate::{demo_gen::DemoConfig, kotlin::KotlinConfig};

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Config {
	pub demo_gen_config : DemoConfig
}