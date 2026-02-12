use crate::config::serde_deserializer::deserialize_parameters;
use crate::config::serde_deserializer::deserialize_prefixed_u32;
use crate::config::serde_deserializer::deserialize_value_or_u32;
use crate::config::serde_deserializer::deserialize_variables;
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
pub struct Variable {
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
pub struct EmulatorConfig {
    pub node_id: u32,
    pub frequency: u32,
    #[serde(rename = "Variables")]
    #[serde(deserialize_with = "deserialize_variables")]
    pub variables: Option<Vec<Variable>>,
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
        let config = r#"
node_id = 0
frequency = 100
[Variables]
   [Variables.Variable1]
    value = 0x12345678
    datatype = "UInt32"
    [Variables.Variable2]
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

        let emu_config: EmulatorConfig = config.try_deserialize().unwrap();

        assert_eq!(emu_config.node_id, 0);
        assert_eq!(emu_config.frequency, 100);

        let variables = emu_config.variables.expect("Variables should be present");
        assert_eq!(variables.len(), 2);

        let var1 = variables
            .iter()
            .find(|v| v.name == "Variable1")
            .expect("Variable1 should exist");
        assert_eq!(var1.value, 0x12345678);

        let var2 = variables
            .iter()
            .find(|v| v.name == "Variable2")
            .expect("Variable2 should exist");
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
