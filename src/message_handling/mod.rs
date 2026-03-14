mod message_handler;

use crate::config::config_representation::EmulatorData;
use anyhow::{anyhow, Result};
use liquidcan::{CanMessage, CanMessageId};
use socketcan::{CanAnyFrame, EmbeddedFrame, Id};
use zerocopy::FromBytes;

pub use message_handler::{
    build_status_message, build_telemetry_group_updates, registration_flow_messages,
    StatusMessageKind,
};

pub fn handle_message(msg: &CanMessage, emulator_data: &mut EmulatorData) -> Vec<CanMessage> {
    message_handler::handle_message(msg, emulator_data)
}

pub fn parse_can_message(frame: CanAnyFrame) -> Result<(CanMessageId, CanMessage)> {
    let CanAnyFrame::Fd(frame) = frame else {
        panic!("Only CAN FD frames are supported");
    };
    let Id::Standard(raw_id) = frame.id() else {
        panic!("Only standard CAN IDs are supported");
    };
    let id = CanMessageId::from_bytes(raw_id.as_raw().to_le_bytes());
    let message: CanMessage = frame
        .try_into()
        .map_err(|e| anyhow!("Failed to parse CAN message data: {}", e))?;

    Ok((id, message))
}
