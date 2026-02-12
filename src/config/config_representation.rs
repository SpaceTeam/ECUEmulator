use crate::config::serde_deserializer::deserialize_parameters;
use crate::config::serde_deserializer::deserialize_prefixed_u32;
use crate::config::serde_deserializer::deserialize_telemetry;
use crate::config::serde_deserializer::deserialize_value_or_u32;
use crate::config::serde_deserializer::max_bytes;
use crate::protocol::payloads::CanDataType;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(remote = "CanDataType")]
enum DataType {
    Float32 = 0,
    Int32 = 1,
    Int16 = 2,
    Int8 = 3,
    UInt32 = 4,
    UInt16 = 5,
    UInt8 = 6,
    Boolean = 7,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TelemetryValue {
    #[serde(skip)]
    pub name: String,
    #[serde(deserialize_with = "deserialize_prefixed_u32")]
    pub value: u32,
    #[serde(with = "DataType")]
    pub datatype: CanDataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Parameter {
    #[serde(skip)]
    pub name: String,
    #[serde(deserialize_with = "deserialize_value_or_u32")]
    pub value: u32,
    pub locked: bool,
    #[serde(with = "DataType")]
    pub datatype: CanDataType,
}

#[derive(Deserialize, Debug)]
pub struct EmulatorData {
    pub node_id: u32,
    pub frequency: u32,
    #[serde(deserialize_with = "deserialize_value_or_u32")]
    pub firmware_hash: u32,
    #[serde(deserialize_with = "deserialize_value_or_u32")]
    pub liquid_hash: u32,
    #[serde(deserialize_with = "max_bytes::deserialize::<53,_>")]
    pub device_name: String,
    #[serde(rename = "TelemetryValues")]
    #[serde(deserialize_with = "deserialize_telemetry")]
    pub telemetry_values: Option<Vec<TelemetryValue>>,
    #[serde(rename = "Parameters")]
    #[serde(deserialize_with = "deserialize_parameters")]
    pub parameters: Option<Vec<Parameter>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use config::Config;
    #[test]
    pub fn test_load_config() {
        let config = r#"node_id = 0
frequency = 100
firmware_hash = "0x123"
liquid_hash = "0x123"
device_name = "Emulator1"
[TelemetryValues]
   [TelemetryValues.tel1]
    value = 0x12345678
    datatype = "UInt32"
    [TelemetryValues.tel2]
    value = 0x12345678
    datatype = "UInt32"

[Parameters]
    [Parameters.Parameter1]
     value = 0xABAC0
     locked = false
     datatype = "UInt32"

     [Parameters.Parameter2]
     value = false
     datatype = "Boolean"
     locked = true"#;

        let config = Config::builder()
            .add_source(config::File::from_str(config, config::FileFormat::Toml))
            .build()
            .unwrap();

        let emu_config: EmulatorData = config.try_deserialize().unwrap();

        assert_eq!(emu_config.node_id, 0);
        assert_eq!(emu_config.frequency, 100);

        let telemetry_values = emu_config
            .telemetry_values
            .expect("Telemetry Values should be present");
        assert_eq!(telemetry_values.len(), 2);

        let var1 = telemetry_values
            .iter()
            .find(|v| v.name == "tel1")
            .expect("tel1 should exist");
        assert_eq!(var1.value, 0x12345678);

        let var2 = telemetry_values
            .iter()
            .find(|v| v.name == "tel2")
            .expect("tel2 should exist");
        assert_eq!(var2.value, 0x12345678);

        let parameters = emu_config.parameters.expect("Parameters should be present");
        assert_eq!(parameters.len(), 2);

        let param1 = parameters
            .iter()
            .find(|p| p.name == "Parameter1")
            .expect("Parameter1 should exist");
        assert_eq!(param1.value, 0xABAC0);
        assert_eq!(param1.locked, false);

        let param2 = parameters
            .iter()
            .find(|p| p.name == "Parameter2")
            .expect("Parameter2 should exist");
        assert_eq!(param2.value, 0); // false -> 0
        assert_eq!(param2.locked, true);
    }
}
