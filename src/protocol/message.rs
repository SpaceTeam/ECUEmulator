use crate::can_manager::send_frame;
use crate::protocol::channels::GenericCommand;
use crate::protocol::{CanMessageBufferType, CanMessageData, CanMessageId};
use anyhow::{anyhow, Error};
use ecu_emulator_macros_derive::EnumDiscriminate;
use socketcan::CanFdSocket;
#[derive(Debug, EnumDiscriminate, PartialEq)]
#[repr(u8)]
pub enum Message {
    GenericChannelMessage(GenericCommand) =
        MessageDiscriminant::GenericChannelMessage.discriminant(),
}

impl TryFrom<CanMessageData> for Message {
    type Error = Error;
    fn try_from(value: CanMessageData) -> Result<Self, Self::Error> {
        match value.data_info.channel_id() as u8 {
            x if x == MessageDiscriminant::GenericChannelMessage as u8 => {
                // Placeholder: In a real implementation, you would parse the payload to create the GenericCommand
                Ok(Message::GenericChannelMessage(value.try_into()?))
            }
            _ => Err(anyhow!("Invalid message type: {value:?}")),
        }
    }
}

impl CommandTrait for Message {
    fn as_can_message_data(&self) -> CanMessageData {
        let mut data = match self {
            Message::GenericChannelMessage(cmd) => cmd.as_can_message_data(),
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
    can_message_data
        .data_info
        .set_channel_id(msg.discriminant() as u8);

    // In the old llserver, only DirectBuffer is used.
    can_message_data
        .data_info
        .set_can_message_buffer(CanMessageBufferType::DirectBuffer);

    send_frame(socket, id, can_message_data)
}
