mod common;

use common::{emulator_data_with, parameter, telemetry};
use liquidcan::{payloads, CanMessage};
use ECUEmulator::message_handling::handle_message;

#[test]
fn node_info_req_emits_full_registration_flow() {
    let telemetry_values = vec![
        telemetry("beta", 10, payloads::CanDataType::UInt16),
        telemetry("alpha", 20, payloads::CanDataType::UInt8),
    ];
    let parameters = vec![
        parameter("p2", 100, payloads::CanDataType::UInt32, false),
        parameter("p1", 200, payloads::CanDataType::Boolean, true),
    ];
    let mut data = emulator_data_with(Some(telemetry_values), Some(parameters));

    let responses = handle_message(&CanMessage::NodeInfoReq, &mut data);

    assert!(!responses.is_empty());
    assert!(matches!(
        responses[0],
        CanMessage::NodeInfoAnnouncement { .. }
    ));

    let telemetry_regs: Vec<_> = responses
        .iter()
        .filter_map(|msg| match msg {
            CanMessage::TelemetryValueRegistration { payload } => Some(payload),
            _ => None,
        })
        .collect();
    assert_eq!(telemetry_regs.len(), 2);

    let names: Vec<String> = telemetry_regs
        .iter()
        .map(|payload| payload.field_name.clone().into())
        .collect();
    assert_eq!(names, vec!["alpha", "beta"]);

    let ids: Vec<u8> = telemetry_regs
        .iter()
        .map(|payload| payload.field_id)
        .collect();
    assert_eq!(ids, vec![0x81, 0x82]);

    let parameter_regs: Vec<_> = responses
        .iter()
        .filter_map(|msg| match msg {
            CanMessage::ParameterRegistration { payload } => Some(payload),
            _ => None,
        })
        .collect();
    assert_eq!(parameter_regs.len(), 2);

    let param_names: Vec<String> = parameter_regs
        .iter()
        .map(|payload| payload.field_name.clone().into())
        .collect();
    assert_eq!(param_names, vec!["p1", "p2"]);

    let param_ids: Vec<u8> = parameter_regs
        .iter()
        .map(|payload| payload.field_id)
        .collect();
    assert_eq!(param_ids, vec![1, 2]);

    let group_defs: Vec<_> = responses
        .iter()
        .filter_map(|msg| match msg {
            CanMessage::TelemetryGroupDefinition { payload } => Some(payload),
            _ => None,
        })
        .collect();
    assert_eq!(group_defs.len(), 1);

    let field_ids_slice: &[u8] = (&group_defs[0].field_ids).into();
    let field_ids: Vec<u8> = field_ids_slice.to_vec();
    assert_eq!(field_ids, vec![0x81, 0x82]);
}

#[test]
fn telemetry_group_definition_splits_by_payload_size() {
    let telemetry_values: Vec<_> = (1..=16)
        .map(|idx| {
            telemetry(
                &format!("t{:02}", idx),
                idx as u32,
                payloads::CanDataType::UInt32,
            )
        })
        .collect();
    let mut data = emulator_data_with(Some(telemetry_values), None);

    let responses = handle_message(&CanMessage::NodeInfoReq, &mut data);
    let group_defs: Vec<_> = responses
        .iter()
        .filter_map(|msg| match msg {
            CanMessage::TelemetryGroupDefinition { payload } => Some(payload),
            _ => None,
        })
        .collect();

    assert_eq!(group_defs.len(), 2);
    assert_eq!(group_defs[0].group_id, 1);
    assert_eq!(group_defs[1].group_id, 2);

    let group1_ids_slice: &[u8] = (&group_defs[0].field_ids).into();
    let group2_ids_slice: &[u8] = (&group_defs[1].field_ids).into();
    let group1_ids: Vec<u8> = group1_ids_slice.to_vec();
    let group2_ids: Vec<u8> = group2_ids_slice.to_vec();

    assert_eq!(group1_ids.len(), 15);
    assert_eq!(group2_ids.len(), 1);
    assert_eq!(group1_ids[0], 0x81);
    assert_eq!(group2_ids[0], 0x90);
}
