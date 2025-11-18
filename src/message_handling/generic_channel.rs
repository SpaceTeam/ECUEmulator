use crate::config::state_storage::StateStorage;
use crate::message_handling::parse_can_message;
use crate::protocol::channels::{GenericCommand, HeartBeatDataMsg, NodeInfoMsg};
use crate::protocol::commands::SetMsgPayload;
use crate::protocol::message::{CommandTrait, Message};
use crate::protocol::raw_can_message::CanMessageDirection::NodeToMaster;
use crate::protocol::CanMessageId;
use socketcan::{CanAnyFrame, EmbeddedFrame};
use zerocopy::IntoBytes;

pub fn handle_generic_command(cmd: &GenericCommand, state: &mut StateStorage) -> Option<Message> {
    match cmd {
        GenericCommand::GenericReqResetAllSettings => Some(Message::GenericChannelMessage(
            GenericCommand::GenericResResetAllSettings,
        )),
        GenericCommand::GenericReqStatus => Some(Message::GenericChannelMessage(
            GenericCommand::GenericResStatus,
        )),
        GenericCommand::GenericReqSetVariable { payload } => {
            // Echo back the set payload as a response
            state.set(
                format!("GenericCommand.variables.{}", payload.variable_id),
                Vec::from(payload.value.to_le_bytes()),
            );
            let resp = SetMsgPayload {
                variable_id: payload.variable_id,
                value: payload.value,
            };
            Some(Message::GenericChannelMessage(
                GenericCommand::GenericResSetVariable { payload: resp },
            ))
        }
        GenericCommand::GenericReqGetVariable { payload } => {
            // Return a dummy value for the requested variable

            let resp = SetMsgPayload {
                variable_id: payload.variable_id,
                value: state
                    .get_u32_or_zero(format!("GenericCommand.variables.{}", payload.variable_id)),
            };
            Some(Message::GenericChannelMessage(
                GenericCommand::GenericResGetVariable { payload: resp },
            ))
        }
        GenericCommand::GenericReqSyncClock => Some(Message::GenericChannelMessage(
            GenericCommand::GenericResSyncClock,
        )),
        GenericCommand::GenericReqData => {
            let hb = HeartBeatDataMsg {
                channel_mask: state
                    .get_u32_or_zero("GenericChannel.GenericReqData.channel_mask".to_string()),
                data: state.get_value_slice_or_zeros::<56>(
                    "GenericChannel.GenericReqData.data".to_string(),
                ),
            };
            Some(Message::GenericChannelMessage(
                GenericCommand::GenericResData { payload: hb },
            ))
        }
        GenericCommand::GenericReqNodeInfo => {
            let node = NodeInfoMsg {
                firmware_version: state.get_u32_or_zero(
                    "GenericChannel.GenericReqNodeInfo.firmware_version".to_string(),
                ),
                channel_mask: state
                    .get_u32_or_zero("GenericChannel.GenericReqNodeInfo.channel_mask".to_string()),
                channel_type: state.get_value_slice_or_zeros::<32>(
                    "GenericChannel.GenericReqNodeInfo.channel_type".to_string(),
                ),
            };
            Some(Message::GenericChannelMessage(
                GenericCommand::GenericResNodeInfo { payload: node },
            ))
        }
        GenericCommand::GenericReqFlashClear => Some(Message::GenericChannelMessage(
            GenericCommand::GenericResFlashStatus {
                status: state
                    .get_u8_or_zero("GenericChannel.GenericReqFlashClear.status".to_string()),
            },
        )),
        // For requests that either have no response or are not implemented here, return None
        _ => {
            println!("Unhandled or non-request GenericCommand: {:?}", cmd);
            None
        }
    }
}

#[test]
pub fn test_handle_generic_command() {
    let cmd = GenericCommand::GenericResGetVariable {
        payload: SetMsgPayload {
            variable_id: 1,
            value: 10,
        },
    };
    let sending_message = Message::GenericChannelMessage(cmd);

    let msg = sending_message.as_can_message_data();
    let bytes: &[u8] = msg.as_bytes();

    let id: CanMessageId = CanMessageId::new()
        .with_direction(NodeToMaster)
        .with_node_id(3);

    let frame =
        socketcan::CanFdFrame::new(socketcan::StandardId::new(id.into()).unwrap(), &bytes).unwrap();

    let parsed_msg = parse_can_message(CanAnyFrame::from(frame)).unwrap();

    assert_eq!(parsed_msg.0, id);
    assert_eq!(parsed_msg.1, sending_message);
}
