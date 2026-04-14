use crate::config::config_representation::EmulatorData;
use anyhow::{Context, Result};
use config::{Config, File};
use std::env;

pub fn load_config(path: &str) -> Result<EmulatorData> {
    let config = Config::builder()
        .add_source(File::with_name(path))
        .build()?;

    let mut emulator_data: EmulatorData = config
        .try_deserialize()
        .with_context(|| format!("Failed to deserialize config from {}", path))?;

    if emulator_data.node_id > 31 {
        return Err(anyhow::anyhow!(
            "Invalid node_id {} (must be <= 31)",
            emulator_data.node_id
        ));
    }

    // Allow overriding the SocketCAN interface from the environment.
    // Useful for containers where the config is bind-mounted read-only.
    if let Ok(iface) = env::var("CAN_INTERFACE") {
        let iface = iface.trim().to_string();
        if !iface.is_empty() {
            emulator_data.can_interface = iface;
        }
    }

    Ok(emulator_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::{Mutex, OnceLock};

    static ENV_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

    fn write_temp_config(contents: &str) -> String {
        let mut p = std::env::temp_dir();
        let uniq = format!(
            "ecuemulator_test_{}_{}.toml",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );
        p.push(uniq);
        fs::write(&p, contents).expect("write temp config");
        p.to_string_lossy().to_string()
    }

    const SAMPLE_CONFIG: &str = r#"node_id = 1
frequency = 100
firmware_hash = "0x123"
can_interface = "vcan0"
liquid_hash = "0x123"
device_name = "Emulator1"

[TelemetryValues]
  [TelemetryValues.tel1]
  value = 0x12345678
  datatype = "UInt32"

[Parameters]
  [Parameters.Parameter1]
  value = 0xABAC0
  locked = false
  datatype = "UInt32"
"#;

    #[test]
    fn can_interface_can_be_overridden_by_env_var() {
        let _guard = ENV_MUTEX
            .get_or_init(|| Mutex::new(()))
            .lock()
            .unwrap_or_else(|e| e.into_inner());

        let original = env::var("CAN_INTERFACE").ok();

        let path = write_temp_config(SAMPLE_CONFIG);

        // Override
        env::set_var("CAN_INTERFACE", "vcan42");
        let cfg = load_config(&path).expect("config should load");
        assert_eq!(cfg.can_interface, "vcan42");

        // Cleanup temp file
        let _ = fs::remove_file(&path);

        // Restore env
        match original {
            Some(v) => env::set_var("CAN_INTERFACE", v),
            None => env::remove_var("CAN_INTERFACE"),
        }
    }

    #[test]
    fn empty_can_interface_env_var_is_ignored() {
        let _guard = ENV_MUTEX
            .get_or_init(|| Mutex::new(()))
            .lock()
            .unwrap_or_else(|e| e.into_inner());

        let original = env::var("CAN_INTERFACE").ok();

        let path = write_temp_config(SAMPLE_CONFIG);

        env::set_var("CAN_INTERFACE", "   ");
        let cfg = load_config(&path).expect("config should load");
        assert_eq!(cfg.can_interface, "vcan0");

        let _ = fs::remove_file(&path);

        match original {
            Some(v) => env::set_var("CAN_INTERFACE", v),
            None => env::remove_var("CAN_INTERFACE"),
        }
    }
}
