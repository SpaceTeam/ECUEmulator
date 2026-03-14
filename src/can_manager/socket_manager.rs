use crate::can_manager::errors::SendFrameError;
use liquidcan::{CanMessage, CanMessageId};
use socketcan::{CanAnyFrame, CanFdSocket, EmbeddedFrame, Socket};

#[allow(dead_code)]
pub fn open_socket(interface: &str) -> Result<CanFdSocket, std::io::Error> {
    CanFdSocket::open(interface)
}

#[allow(dead_code)]
pub fn read_frame(socket: &mut CanFdSocket) -> Result<CanAnyFrame, std::io::Error> {
    socket.read_frame()
}

#[allow(dead_code)]
pub fn send_frame(
    socket: &mut CanFdSocket,
    can_message_id: CanMessageId,
    can_message: CanMessage,
) -> Result<(), SendFrameError> {
    let raw_id: u16 = can_message_id.into();
    let id = socketcan::StandardId::new(raw_id).ok_or(SendFrameError::InvalidId { raw_id })?;
    let frame: socketcan::CanFdFrame = can_message.into();
    let data = frame.data();
    let frame = socketcan::CanFdFrame::new(id, data)
        .ok_or(SendFrameError::InvalidFrameLength { len: data.len() })?;

    socket.write_frame_insist(&frame)?;
    Ok(())
}

#[allow(dead_code)]
fn send_raw_frame(
    socket: &mut CanFdSocket,
    frame: socketcan::CanFdFrame,
) -> Result<(), std::io::Error> {
    match socket.write_frame_insist(&frame) {
        Err(reason) => Err(reason),
        Ok(_) => Ok(()),
    }
}
