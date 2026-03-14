use crate::config::config_representation::EmulatorData;
use crate::config::config_representation::{Parameter, TelemetryValue};
use liquidcan::payloads;
use liquidcan::CanMessage;

const TELEMETRY_ID_BIT: u8 = 0b1000_0000;

fn sorted_parameter_indices(parameters: &[Parameter]) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..parameters.len()).collect();
    indices.sort_by(|&a, &b| parameters[a].name.cmp(&parameters[b].name));
    indices
}

fn sorted_telemetry_indices(telemetry: &[TelemetryValue]) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..telemetry.len()).collect();
    indices.sort_by(|&a, &b| telemetry[a].name.cmp(&telemetry[b].name));
    indices
}

fn parameter_id_for_index(index: usize) -> Option<u8> {
    let id = index.checked_add(1)?;
    if id > 0x7F {
        return None;
    }
    Some(id as u8)
}

fn telemetry_id_for_index(index: usize) -> Option<u8> {
    parameter_id_for_index(index).map(|id| id | TELEMETRY_ID_BIT)
}

fn index_from_field_id(field_id: u8) -> Option<usize> {
    let raw = field_id & 0x7F;
    if raw == 0 {
        return None;
    }
    Some((raw - 1) as usize)
}

fn u32_from_value(value: &payloads::CanDataValue, data_type: payloads::CanDataType) -> Option<u32> {
    let typed = match value {
        payloads::CanDataValue::Raw(_) => value.convert_from_raw(data_type).ok()?,
        _ => value.clone(),
    };
    let v = match typed {
        payloads::CanDataValue::Float32(v) => v.to_bits(),
        payloads::CanDataValue::Int32(v) => v as u32,
        payloads::CanDataValue::Int16(v) => (v as u16) as u32,
        payloads::CanDataValue::Int8(v) => (v as u8) as u32,
        payloads::CanDataValue::UInt32(v) => v,
        payloads::CanDataValue::UInt16(v) => v as u32,
        payloads::CanDataValue::UInt8(v) => v as u32,
        payloads::CanDataValue::Boolean(v) => {
            if v {
                1
            } else {
                0
            }
        }
        payloads::CanDataValue::Raw(_) => return None,
    };
    Some(v)
}

fn value_from_u32(value: u32, data_type: payloads::CanDataType) -> payloads::CanDataValue {
    match data_type {
        payloads::CanDataType::Float32 => payloads::CanDataValue::Float32(f32::from_bits(value)),
        payloads::CanDataType::Int32 => payloads::CanDataValue::Int32(value as i32),
        payloads::CanDataType::Int16 => payloads::CanDataValue::Int16((value as u16) as i16),
        payloads::CanDataType::Int8 => payloads::CanDataValue::Int8((value as u8) as i8),
        payloads::CanDataType::UInt32 => payloads::CanDataValue::UInt32(value),
        payloads::CanDataType::UInt16 => payloads::CanDataValue::UInt16(value as u16),
        payloads::CanDataType::UInt8 => payloads::CanDataValue::UInt8(value as u8),
        payloads::CanDataType::Boolean => payloads::CanDataValue::Boolean(value != 0),
    }
}

fn node_info_announcement(emulator_data: &EmulatorData) -> CanMessage {
    CanMessage::NodeInfoAnnouncement {
        payload: payloads::NodeInfoResPayload {
            tel_count: emulator_data
                .telemetry_values
                .as_ref()
                .map(|vals| vals.len())
                .unwrap_or(0)
                .try_into()
                .expect("Maxmimum telemetry values exceeded(255)"),
            par_count: emulator_data
                .parameters
                .as_ref()
                .map(|vals| vals.len())
                .unwrap_or(0)
                .try_into()
                .expect("Maxmimum telemetry values exceeded(255)"),
            firmware_hash: emulator_data.firmware_hash,
            liquid_hash: emulator_data.liquid_hash,
            device_name: payloads::CanString::<53>::try_from(emulator_data.device_name.as_str())
                .expect("Device name too long (max 53 bytes); shouldn't happen due to config deserialization"),
        },
    }
}

