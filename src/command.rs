use std::time::{Duration, Instant};

use crate::{expiry::Expiry, protocol::Resp};

#[derive(Debug)]
pub enum Command {
    Ping,
    Echo(String),
    Set(String, String, Expiry),
    Get(String),
}

impl Command {
    pub fn from_resp(message: Resp) -> Option<Command> {
        match message {
            Resp::SimpleString(s) | Resp::BulkString(s) if s.eq("PING") => Some(Command::Ping),

            Resp::Array(a) => match a.get(0)? {
                Resp::SimpleString(s) | Resp::BulkString(s) if s.eq("PING") => Some(Command::Ping),

                Resp::BulkString(s) if s.eq(&"ECHO") => match a.get(1)? {
                    Resp::SimpleString(s) | Resp::BulkString(s) => {
                        Some(Command::Echo(String::from(s)))
                    }
                    _ => None,
                },

                Resp::BulkString(s) if s.eq(&"GET") => match a.get(1)? {
                    Resp::SimpleString(s) | Resp::BulkString(s) => Some(Command::Get(s.clone())),
                    _ => None,
                },

                Resp::BulkString(s) if s.eq(&"SET") => match a.get(1)? {
                    Resp::SimpleString(k) | Resp::BulkString(k) => match a.get(2)? {
                        Resp::SimpleString(v) | Resp::BulkString(v) => match a.get(3) {
                            None => Some(Command::Set(
                                String::from(k),
                                String::from(v),
                                Expiry::Infinity,
                            )),

                            Some(Resp::SimpleString(s)) | Some(Resp::BulkString(s))
                                if s.eq("PX") =>
                            {
                                match a.get(4)? {
                                    Resp::SimpleString(d) | Resp::BulkString(d) => {
                                        let d = d.parse::<u64>().ok()?;
                                        return Some(Command::Set(
                                            String::from(k),
                                            String::from(v),
                                            Expiry::ExpiresAt(
                                                Instant::now() + Duration::from_millis(d),
                                            ),
                                        ));
                                    }
                                    _ => None,
                                }
                            }

                            _ => None,
                        },
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        }
    }
}
