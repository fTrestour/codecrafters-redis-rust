use std::sync::{Arc, Mutex};

use crate::{command::Command, protocol::Resp, store::Store};

pub fn run_command(command: Command, store: &Arc<Mutex<Store>>) -> Resp {
    match command {
        Command::Ping => Resp::pong(),
        Command::Echo(s) => Resp::BulkString(String::from(s)),
        Command::Set(k, v) => {
            let mut store = store.lock().unwrap();
            store.set(k, v);

            return Resp::ok();
        }
        Command::Get(k) => {
            let store = store.lock().unwrap();
            let v = store.get(k);
            match v {
                Some(v) => return Resp::SimpleString(v.to_owned()),
                None => todo!(),
            };
        }
    }
}
