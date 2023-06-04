mod command;
mod connection;
mod expiry;
mod logic;
mod protocol;
mod store;

use std::{
    net::TcpListener,
    sync::{Arc, Mutex},
    thread,
};
use store::Store;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").expect("Could not bind to port 6379");
    let store = Arc::new(Mutex::new(Store::new()));

    for stream in listener.incoming() {
        let store = store.clone();
        match stream {
            Ok(stream) => {
                thread::spawn(move || connection::handle(stream, store, logic::run_command));
            }

            Err(e) => {
                println!("Error listening incoming stream: {}", e);
            }
        }
    }
}
