use crate::can_manager::send_frame;
use crate::protocol::channels::GenericCommand;
use crate::protocol::{CanMessageData, CanMessageId};
use enum_dispatch::enum_dispatch;
use socketcan::CanFdSocket;

#[derive(Debug)]
#[enum_dispatch(CommandTrait)]
pub enum Message {
    GenericChannelMessage(GenericCommand),
}
#[enum_dispatch]
pub trait CommandTrait {
    fn as_can_message_data(&self) -> CanMessageData;
}
pub fn send_message(
    id: CanMessageId,
    msg: Message,
    mut socket: CanFdSocket,
) -> Result<(), crate::can_manager::errors::SendFrameError> {
    let can_message_data = msg.as_can_message_data();

    send_frame(&mut socket, id, can_message_data)
}
