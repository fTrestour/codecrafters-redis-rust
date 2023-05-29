mod command;
mod connection;
mod logic;
mod protocol;

use std::{
    collections::HashMap,
    net::TcpListener,
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").expect("Could not bind to port 6379");
    let store = Arc::new(Mutex::new(HashMap::<String, String>::new()));

    for stream in listener.incoming() {
        let store = store.clone();
        match stream {
            Ok(stream) => {
                thread::spawn(move || connection::handle(stream, store, logic::run_command));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
