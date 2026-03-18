mod common;

use common::{emulator_data_with, telemetry};
use liquidcan::{payloads, CanMessage};
use ECUEmulator::message_handling::build_telemetry_group_updates;

#[test]
fn telemetry_group_update_orders_values_by_name() {
    let telemetry_values = vec![
        telemetry("b", 2, payloads::CanDataType::UInt16),
        telemetry("a", 1, payloads::CanDataType::UInt16),
    ];
    let data = emulator_data_with(Some(telemetry_values), None);

    let updates = build_telemetry_group_updates(&data);
    assert_eq!(updates.len(), 1);

    let CanMessage::TelemetryGroupUpdate { payload } = &updates[0] else {
        panic!("Expected TelemetryGroupUpdate");
    };

    let mut unpacked = payload
        .values
        .unpack([payloads::CanDataType::UInt16, payloads::CanDataType::UInt16].into_iter())
        .map(|val| val.expect("unpack should succeed"))
        .collect::<Vec<_>>();

    assert_eq!(unpacked.remove(0), payloads::CanDataValue::UInt16(1));
    assert_eq!(unpacked.remove(0), payloads::CanDataValue::UInt16(2));
}

#[test]
fn telemetry_group_update_splits_by_payload_size() {
    let telemetry_values: Vec<_> = (1..=16)
        .map(|idx| {
            telemetry(
                &format!("t{:02}", idx),
                idx as u32,
                payloads::CanDataType::UInt32,
            )
        })
        .collect();
    let data = emulator_data_with(Some(telemetry_values), None);

    let updates = build_telemetry_group_updates(&data);
    assert_eq!(updates.len(), 2);
}
