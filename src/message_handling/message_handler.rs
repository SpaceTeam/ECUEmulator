use crate::config::state_storage::StateStorage;
use liquidcan::payloads;
use liquidcan::CanMessage;

pub fn handle_message(msg: &CanMessage, _state: &mut StateStorage) -> Option<CanMessage> {
    match msg {
        CanMessage::NodeInfoReq => Some(CanMessage::NodeInfoAnnouncement {
            payload: payloads::NodeInfoResPayload {
                tel_count: todo!(),
                par_count: todo!(),
                firmware_hash: todo!(),
                liquid_hash: todo!(),
                device_name: todo!(),
            },
        }),
        CanMessage::HeartbeatReq { payload } => Some(CanMessage::HeartbeatRes {
            payload: payloads::HeartbeatPayload {
                counter: payload.counter + 1,
            },
        }),
        CanMessage::ParameterSetReq { payload: _payload } => {
            Some(CanMessage::ParameterSetConfirmation {
                payload: payloads::ParameterSetConfirmationPayload {
                    parameter_id: todo!(),
                    status: todo!(),
                    value: todo!(),
                },
            })
        }
        CanMessage::ParameterSetConfirmation { payload: _payload } => None,
        CanMessage::ParameterSetLockReq { payload: _payload } => {
            Some(CanMessage::ParameterSetLockConfirmation {
                payload: payloads::ParameterSetLockPayload {
                    parameter_id: todo!(),
                    parameter_lock: todo!(),
                },
            })
        }
        CanMessage::FieldGetReq { payload: _payload } => Some(CanMessage::FieldGetRes {
            payload: payloads::FieldGetResPayload {
                field_id: todo!(),
                value: todo!(),
            },
        }),
        CanMessage::FieldGetRes { payload: _payload } => None,
        CanMessage::FieldIDLookupReq { payload } => Some(CanMessage::FieldIDLookupRes {
            payload: payloads::FieldIDLookupResPayload {
                field_id: todo!(),
                field_type: todo!(),
            },
        }),
        CanMessage::FieldIDLookupRes { payload } => todo!(),
        _ => None,
    }
}
