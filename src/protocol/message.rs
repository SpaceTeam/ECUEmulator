use crate::can_manager::send_frame;
use crate::protocol::channels::GenericCommand;
use crate::protocol::{CanMessageBufferType, CanMessageData, CanMessageId};
use enum_dispatch::enum_dispatch;
use socketcan::CanFdSocket;

#[derive(Debug)]
#[enum_dispatch(CommandTrait)]
pub enum Message {
    GenericChannelMessage(GenericCommand),
}
pub enum MessageDiscriminant {
    GenericChannelMessage = 0,
}

impl From<Message> for u8 {
    fn from(value: Message) -> Self {
        match value {
            Message::GenericChannelMessage(_) => MessageDiscriminant::GenericChannelMessage as u8,
        }
    }
}
#[enum_dispatch]
pub trait CommandTrait {
    fn as_can_message_data(&self) -> CanMessageData;
}
pub fn send_message(
    id: CanMessageId,
    msg: Message,
    socket: &mut CanFdSocket,
) -> Result<(), crate::can_manager::errors::SendFrameError> {
    let mut can_message_data: CanMessageData = msg.as_can_message_data();
    can_message_data.data_info.set_channel_id(msg.into());

    // In the old llserver, only DirectBuffer is used.
    can_message_data
        .data_info
        .set_can_message_buffer(CanMessageBufferType::DirectBuffer);

    send_frame(socket, id, can_message_data)
}
