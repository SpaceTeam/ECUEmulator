use crate::config::channels::generic_channel_config::GenericChannelConfig;
use crate::config::serde_binary_deserialize::{deserialize_prefixed_u32, deserialize_prefixed_u8};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Variable {
    #[serde(deserialize_with = "deserialize_prefixed_u8")]
    pub id: u8,
    #[serde(deserialize_with = "deserialize_prefixed_u32")]
    pub value: u32,
}
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EmulatorConfig {
    pub general: GeneralSettings,
    pub generic_channel: Option<GenericChannelConfig>,
}

#[derive(Deserialize)]
pub struct GeneralSettings {
    #[serde(deserialize_with = "deserialize_prefixed_u8")]
    pub can_id: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    use config::Config;
    #[test]
    pub fn test_load_config() {
        let config = r#"
    [General]
can_id = 0
[GenericChannel]
    Variables = [
        {id=1, value = 5}
    ]


    [GenericChannel.GenericReqData]
    channel_mask = "0b11111111111"
    data = "0x12387168743618723648761283648"

    [GenericChannel.GenericRequestNodeInfo]
    firmware_version = "0x0110101"
    channel_mask = "0b11111111111"
    channel_type = "0x01"
    data = "0xFFEEAABBCCDDEEFF"

    [GenericChannel.GenericReqFlashClear]
    status = 1
    "#;

        let config = Config::builder()
            .add_source(config::File::from_str(config, config::FileFormat::Toml))
            .build()
            .unwrap();

        let emu_config: EmulatorConfig = config.try_deserialize().unwrap();
    }
}
