use liquidcan::{payloads, CanMessage};
use ECUEmulator::message_handling::{build_status_message, handle_message, StatusMessageKind};

#[test]
fn heartbeat_req_increments_counter() {
    let mut data = ECUEmulator::config::config_representation::EmulatorData {
        node_id: 1,
        frequency: 100,
        firmware_hash: 0,
        liquid_hash: 0,
        device_name: "ECUEmulatorTest".to_string(),
        telemetry_values: None,
        parameters: None,
    };

    let request = CanMessage::HeartbeatReq {
        payload: payloads::HeartbeatPayload { counter: 41 },
    };

    let responses = handle_message(&request, &mut data);
    let CanMessage::HeartbeatRes { payload } = &responses[0] else {
        panic!("Expected HeartbeatRes");
    };

    assert_eq!(payload.counter, 42);
}

#[test]
fn build_status_message_variants() {
    let info = build_status_message(StatusMessageKind::Info, "all good");
    let warn = build_status_message(StatusMessageKind::Warning, "check config");
    let err = build_status_message(StatusMessageKind::Error, "boom");

    assert!(matches!(info, CanMessage::InfoStatus { .. }));
    assert!(matches!(warn, CanMessage::WarningStatus { .. }));
    assert!(matches!(err, CanMessage::ErrorStatus { .. }));
}
