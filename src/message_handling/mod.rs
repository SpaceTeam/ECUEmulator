mod message_handler;

use crate::config::state_storage::StateStorage;
use anyhow::{anyhow, Result};
use liquidcan::{CanMessage, CanMessageFrame, CanMessageId};
use socketcan::{CanAnyFrame, EmbeddedFrame, Id};
use zerocopy::FromBytes;

pub fn handle_message(msg: &CanMessage, state: &mut StateStorage) -> Option<CanMessage> {
    message_handler::handle_message(msg, state)
}

pub fn parse_can_message(frame: CanAnyFrame) -> Result<(CanMessageId, CanMessage)> {
    let CanAnyFrame::Fd(frame) = frame else {
        panic!("Only CAN FD frames are supported");
    };
    let Id::Standard(raw_id) = frame.id() else {
        panic!("Only standard CAN IDs are supported");
    };
    let id = CanMessageId::from_bytes(raw_id.as_raw().to_le_bytes());
    let data = CanMessageFrame::read_from_bytes(frame.data())
        .map_err(|e| anyhow!("Failed to parse CAN message data: {}", e))?;
    println!("can msg data: {:?}", data);

    let message: CanMessage = data.try_into()?;

    Ok((id, message))
}
