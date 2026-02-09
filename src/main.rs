mod can_manager;
mod config;
mod message_handling;
mod protocol;

use crate::message_handling::{handle_message, parse_can_message};
use crate::protocol::message_conversion::send_message;
use clap::Parser;
use socketcan::{CanFdSocket, Socket};
use std::env;
use std::string::ToString;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut res = config::config_loader::load_config((&args[1]).as_ref());
    let Ok(mut config) = res else {
        println!("Error loading config file");
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
        let Ok((id, msg)) = res else {
            println!("Error during parsing frame: {}", res.err().unwrap());
            continue;
        };

        let response = handle_message(&msg, &mut config);

        match response {
            None => {}
            Some(msg) => {
                //TODO id should of course be the other way around/adressing the main server
                match send_message(id, msg, &mut socket) {
                    Ok(_) => {}
                    Err(err) => {
                        println!("Error during sending message: {}", err);
                        continue;
                    }
                }
            }
        }
    }
}
