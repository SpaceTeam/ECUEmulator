mod generic_channel;

use crate::config::state_storage::StateStorage;
use crate::protocol::message::Message;
use crate::protocol::{CanMessageData, CanMessageId};
use anyhow::{anyhow, Result};
use socketcan::{CanAnyFrame, EmbeddedFrame, Id};
use zerocopy::FromBytes;

pub fn handle_message(msg: &Message, state: &mut StateStorage) -> Option<Message> {
    match msg {
        Message::GenericChannelMessage(cmd) => generic_channel::handle_generic_command(cmd, state),
    }
}

pub fn parse_can_message(frame: CanAnyFrame) -> Result<(CanMessageId, Message)> {
    let CanAnyFrame::Fd(frame) = frame else {
        panic!("Only CAN FD frames are supported");
    };
    let Id::Standard(raw_id) = frame.id() else {
        panic!("Only standard CAN IDs are supported");
    };
    let id = CanMessageId::from_bytes(raw_id.as_raw().to_le_bytes());
    let data = CanMessageData::read_from_bytes(frame.data())
        .map_err(|e| anyhow!("Failed to parse CAN message data: {}", e))?;
    println!("can msg data: {:?}", data);

    let message: Message = data.try_into()?;

    Ok((id, message))
}
