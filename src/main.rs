mod can_manager;
mod config;
mod message_handling;

use crate::message_handling::{handle_message, parse_can_message};
use socketcan::{CanFdSocket, Socket};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let res = config::config_loader::load_config((args[1]).as_ref());
    let Ok(mut config) = res else {
        eprintln!("Error loading config file: {:?}", res.err().unwrap());
        return;
    };

    let res = CanFdSocket::open("vcan0");

    let Ok(mut socket) = res else {
        eprintln!("Error opening CAN FD socket: ",);
        return;
    };

    loop {
        let res = can_manager::socket_manager::read_frame(&mut socket);
        let Ok(frame) = res else {
            eprintln!("Error reading CAN FD frame: ",);
            continue;
        };
        let res = parse_can_message(frame);
        let Ok((_id, msg)) = res else {
            println!("Error during parsing frame: {}", res.err().unwrap());
            continue;
        };

        let responses = handle_message(&msg, &mut config);

        for msg in responses {
            //TODO id should of course be the other way around/adressing the main server
            println!("Sending response: {:?}", msg);
        }
    }
}
