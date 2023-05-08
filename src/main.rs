mod connection;
mod logic;
mod protocol;

use std::{net::TcpListener, thread};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").expect("Could not bind to port 6379");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| connection::handle(stream, logic::compute_response));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
