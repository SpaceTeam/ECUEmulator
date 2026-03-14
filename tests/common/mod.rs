use liquidcan::payloads;
use ECUEmulator::config::config_representation::{EmulatorData, Parameter, TelemetryValue};

pub fn telemetry(name: &str, value: u32, datatype: payloads::CanDataType) -> TelemetryValue {
    TelemetryValue {
        name: name.to_string(),
        value,
        datatype,
    }
}

pub fn parameter(
    name: &str,
    value: u32,
    datatype: payloads::CanDataType,
    locked: bool,
) -> Parameter {
    Parameter {
        name: name.to_string(),
        value,
        locked,
        datatype,
    }
}

pub fn emulator_data_with(
    telemetry_values: Option<Vec<TelemetryValue>>,
    parameters: Option<Vec<Parameter>>,
) -> EmulatorData {
    EmulatorData {
        node_id: 1,
        frequency: 100,
        firmware_hash: 0x123,
        liquid_hash: 0x456,
        device_name: "ECUEmulatorTest".to_string(),
        telemetry_values,
        parameters,
    }
}
