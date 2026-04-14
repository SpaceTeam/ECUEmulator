use crate::can_manager;
use liquidcan::raw_can_message::CanMessagePriority;
use liquidcan::CanMessageId;
use socketcan::CanFdSocket;

pub mod errors;
pub mod socket_manager;

fn make_message_id(receiver_id: u8, sender_id: u8) -> CanMessageId {
    CanMessageId::new()
        .with_receiver_id(receiver_id)
        .with_sender_id(sender_id)
        .with_priority(CanMessagePriority::Low)
}

fn should_also_notify_server(msg: &liquidcan::CanMessage) -> bool {
    matches!(
        msg,
        liquidcan::CanMessage::ParameterSetConfirmation { .. }
            | liquidcan::CanMessage::ParameterSetLockConfirmation { .. }
    )
}

pub fn send_messages(
    socket: &mut CanFdSocket,
    sender_id: u8,
    receiver_id: u8,
    messages: Vec<liquidcan::CanMessage>,
) {
    for msg in messages {
        let id = make_message_id(receiver_id, sender_id);
        if let Err(err) = can_manager::socket_manager::send_frame(socket, id, msg.clone()) {
            eprintln!("Error sending CAN FD frame: {err:?}");
        }
        if receiver_id != 0 && should_also_notify_server(&msg) {
            let server_id = 0;
            let server_msg_id = make_message_id(server_id, sender_id);
            if let Err(err) = can_manager::socket_manager::send_frame(socket, server_msg_id, msg) {
                eprintln!("Error sending CAN FD frame to server: {err:?}");
            }
        }
    }
}
