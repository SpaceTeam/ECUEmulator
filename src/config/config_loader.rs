use crate::config::config_representation::EmulatorConfig;
use anyhow::{Context, Result};
use config::{Config, File};

pub fn load_config(path: &str) -> Result<EmulatorConfig> {
    let config = Config::builder()
        .add_source(File::with_name(path))
        .build()?;

    config
        .try_deserialize()
        .with_context(|| format!("Failed to deserialize config from {}", path))
}
