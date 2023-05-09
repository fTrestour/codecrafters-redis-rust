use crate::protocol::Resp;

#[derive(Debug)]
pub enum Command {
    Ping,
    Echo(String),
    Set(String, String),
    Get(String),
}

impl Command {
    pub fn from_resp(message: Resp) -> Option<Command> {
        match message {
            Resp::SimpleString(s) => {
                if s.eq("PING") {
                    return Some(Command::Ping);
                } else {
                    return None;
                }
            }
            Resp::BulkString(s) => {
                if s.eq("PING") {
                    return Some(Command::Ping);
                } else {
                    return None;
                }
            }
            Resp::Array(a) => {
                let first = a.get(0);
                match first {
                    Some(Resp::BulkString(s)) => {
                        if s.eq("PING") {
                            return Some(Command::Ping);
                        } else if s.eq(&"ECHO") {
                            let second = a.get(1);
                            match second {
                                Some(Resp::SimpleString(s)) => {
                                    return Some(Command::Echo(String::from(s)))
                                }
                                Some(Resp::BulkString(s)) => {
                                    return Some(Command::Echo(String::from(s)))
                                }
                                _ => return None,
                            };
                        } else if s.eq(&"GET") {
                            let second = a.get(1);
                            match second {
                                Some(Resp::SimpleString(s)) => {
                                    return Some(Command::Get(String::from(s)))
                                }
                                Some(Resp::BulkString(s)) => {
                                    return Some(Command::Get(String::from(s)))
                                }
                                _ => return None,
                            };
                        } else if s.eq(&"SET") {
                            let second = a.get(1);
                            let third = a.get(2);
                            match (second, third) {
                                (Some(Resp::SimpleString(s)), Some(Resp::SimpleString(t))) => {
                                    return Some(Command::Set(String::from(s), String::from(t)))
                                }
                                (Some(Resp::SimpleString(s)), Some(Resp::BulkString(t))) => {
                                    return Some(Command::Set(String::from(s), String::from(t)))
                                }
                                (Some(Resp::BulkString(s)), Some(Resp::BulkString(t))) => {
                                    return Some(Command::Set(String::from(s), String::from(t)))
                                }
                                _ => return None,
                            };
                        } else {
                            return None;
                        }
                    }
                    _ => return None,
                }
            }
        };
    }
}
