mod can_manager;
mod protocol;

use crate::protocol::raw_can_message::MessageSpecialCommand::StandardSpecialCmd;
use crate::protocol::raw_can_message::{CanMessageDirection, CanMessagePriority};
use crate::protocol::CanMessageBufferType::DirectBuffer;
use socketcan::{CanAnyFrame, CanFdFrame, CanFdSocket, EmbeddedFrame, Id, Socket, StandardId};
use zerocopy::IntoBytes;

fn main() {
    let s = CanFdSocket::open("vcan0");
    let socket: CanFdSocket;

    let id = protocol::CanMessageId::new()
        .with_direction(CanMessageDirection::NodeToMaster)
        .with_node_id(4)
        .with_special_cmd(StandardSpecialCmd)
        .with_priority(CanMessagePriority::StandardPriority);

    let mut data = protocol::CanMessageData {
        data_info: protocol::CanMessageDataInfo::new()
            .with_channel_id(5)
            .with_can_message_buffer(DirectBuffer),
        command_id: 2,
        data: [0; 62],
    };
    data.data[13] = 0x3;

    let frame = CanFdFrame::new(
        Id::Standard(StandardId::new(id.into()).unwrap()),
        data.as_mut_bytes(),
    );

    match frame {
        Some(f) => {
            println!("Sending frame {:?}", f)
        }
        None => {
            println!("Error sending frame");
        }
    }
    match s {
        Ok(s) => socket = s,
        Err(e) => {
            eprintln!("Error opening CAN FD socket: {}", e);
            return;
        }
    }
    loop {
        let frame = socket.read_frame().unwrap();
        println!("{:?}", frame);
        if matches!(frame, CanAnyFrame::Normal(_)) {
            println!("Received CAN FD frame: {:?}", frame);
        }
        if matches!(frame, CanAnyFrame::Fd(_)) {
            println!("Received CAN FD frame: {:?}", frame);
        }
        if matches!(frame, CanAnyFrame::Error(_)) {
            println!("Received CAN FD frame: {:?}", frame);
        }
    }
}
