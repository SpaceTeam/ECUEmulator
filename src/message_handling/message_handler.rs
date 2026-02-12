use crate::config::config_representation::EmulatorData;
use crate::config::config_representation::Parameter;
use crate::protocol::payloads;
use crate::protocol::CanMessage;

fn find_parameter_and_map<F>(
    parameters: &mut Option<Vec<Parameter>>,
    parameter_id: u8,
    func: F,
) -> Option<CanMessage>
where
    F: FnOnce(&mut Parameter) -> CanMessage,
{
    let Some(parameters_unwrapped) = parameters.as_mut() else {
        return Some(CanMessage::ParameterSetConfirmation {
            payload: payloads::ParameterSetConfirmationPayload {
                parameter_id: parameter_id,
                status: payloads::ParameterSetStatus::InvalidParameterID,
                value: [0; 61],
            },
        });
    };
    Some(
        parameters_unwrapped
            .iter_mut()
            .find(|p| p.name == parameter_id.to_string())
            .map_or(
                //Nothing found => invalid parameter ID
                CanMessage::ParameterSetConfirmation {
                    payload: payloads::ParameterSetConfirmationPayload {
                        parameter_id: parameter_id,
                        status: payloads::ParameterSetStatus::InvalidParameterID,
                        value: [0; 61],
                    },
                },
                //Parameter found, check if it's locked and update value if not
                |param| func(param),
            ),
    )
}

pub fn handle_message(msg: &CanMessage, emulator_data: &mut EmulatorData) -> Option<CanMessage> {
    match msg {
        CanMessage::NodeInfoReq => Some(CanMessage::NodeInfoAnnouncement {
            payload: payloads::NodeInfoResPayload {
                tel_count: emulator_data.telemetry_values.iter().len().try_into().expect("Maxmimum telemetry values exceeded(255)"),
                par_count: emulator_data.parameters.iter().len().try_into().expect("Maxmimum telemetry values exceeded(255)"),
                firmware_hash: emulator_data.firmware_hash,
                liquid_hash: emulator_data.liquid_hash,
                device_name: <[u8; 53]>::try_from(emulator_data.device_name.as_bytes()).expect("Device name too long (max 53 bytes); shouldn't happen due to config deserialization"),
            },
        }),
        CanMessage::HeartbeatReq { payload } => Some(CanMessage::HeartbeatRes {
            payload: payloads::HeartbeatPayload {
                counter: payload.counter + 1,
            },
        }),
        CanMessage::ParameterSetReq { payload } => {
            find_parameter_and_map(&mut emulator_data.parameters, payload.parameter_id, |param|
                if param.locked {
                    CanMessage::ParameterSetConfirmation {
                        payload: payloads::ParameterSetConfirmationPayload {
                            parameter_id: payload.parameter_id,
                            status: payloads::ParameterSetStatus::ParameterLocked,
                            value: [0; 61],
                        },
                    }
                } else {
                    param.value = u32::from_le_bytes(payload.value[0..4].try_into().expect("Value must be at least 4 bytes for u32"));
                    CanMessage::ParameterSetConfirmation {
                        payload: payloads::ParameterSetConfirmationPayload {
                            parameter_id: payload.parameter_id,
                            status: payloads::ParameterSetStatus::Success,
                            value: payload.value,
                        },
                    }
                })
        },
        CanMessage::ParameterSetConfirmation { payload: _payload } => None,
        CanMessage::ParameterSetLockReq { payload } => {
            find_parameter_and_map(&mut emulator_data.parameters, payload.parameter_id, |param| {
                //TODO : we should prevent unlocking from nodes which didn't lock the parameter
                param.locked = payload.parameter_lock == payloads::ParameterLockStatus::Locked;
                CanMessage::ParameterSetLockConfirmation {
                    payload: payloads::ParameterSetLockPayload {
                        parameter_id: payload.parameter_id,
                        parameter_lock: payload.parameter_lock,
                    },
                }
            })
        }
        CanMessage::FieldGetReq { payload } => Some(CanMessage::FieldGetRes {
            payload: payloads::FieldGetResPayload {
                field_id: todo!(),
                value: todo!(),
            },
        }),
        CanMessage::FieldGetRes { payload } => None,
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
