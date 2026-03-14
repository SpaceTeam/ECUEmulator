mod common;

use common::{emulator_data_with, parameter};
use liquidcan::{payloads, CanMessage};
use ECUEmulator::message_handling::handle_message;

#[test]
fn parameter_set_req_updates_value_when_unlocked() {
    let parameters = vec![parameter("p1", 10, payloads::CanDataType::UInt16, false)];
    let mut data = emulator_data_with(None, Some(parameters));

    let request = CanMessage::ParameterSetReq {
        payload: payloads::ParameterSetReqPayload {
            parameter_id: 1,
            value: payloads::CanDataValue::UInt16(42),
        },
    };

    let responses = handle_message(&request, &mut data);
    assert_eq!(responses.len(), 1);

    let CanMessage::ParameterSetConfirmation { payload } = &responses[0] else {
        panic!("Expected ParameterSetConfirmation");
    };

    assert_eq!(payload.status, payloads::ParameterSetStatus::Success);
    assert_eq!(payload.value, payloads::CanDataValue::UInt16(42));
}

#[test]
fn parameter_set_req_respects_lock() {
    let parameters = vec![parameter("p1", 99, payloads::CanDataType::UInt8, true)];
    let mut data = emulator_data_with(None, Some(parameters));

    let request = CanMessage::ParameterSetReq {
        payload: payloads::ParameterSetReqPayload {
            parameter_id: 1,
            value: payloads::CanDataValue::UInt8(1),
        },
    };

    let responses = handle_message(&request, &mut data);
    let CanMessage::ParameterSetConfirmation { payload } = &responses[0] else {
        panic!("Expected ParameterSetConfirmation");
    };

    assert_eq!(
        payload.status,
        payloads::ParameterSetStatus::ParameterLocked
    );
    assert_eq!(payload.value, payloads::CanDataValue::UInt8(99));
}

#[test]
fn parameter_set_req_invalid_id_returns_invalid_status() {
    let parameters = vec![parameter("p1", 10, payloads::CanDataType::UInt32, false)];
    let mut data = emulator_data_with(None, Some(parameters));

    let request = CanMessage::ParameterSetReq {
        payload: payloads::ParameterSetReqPayload {
            parameter_id: 7,
            value: payloads::CanDataValue::UInt32(11),
        },
    };

    let responses = handle_message(&request, &mut data);
    let CanMessage::ParameterSetConfirmation { payload } = &responses[0] else {
        panic!("Expected ParameterSetConfirmation");
    };

    assert_eq!(
        payload.status,
        payloads::ParameterSetStatus::InvalidParameterID
    );
}

#[test]
fn parameter_set_req_invalid_payload_returns_invalid_status() {
    let parameters = vec![parameter("p1", 10, payloads::CanDataType::UInt32, false)];
    let mut data = emulator_data_with(None, Some(parameters));

    let request = CanMessage::ParameterSetReq {
        payload: payloads::ParameterSetReqPayload {
            parameter_id: 1,
            value: payloads::CanDataValue::Raw(vec![0x01, 0x02]),
        },
    };

    let responses = handle_message(&request, &mut data);
    let CanMessage::ParameterSetConfirmation { payload } = &responses[0] else {
        panic!("Expected ParameterSetConfirmation");
    };

    assert_eq!(
        payload.status,
        payloads::ParameterSetStatus::InvalidParameterID
    );
}

#[test]
fn parameter_set_lock_updates_state() {
    let parameters = vec![parameter("p1", 10, payloads::CanDataType::UInt8, false)];
    let mut data = emulator_data_with(None, Some(parameters));

    let request = CanMessage::ParameterSetLockReq {
        payload: payloads::ParameterSetLockPayload {
            parameter_id: 1,
            parameter_lock: payloads::ParameterLockStatus::Locked,
        },
    };

    let responses = handle_message(&request, &mut data);
    let CanMessage::ParameterSetLockConfirmation { payload } = &responses[0] else {
        panic!("Expected ParameterSetLockConfirmation");
    };

    assert_eq!(payload.field_status, payloads::FieldStatus::Ok);
    assert_eq!(
        payload.parameter_lock,
        payloads::ParameterLockStatus::Locked
    );

    let updated = data.parameters.as_ref().unwrap();
    assert!(updated[0].locked);
}
