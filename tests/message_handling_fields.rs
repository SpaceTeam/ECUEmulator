mod common;

use common::{emulator_data_with, parameter, telemetry};
use liquidcan::{payloads, CanMessage};
use ECUEmulator::message_handling::handle_message;

#[test]
fn field_get_req_returns_telemetry_value() {
    let telemetry_values = vec![telemetry("t1", 0xAA, payloads::CanDataType::UInt8)];
    let mut data = emulator_data_with(Some(telemetry_values), None);

    let request = CanMessage::FieldGetReq {
        payload: payloads::FieldGetReqPayload { field_id: 0x81 },
    };

    let responses = handle_message(&request, &mut data);
    let CanMessage::FieldGetRes { payload } = &responses[0] else {
        panic!("Expected FieldGetRes");
    };

    assert_eq!(payload.field_status, payloads::FieldStatus::Ok);
    assert_eq!(payload.value, payloads::CanDataValue::UInt8(0xAA));
}

#[test]
fn field_get_req_returns_parameter_value() {
    let parameters = vec![parameter("p1", 0x10, payloads::CanDataType::UInt8, false)];
    let mut data = emulator_data_with(None, Some(parameters));

    let request = CanMessage::FieldGetReq {
        payload: payloads::FieldGetReqPayload { field_id: 1 },
    };

    let responses = handle_message(&request, &mut data);
    let CanMessage::FieldGetRes { payload } = &responses[0] else {
        panic!("Expected FieldGetRes");
    };

    assert_eq!(payload.field_status, payloads::FieldStatus::Ok);
    assert_eq!(payload.value, payloads::CanDataValue::UInt8(0x10));
}

#[test]
fn field_get_req_not_found_returns_empty_raw() {
    let mut data = emulator_data_with(None, None);

    let request = CanMessage::FieldGetReq {
        payload: payloads::FieldGetReqPayload { field_id: 1 },
    };

    let responses = handle_message(&request, &mut data);
    let CanMessage::FieldGetRes { payload } = &responses[0] else {
        panic!("Expected FieldGetRes");
    };

    assert_eq!(payload.field_status, payloads::FieldStatus::NotFound);
    assert_eq!(payload.value, payloads::CanDataValue::Raw(Vec::new()));
}

#[test]
fn field_id_lookup_prefers_telemetry_when_names_match() {
    let telemetry_values = vec![telemetry("dup", 1, payloads::CanDataType::UInt8)];
    let parameters = vec![parameter("dup", 2, payloads::CanDataType::UInt8, false)];
    let mut data = emulator_data_with(Some(telemetry_values), Some(parameters));

    let request = CanMessage::FieldIDLookupReq {
        payload: payloads::FieldIDLookupReqPayload {
            field_name: payloads::CanString::<61>::try_from("dup").unwrap(),
        },
    };

    let responses = handle_message(&request, &mut data);
    let CanMessage::FieldIDLookupRes { payload } = &responses[0] else {
        panic!("Expected FieldIDLookupRes");
    };

    assert_eq!(payload.field_status, payloads::FieldStatus::Ok);
    assert_eq!(payload.field_id, 0x81);
}

#[test]
fn field_id_lookup_not_found_returns_not_found_status() {
    let mut data = emulator_data_with(None, None);

    let request = CanMessage::FieldIDLookupReq {
        payload: payloads::FieldIDLookupReqPayload {
            field_name: payloads::CanString::<61>::try_from("missing").unwrap(),
        },
    };

    let responses = handle_message(&request, &mut data);
    let CanMessage::FieldIDLookupRes { payload } = &responses[0] else {
        panic!("Expected FieldIDLookupRes");
    };

    assert_eq!(payload.field_status, payloads::FieldStatus::NotFound);
    assert_eq!(payload.field_id, 0);
}
