use std::time::{Duration, Instant};

use crate::{expiry::Expiry, protocol::Resp};

#[derive(Debug)]
pub enum Command {
    Ping,
    Echo(String),
    Set(String, String, Expiry),
    Get(String),
}

impl TryFrom<Resp> for Command {
    type Error = &'static str;

    fn try_from(message: Resp) -> Result<Self, Self::Error> {
        match message {
            Resp::SimpleString(s) | Resp::BulkString(s) if s.eq("PING") => Ok(Command::Ping),
            Resp::SimpleString(_) | Resp::BulkString(_) => Err("Unexpected string"),
            Resp::Array(a) => Self::handle_command(&a),
            Resp::Null => Err("Unexpected null"),
        }
    }
}

impl Command {
    fn handle_command(a: &Vec<Resp>) -> Result<Command, &'static str> {
        match a.get(0).ok_or("Command is missing")? {
            Resp::SimpleString(s) | Resp::BulkString(s) if s.eq("PING") => Ok(Command::Ping),

            Resp::BulkString(s) if s.eq(&"ECHO") => {
                match a.get(1).ok_or("ECHO expects 1 argument and got 0")? {
                    Resp::SimpleString(s) | Resp::BulkString(s) => {
                        Ok(Command::Echo(String::from(s)))
                    }
                    _ => Err("ECHO expects a string argument"),
                }
            }

            Resp::BulkString(s) if s.eq(&"GET") => {
                match a.get(1).ok_or("GET expects 1 argument and got 0")? {
                    Resp::SimpleString(s) | Resp::BulkString(s) => Ok(Command::Get(s.clone())),
                    _ => Err("GET expects a string argument"),
                }
            }

            Resp::BulkString(s) if s.eq(&"SET") => Self::handle_set(a),
            _ => Err("Unknown command"),
        }
    }

    fn handle_set(a: &Vec<Resp>) -> Result<Command, &'static str> {
        match a.get(1).ok_or("SET expects 2 arguments and got 0")? {
            Resp::SimpleString(k) | Resp::BulkString(k) => {
                match a.get(2).ok_or("SET expects 2 arguments and got 1")? {
                    Resp::SimpleString(v) | Resp::BulkString(v) => match a.get(3) {
                        None => Ok(Command::Set(
                            String::from(k),
                            String::from(v),
                            Expiry::Infinity,
                        )),

                        Some(Resp::SimpleString(s)) | Some(Resp::BulkString(s)) if s.eq("PX") => {
                            Self::handle_expiry(k, v, a)
                        }

                        _ => Err("SET expects 2 arguments and got more"),
                    },
                    _ => Err("SET only accepts string values"),
                }
            }
            _ => Err("SET only accepts string keys"),
        }
    }

    fn handle_expiry(k: &str, v: &str, a: &Vec<Resp>) -> Result<Command, &'static str> {
        match a.get(4).ok_or("PX expects 1 argument and got 0")? {
            Resp::SimpleString(d) | Resp::BulkString(d) => {
                let d = d.parse::<u64>().or(Err("PX expects an integer argument"))?;
                return Ok(Command::Set(
                    String::from(k),
                    String::from(v),
                    Expiry::ExpiresAt(Instant::now() + Duration::from_millis(d)),
                ));
            }
            _ => Err("PX expects an integer argument"),
        }
    }
}
