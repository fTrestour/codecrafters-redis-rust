use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").expect("Could not bind to port 6379");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    println!("Accepted new connection");

    let mut buf = [0; 512];

    loop {
        let bytes_read = stream
            .read(&mut buf)
            .expect("Could not read bytes from connection");

        if bytes_read == 0 {
            println!("Closing connection");
            break;
        }

        stream.write_all(b"+PONG\r\n").expect("Could not respond")
    }
}