fn telemetry_registrations(telemetry: &[TelemetryValue]) -> Vec<CanMessage> {
    let indices = sorted_telemetry_indices(telemetry);
    indices
        .iter()
        .enumerate()
        .filter_map(|(pos, &idx)| {
            let field_id = telemetry_id_for_index(pos)?;
            let tel = &telemetry[idx];
            let field_name = payloads::CanString::<61>::try_from(tel.name.as_str())
                .expect("Telemetry field name too long (max 61 bytes)");
            Some(CanMessage::TelemetryValueRegistration {
                payload: payloads::FieldRegistrationPayload {
                    field_id,
                    field_type: tel.datatype,
                    field_name,
                },
            })
        })
        .collect()
}

fn parameter_registrations(parameters: &[Parameter]) -> Vec<CanMessage> {
    let indices = sorted_parameter_indices(parameters);
    indices
        .iter()
        .enumerate()
        .filter_map(|(pos, &idx)| {
            let field_id = parameter_id_for_index(pos)?;
            let param = &parameters[idx];
            let field_name = payloads::CanString::<61>::try_from(param.name.as_str())
                .expect("Parameter name too long (max 61 bytes)");
            Some(CanMessage::ParameterRegistration {
                payload: payloads::FieldRegistrationPayload {
                    field_id,
                    field_type: param.datatype,
                    field_name,
                },
            })
        })
        .collect()
}

fn telemetry_group_definitions(telemetry: &[TelemetryValue]) -> Vec<CanMessage> {
    let indices = sorted_telemetry_indices(telemetry);
    let mut groups = Vec::new();
    let mut current_ids: Vec<u8> = Vec::new();
    let mut current_size: usize = 0;
    let mut group_id: u8 = 1;

    for (pos, &idx) in indices.iter().enumerate() {
        let Some(field_id) = telemetry_id_for_index(pos) else {
            continue;
        };
        let tel = &telemetry[idx];
        let field_size = tel.datatype.get_size();

        let would_overflow = current_ids.len() >= 62 || current_size + field_size > 62;
        if would_overflow {
            if !current_ids.is_empty() {
                let field_ids = payloads::NonNullCanBytes::<62>::try_from(current_ids.as_slice())
                    .expect("Telemetry group field IDs must be <= 62 bytes and non-zero");
                groups.push(CanMessage::TelemetryGroupDefinition {
                    payload: payloads::TelemetryGroupDefinitionPayload {
                        group_id,
                        field_ids,
                    },
                });
                group_id = group_id.saturating_add(1);
            }
            current_ids = Vec::new();
            current_size = 0;
        }

        current_ids.push(field_id);
        current_size += field_size;
    }

    if !current_ids.is_empty() {
        let field_ids = payloads::NonNullCanBytes::<62>::try_from(current_ids.as_slice())
            .expect("Telemetry group field IDs must be <= 62 bytes and non-zero");
        groups.push(CanMessage::TelemetryGroupDefinition {
            payload: payloads::TelemetryGroupDefinitionPayload {
                group_id,
                field_ids,
            },
        });
    }

    groups
}

fn registration_flow_messages(emulator_data: &EmulatorData) -> Vec<CanMessage> {
    let mut messages = Vec::new();
    messages.push(node_info_announcement(emulator_data));

    if let Some(telemetry) = emulator_data.telemetry_values.as_ref() {
        messages.extend(telemetry_registrations(telemetry));
    }
    if let Some(parameters) = emulator_data.parameters.as_ref() {
        messages.extend(parameter_registrations(parameters));
    }
    if let Some(telemetry) = emulator_data.telemetry_values.as_ref() {
        messages.extend(telemetry_group_definitions(telemetry));
    }

    messages
}

