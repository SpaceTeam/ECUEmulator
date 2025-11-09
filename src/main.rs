use socketcan::{CanAnyFrame, CanFdSocket, Socket};


fn main() {

    let s = CanFdSocket::open("vcan0");
    let socket : CanFdSocket;
    match s {
        Ok(s)=> socket = s,
        Err(e)=>{
            eprintln!("Error opening CAN FD socket: {}", e);
            return;
        }
    }
    loop{
        let frame = socket.read_frame().unwrap();
        if matches!(frame,CanAnyFrame::Normal(_)){
            println!("Received CAN FD frame: {:?}", frame);
        }
    }
}
