use crate::protocol::Resp;
use std::{
    io::{Read, Write},
    net::TcpStream,
    str,
};

pub fn handle(stream: TcpStream, callback: fn(Resp) -> Resp) {
    println!("Accepted new connection");
    let mut buf: [u8; 512] = [0; 512];

    loop {
        let message = read(&stream, &mut buf);
        match message {
            None => break,
            Some(message) => {
                let response = callback(message);

                send(&stream, response);
            }
        }
    }

    println!("Closing connection");
}

fn read<'a>(mut stream: &TcpStream, buf: &'a mut [u8]) -> Option<Resp<'a>> {
    let bytes_read = stream
        .read(buf)
        .expect("Could not read bytes from connection");

    if bytes_read == 0 {
        return Option::None;
    }

    let message = str::from_utf8(buf).expect("Could not parse message");
    let message = Resp::from_str(message);

    println!("Received {:?}", message);
    return Option::Some(message);
}

fn send(mut stream: &TcpStream, message: Resp) {
    let payload = message.to_str();
    let payload = payload.as_bytes();

    stream.write_all(payload).expect("Could not respond");
    println!("Sent {:?}", message);
}
