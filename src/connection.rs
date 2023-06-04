use crate::{command::Command, protocol::Resp, store::Store};
use std::{
    io::{Read, Write},
    net::TcpStream,
    str,
    sync::{Arc, Mutex},
};

pub fn handle(
    stream: TcpStream,
    store: Arc<Mutex<Store>>,
    callback: fn(Command, &Arc<Mutex<Store>>) -> Resp,
) {
    println!("Accepted new connection");
    let mut buf: [u8; 512] = [0; 512];

    loop {
        let message = read(&stream, &mut buf);
        match message {
            Err(e) => {
                println!("{:?}", e);
                break;
            }
            Ok(message) => {
                let response: Resp = callback(message, &store);

                send(&stream, response);
            }
        }
    }

    println!("Closing connection");
}

fn read(mut stream: &TcpStream, buf: &mut [u8]) -> Result<Command, &'static str> {
    let bytes_read = stream
        .read(buf)
        .or(Err("Could not read bytes from connection"))?;

    if bytes_read == 0 {
        return Err("Received empty message");
    }

    let message = str::from_utf8(buf).or(Err("Could not parse message"))?;
    let message = Resp::try_from(message)?;

    let command = Command::try_from(message)?;

    println!("Received:\n{:#?}", command);
    return Ok(command);
}

fn send(mut stream: &TcpStream, message: Resp) {
    let payload = message.render();
    let payload = payload.as_bytes();

    match stream.write_all(payload) {
        Ok(_) => println!("Sent:\n{:#?}", message),
        Err(_) => println!("Could not respond to command"),
    };
}
