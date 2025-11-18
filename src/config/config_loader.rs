use crate::config::channel_parser::parse_generic_command_arguments;
use crate::config::state_storage::StateStorage;
use anyhow::{Context, Result};
use std::path::Path;
use toml::Table;

pub fn load_config(path: &Path) -> Result<StateStorage> {
    let config_data = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file {}", path.display()))?;
    let config: Table = toml::from_str(&config_data)
        .with_context(|| format!("Failed to parse config file {}", path.display()))?;

    let mut state_storage = StateStorage::new();
    parse_generic_command_arguments(&config, &mut state_storage)
        .with_context(|| format!("Error parsing config file {}", path.display()))?;

    Ok(state_storage)
}
