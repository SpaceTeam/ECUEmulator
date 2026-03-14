mod can_manager;
mod config;
mod message_handling;

use crate::can_manager::socket_manager;
use crate::message_handling::{
    build_telemetry_group_updates, handle_message, parse_can_message, registration_flow_messages,
};
use liquidcan::raw_can_message::CanMessagePriority;
use liquidcan::CanMessageId;
use socketcan::{CanFdSocket, ShouldRetry, Socket};
use std::env;
use std::time::{Duration, Instant};

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

fn send_messages(
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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <config_file_path>", args[0]);
        std::process::exit(1);
    }
    let res = config::config_loader::load_config((args[1]).as_ref());
    let Ok(mut config) = res else {
        eprintln!("Error loading config file: {:?}", res.err().unwrap());
        return;
    };

    if config.node_id > 31 {
        eprintln!("Invalid node_id {} (must be <= 31)", config.node_id);
        return;
    }
    let sender_id = config.node_id as u8;

    let res = can_manager::socket_manager::open_socket(&config.can_interface);
    let Ok(mut socket) = res else {
        eprintln!("Error opening CAN FD socket: :{:?}", res.err().unwrap());
        return;
    };

    if let Err(err) = socket.set_read_timeout(Some(Duration::from_millis(50))) {
        eprintln!("Error setting CAN FD socket timeout: {err:?}");
        return;
    }

    let registration_messages = registration_flow_messages(&config);
    send_messages(&mut socket, sender_id, 0, registration_messages);

    let update_interval = if config.frequency == 0 {
        None
    } else {
        Some(Duration::from_secs_f64(1.0 / config.frequency as f64))
    };
    let mut last_update = Instant::now();

    loop {
        if let Some(interval) = update_interval {
            if last_update.elapsed() >= interval {
                let updates = build_telemetry_group_updates(&config);
                send_messages(&mut socket, sender_id, 0, updates);
                last_update = Instant::now();
            }
        }

        let res = socket_manager::read_frame(&mut socket);
        let Ok(frame) = res else {
            if res.should_retry() {
                continue;
            }
            eprintln!("Error reading CAN FD frame: {:?}", res.err().unwrap());
            continue;
        };
        let res = parse_can_message(frame);
        let Ok((id, msg)) = res else {
            println!("Error during parsing frame: {}", res.err().unwrap());
            continue;
        };

        let responses = handle_message(&msg, &mut config);
        let receiver_id = id.sender_id();
        send_messages(&mut socket, sender_id, receiver_id, responses);
    }
}
