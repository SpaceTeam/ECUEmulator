use crate::can_manager::send_frame;
use crate::protocol::channels::{GenericCommand, GenericCommandPadded};
use crate::protocol::{CanMessageBufferType, CanMessageData, CanMessageId};
use anyhow::{anyhow, Error};
use ecu_emulator_macros_derive::EnumDiscriminate;
use socketcan::CanFdSocket;
use zerocopy::TryFromBytes;
use zerocopy::{FromZeros, IntoBytes};

#[derive(Debug, EnumDiscriminate, PartialEq)]
#[repr(u8)]
pub enum Message {
    GenericChannelMessage(GenericCommand) =
        MessageDiscriminant::GenericChannelMessage.discriminant(),
}

impl TryFrom<CanMessageData> for Message {
    type Error = Error;
    fn try_from(value: CanMessageData) -> Result<Self, Self::Error> {
        let channel_id = value.data_info.channel_id();
        let message_data = &value.as_bytes()[1..];
        match channel_id {
            x if x == MessageDiscriminant::GenericChannelMessage as u8 => {
                let padded = GenericCommandPadded::try_read_from_bytes(&message_data)
                    .map_err(|e| anyhow!("Failed to convert message: {}", e))?;
                Ok(Message::GenericChannelMessage(padded.into()))
            }
            _ => Err(anyhow!("Invalid message type: {value:?}")),
        }
    }
}

impl CommandTrait for Message {
    fn as_can_message_data(&self) -> CanMessageData {
        let mut data = CanMessageData::new_zeroed();
        match self {
            Message::GenericChannelMessage(cmd) => {
                let padded: GenericCommandPadded = cmd.clone().into();
                // The first byte is the discriminant, which is set separately.
                let bytes: &[u8] = &padded.as_bytes()[1..];
                data.data[..bytes.len()].copy_from_slice(bytes);
                data.command_id = cmd.discriminant();
            }
        };
        data.data_info.set_channel_id(self.discriminant() as u8);
        data
    }
}
#[derive(EnumDiscriminate)]
#[repr(u8)]
pub enum MessageDiscriminant {
    GenericChannelMessage = 5,
}

pub trait CommandTrait {
    fn as_can_message_data(&self) -> CanMessageData;
}
pub fn send_message(
    id: CanMessageId,
    msg: Message,
    socket: &mut CanFdSocket,
) -> Result<(), crate::can_manager::errors::SendFrameError> {
    let mut can_message_data: CanMessageData = msg.as_can_message_data();

    // In the old llserver, only DirectBuffer is used.
    can_message_data
        .data_info
        .set_can_message_buffer(CanMessageBufferType::DirectBuffer);

    send_frame(socket, id, can_message_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::channels::{GenericCommand, HeartBeatDataMsg, NodeInfoMsg};
    use crate::protocol::commands::{GetMsgPayload, SetMsgPayload};

    fn test_round_trip(msg: Message) {
        let can_data = msg.as_can_message_data();
        let msg_back = Message::try_from(can_data).expect("Failed to convert back to Message");
        assert_eq!(msg, msg_back);
    }

    #[test]
    fn test_generic_req_reset_all_settings() {
        let msg = Message::GenericChannelMessage(GenericCommand::GenericReqResetAllSettings);
        test_round_trip(msg);
    }

    #[test]
    fn test_generic_req_set_variable() {
        let payload = SetMsgPayload {
            variable_id: 10,
            value: 12345,
        };
        let msg = Message::GenericChannelMessage(GenericCommand::GenericReqSetVariable { payload });
        test_round_trip(msg);
    }

    #[test]
    fn test_generic_req_get_variable() {
        let payload = GetMsgPayload { variable_id: 42 };
        let msg = Message::GenericChannelMessage(GenericCommand::GenericReqGetVariable { payload });
        test_round_trip(msg);
    }

    #[test]
    fn test_generic_res_data() {
        let payload = HeartBeatDataMsg {
            channel_mask: 0xFF00FF00,
            data: [0xAA; 56],
        };
        let msg = Message::GenericChannelMessage(GenericCommand::GenericResData { payload });
        test_round_trip(msg);
    }

    #[test]
    fn test_generic_res_node_info() {
        let payload = NodeInfoMsg {
            firmware_version: 1,
            channel_mask: 0x00FF00FF,
            channel_type: [0xBB; 32],
        };
        let msg = Message::GenericChannelMessage(GenericCommand::GenericResNodeInfo { payload });
        test_round_trip(msg);
    }

    #[test]
    fn test_generic_res_flash_status() {
        let msg =
            Message::GenericChannelMessage(GenericCommand::GenericResFlashStatus { status: 5 });
        test_round_trip(msg);
    }
}