pub fn handle_message(msg: &CanMessage, emulator_data: &mut EmulatorData) -> Vec<CanMessage> {
    match msg {
        CanMessage::NodeInfoReq => registration_flow_messages(emulator_data),
        CanMessage::HeartbeatReq { payload } => vec![CanMessage::HeartbeatRes {
            payload: payloads::HeartbeatPayload {
                counter: payload.counter + 1,
            },
        }],
        CanMessage::ParameterSetReq { payload } => {
            let Some(parameters) = emulator_data.parameters.as_mut() else {
                return vec![CanMessage::ParameterSetConfirmation {
                    payload: payloads::ParameterSetConfirmationPayload {
                        parameter_id: payload.parameter_id,
                        status: payloads::ParameterSetStatus::InvalidParameterID,
                        value: payload.value.clone(),
                    },
                }];
            };
            let indices = sorted_parameter_indices(parameters);
            let field_index =
                index_from_field_id(payload.parameter_id).and_then(|idx| indices.get(idx).copied());
            let Some(param_index) = field_index else {
                return vec![CanMessage::ParameterSetConfirmation {
                    payload: payloads::ParameterSetConfirmationPayload {
                        parameter_id: payload.parameter_id,
                        status: payloads::ParameterSetStatus::InvalidParameterID,
                        value: payload.value.clone(),
                    },
                }];
            };
            let param = &mut parameters[param_index];
            if param.locked {
                let current_value = value_from_u32(param.value, param.datatype);
                return vec![CanMessage::ParameterSetConfirmation {
                    payload: payloads::ParameterSetConfirmationPayload {
                        parameter_id: payload.parameter_id,
                        status: payloads::ParameterSetStatus::ParameterLocked,
                        value: current_value,
                    },
                }];
            }
            let Some(new_value) = u32_from_value(&payload.value, param.datatype) else {
                let current_value = value_from_u32(param.value, param.datatype);
                return vec![CanMessage::ParameterSetConfirmation {
                    payload: payloads::ParameterSetConfirmationPayload {
                        parameter_id: payload.parameter_id,
                        status: payloads::ParameterSetStatus::InvalidParameterID,
                        value: current_value,
                    },
                }];
            };
            param.value = new_value;
            let confirmed_value = value_from_u32(param.value, param.datatype);
            vec![CanMessage::ParameterSetConfirmation {
                payload: payloads::ParameterSetConfirmationPayload {
                    parameter_id: payload.parameter_id,
                    status: payloads::ParameterSetStatus::Success,
                    value: confirmed_value,
                },
            }]
        }
        CanMessage::ParameterSetConfirmation { payload: _payload } => Vec::new(),
        CanMessage::ParameterSetLockReq { payload } => {
            let Some(parameters) = emulator_data.parameters.as_mut() else {
                return vec![CanMessage::ParameterSetLockConfirmation {
                    payload: payloads::ParameterSetLockConfirmationPayload {
                        parameter_id: payload.parameter_id,
                        parameter_lock: payload.parameter_lock,
                        field_status: payloads::FieldStatus::NotFound,
                    },
                }];
            };
            let indices = sorted_parameter_indices(parameters);
            let field_index =
                index_from_field_id(payload.parameter_id).and_then(|idx| indices.get(idx).copied());
            let Some(param_index) = field_index else {
                return vec![CanMessage::ParameterSetLockConfirmation {
                    payload: payloads::ParameterSetLockConfirmationPayload {
                        parameter_id: payload.parameter_id,
                        parameter_lock: payload.parameter_lock,
                        field_status: payloads::FieldStatus::NotFound,
                    },
                }];
            };
            let param = &mut parameters[param_index];
            param.locked = matches!(
                payload.parameter_lock,
                payloads::ParameterLockStatus::Locked
            );
            vec![CanMessage::ParameterSetLockConfirmation {
                payload: payloads::ParameterSetLockConfirmationPayload {
                    parameter_id: payload.parameter_id,
                    parameter_lock: payload.parameter_lock,
                    field_status: payloads::FieldStatus::Ok,
                },
            }]
        }
        CanMessage::FieldGetReq { payload } => {
            let field_id = payload.field_id;
            let is_telemetry = (field_id & TELEMETRY_ID_BIT) != 0;
            if is_telemetry {
                let Some(telemetry) = emulator_data.telemetry_values.as_ref() else {
                    return vec![CanMessage::FieldGetRes {
                        payload: payloads::FieldGetResPayload {
                            field_id,
                            field_status: payloads::FieldStatus::NotFound,
                            value: payloads::CanDataValue::Raw(Vec::new()),
                        },
                    }];
                };
                let indices = sorted_telemetry_indices(telemetry);
                let field_index =
                    index_from_field_id(field_id).and_then(|idx| indices.get(idx).copied());
                let Some(tel_index) = field_index else {
                    return vec![CanMessage::FieldGetRes {
                        payload: payloads::FieldGetResPayload {
                            field_id,
                            field_status: payloads::FieldStatus::NotFound,
                            value: payloads::CanDataValue::Raw(Vec::new()),
                        },
                    }];
                };
                let tel = &telemetry[tel_index];
                let value = value_from_u32(tel.value, tel.datatype);
                vec![CanMessage::FieldGetRes {
                    payload: payloads::FieldGetResPayload {
                        field_id,
                        field_status: payloads::FieldStatus::Ok,
                        value,
                    },
                }]
            } else {
                let Some(parameters) = emulator_data.parameters.as_ref() else {
                    return vec![CanMessage::FieldGetRes {
                        payload: payloads::FieldGetResPayload {
                            field_id,
                            field_status: payloads::FieldStatus::NotFound,
                            value: payloads::CanDataValue::Raw(Vec::new()),
                        },
                    }];
                };
                let indices = sorted_parameter_indices(parameters);
                let field_index =
                    index_from_field_id(field_id).and_then(|idx| indices.get(idx).copied());
                let Some(param_index) = field_index else {
                    return vec![CanMessage::FieldGetRes {
                        payload: payloads::FieldGetResPayload {
                            field_id,
                            field_status: payloads::FieldStatus::NotFound,
                            value: payloads::CanDataValue::Raw(Vec::new()),
                        },
                    }];
                };
                let param = &parameters[param_index];
                let value = value_from_u32(param.value, param.datatype);
                vec![CanMessage::FieldGetRes {
                    payload: payloads::FieldGetResPayload {
                        field_id,
                        field_status: payloads::FieldStatus::Ok,
                        value,
                    },
                }]
            }
        }
        CanMessage::FieldGetRes { payload: _payload } => Vec::new(),
        CanMessage::FieldIDLookupReq { payload } => {
            let field_name: String = payload.field_name.clone().into();
            if let Some(telemetry) = emulator_data.telemetry_values.as_ref() {
                let indices = sorted_telemetry_indices(telemetry);
                if let Some((pos, tel)) = indices.iter().enumerate().find_map(|(pos, &idx)| {
                    let tel = &telemetry[idx];
                    (tel.name == field_name).then_some((pos, tel))
                }) {
                    let field_id = telemetry_id_for_index(pos).unwrap_or(0);
                    return vec![CanMessage::FieldIDLookupRes {
                        payload: payloads::FieldIDLookupResPayload {
                            field_id,
                            field_status: payloads::FieldStatus::Ok,
                            field_type: tel.datatype,
                        },
                    }];
                }
            }
            if let Some(parameters) = emulator_data.parameters.as_ref() {
                let indices = sorted_parameter_indices(parameters);
                if let Some((pos, param)) = indices.iter().enumerate().find_map(|(pos, &idx)| {
                    let param = &parameters[idx];
                    (param.name == field_name).then_some((pos, param))
                }) {
                    let field_id = parameter_id_for_index(pos).unwrap_or(0);
                    return vec![CanMessage::FieldIDLookupRes {
                        payload: payloads::FieldIDLookupResPayload {
                            field_id,
                            field_status: payloads::FieldStatus::Ok,
                            field_type: param.datatype,
                        },
                    }];
                }
            }
            vec![CanMessage::FieldIDLookupRes {
                payload: payloads::FieldIDLookupResPayload {
                    field_id: 0,
                    field_status: payloads::FieldStatus::NotFound,
                    field_type: payloads::CanDataType::UInt8,
                },
            }]
        }
        CanMessage::FieldIDLookupRes { payload: _payload } => Vec::new(),
        _ => Vec::new(),
    }
}

pub enum StatusMessageKind {
    Info,
    Warning,
    Error,
}

pub fn build_status_message(kind: StatusMessageKind, message: &str) -> CanMessage {
    let msg = payloads::CanString::<63>::try_from(message)
        .expect("Status message too long (max 63 bytes)");
    let payload = payloads::StatusPayload { msg };
    match kind {
        StatusMessageKind::Info => CanMessage::InfoStatus { payload },
        StatusMessageKind::Warning => CanMessage::WarningStatus { payload },
        StatusMessageKind::Error => CanMessage::ErrorStatus { payload },
    }
}
