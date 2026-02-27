use crate::can_manager::errors::SendFrameError;
use liquidcan::{CanMessageFrame, CanMessageId};
use socketcan::{CanAnyFrame, CanFdSocket, EmbeddedFrame, Socket};
use zerocopy::IntoBytes;

pub fn open_socket(interface: &str) -> Result<CanFdSocket, std::io::Error> {
    CanFdSocket::open(interface)
}

pub fn read_frame(socket: &mut CanFdSocket) -> Result<CanAnyFrame, std::io::Error> {
    socket.read_frame()
}
pub fn send_frame(
    socket: &mut CanFdSocket,
    can_message_id: CanMessageId,
    can_message_frame: CanMessageFrame,
) -> Result<(), SendFrameError> {
    let raw_id: u16 = can_message_id.into();
    let id =
        socketcan::StandardId::new(raw_id).ok_or(SendFrameError::InvalidId { raw_id: raw_id })?;
    let bytes = can_message_frame.as_bytes();
    let frame = socketcan::CanFdFrame::new(id, bytes)
        .ok_or(SendFrameError::InvalidFrameLength { len: bytes.len() })?;

    socket.write_frame_insist(&frame)?;
    Ok(())
}
fn send_raw_frame(
    socket: &mut CanFdSocket,
    frame: socketcan::CanFdFrame,
) -> Result<(), std::io::Error> {
    match socket.write_frame_insist(&frame) {
        Err(reason) => Err(reason),
        Ok(_) => Ok(()),
    }
}
