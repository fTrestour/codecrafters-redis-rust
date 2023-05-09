use crate::{command::Command, protocol::Resp};
use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpStream,
    str,
    sync::{Arc, Mutex},
};

pub fn handle(
    stream: TcpStream,
    store: &Arc<Mutex<HashMap<String, String>>>,
    callback: fn(Command, &Arc<Mutex<HashMap<String, String>>>) -> Resp,
) {
    println!("Accepted new connection");
    let mut buf: [u8; 512] = [0; 512];

    loop {
        let message = read(&stream, &mut buf);
        match message {
            None => break,
            Some(message) => {
                let response: Resp = callback(message, &store);

                send(&stream, response);
            }
        }
    }

    println!("Closing connection");
}

fn read(mut stream: &TcpStream, buf: &mut [u8]) -> Option<Command> {
    let bytes_read = stream
        .read(buf)
        .expect("Could not read bytes from connection");

    if bytes_read == 0 {
        return Option::None;
    }

    let message = str::from_utf8(buf).expect("Could not parse message");
    let message = Resp::from(message);
    let command = Command::from_resp(message);

    println!("Received {:?}", command);
    return command;
}

fn send(mut stream: &TcpStream, message: Resp) {
    let payload = message.to_string();
    let payload = payload.as_bytes();

    stream.write_all(payload).expect("Could not respond");
    println!("Sent {:?}", message);
}
