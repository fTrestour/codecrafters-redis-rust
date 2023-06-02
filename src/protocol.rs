#[derive(Debug)]
pub enum Resp {
    SimpleString(String),
    // Error(String),
    // Integer(&i64),
    BulkString(String),
    Array(Vec<Resp>),
}

impl Resp {
    pub fn pong() -> Resp {
        Resp::SimpleString(String::from("PONG"))
    }

    pub fn ok() -> Resp {
        Resp::SimpleString(String::from("OK"))
    }

    pub fn from(s: &str) -> Resp {
        Self::parse(s).0
    }

    pub fn to_string(&self) -> String {
        match self {
            Resp::SimpleString(s) => format!("+{s}{CRLF}"),
            Resp::BulkString(s) => {
                let size = s.len();
                return format!("${size}{CRLF}{s}{CRLF}");
            }
            Resp::Array(a) => {
                let size = a.len();
                let mut result = format!("*{size}{CRLF}");

                for element in a {
                    result += &(element.to_string() + CRLF);
                }

                return result;
            }
        }
    }

    fn parse(s: &str) -> (Resp, &str) {
        let (prefix, value) = s.split_at(1);

        match prefix {
            "+" => {
                let (value, rest) = split_once_on_crlf(value);
                return (Resp::SimpleString(value.to_owned()), rest);
            }
            "$" => {
                let (_, value) = split_once_on_crlf(value);
                let (value, rest) = split_once_on_crlf(value);
                return (Resp::BulkString(value.to_owned()), rest);
            }
            "*" => {
                let (size, mut value) = split_once_on_crlf(value);

                let size: i32 = size.parse().expect("Could not parse string");

                let rest = "";
                let mut array = Vec::<Resp>::new();
                for _ in 0..size {
                    let parsed: (Resp, &str) = Resp::parse(value);

                    array.push(parsed.0);
                    value = parsed.1;
                }

                return (Resp::Array(array), rest);
            }
            _ => return (Resp::SimpleString(String::from("PONG")), ""),
        };
    }
}

const CRLF: &str = "\r\n";

fn split_once_on_crlf(s: &str) -> (&str, &str) {
    s.split_once(CRLF).expect("Error parsing on CRLF")
}
