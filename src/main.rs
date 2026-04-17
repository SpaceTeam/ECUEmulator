use socketcan::{ShouldRetry, Socket};
use std::env;
use std::time::{Duration, Instant};
use ECUEmulator::can_manager::{self, send_messages, socket_manager};
use ECUEmulator::config;
use ECUEmulator::message_handling::{
    build_telemetry_group_updates, handle_message, parse_can_message, registration_flow_messages,
};

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
    send_messages(&mut socket, sender_id, 1, registration_messages);

    let update_interval = if config.frequency == 0 {
        None
    } else {
        Some(Duration::from_secs_f64(1.0 / config.frequency as f64))
    };
    let mut last_update = Instant::now();

    println!("Starting ECUEmulator");
    loop {
        if let Some(interval) = update_interval {
            if last_update.elapsed() >= interval {
                let updates = build_telemetry_group_updates(&config);
                send_messages(&mut socket, sender_id, 1, updates);
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
