use std::{io::Write, net::TcpListener};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("Accepted new connection");

                _stream.write_all(b"+PONG\r\n").expect("Could not respond");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
