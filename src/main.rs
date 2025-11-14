mod can_manager;
mod protocol;

use crate::protocol::raw_can_message::MessageSpecialCommand::StandardSpecialCmd;
use crate::protocol::raw_can_message::{CanMessageDirection, CanMessagePriority};
use crate::protocol::CanMessageBufferType::DirectBuffer;
use socketcan::{CanAnyFrame, CanFdFrame, CanFdSocket, EmbeddedFrame, Id, Socket, StandardId};
use zerocopy::IntoBytes;
use crate::protocol::commands::{GetMsgPayload, SetMsgPayload};

fn main() {
    let s = CanFdSocket::open("vcan0");
    let mut socket: CanFdSocket;

    match s {
        Ok(s) => socket = s,
        Err(e) => {
            eprintln!("Error opening CAN FD socket: {}", e);
            return;
        }
    }
    let id = protocol::CanMessageId::new()
        .with_direction(CanMessageDirection::NodeToMaster)
        .with_node_id(4)
        .with_special_cmd(StandardSpecialCmd)
        .with_priority(CanMessagePriority::StandardPriority);

    let message = protocol::channels::GenericCommand::GenericResGetVariable {
        payload: SetMsgPayload {
            variable_id: 1,
            value: 10
        },
    };

    match protocol::message::send_message(id, protocol::message::Message::GenericChannelMessage(message), &mut socket){
        Ok(a)=>{
            println!("Message sent successfully");
        },
        Err(e)=>{
            eprintln!("Error sending message: {}", e);
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
