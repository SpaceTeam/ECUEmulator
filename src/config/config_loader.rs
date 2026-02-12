use crate::config::config_representation::EmulatorData;
use anyhow::{Context, Result};
use config::{Config, File};

pub fn load_config(path: &str) -> Result<EmulatorData> {
    let config = Config::builder()
        .add_source(File::with_name(path))
        .build()?;

    config
        .try_deserialize()
        .with_context(|| format!("Failed to deserialize config from {}", path))
}
