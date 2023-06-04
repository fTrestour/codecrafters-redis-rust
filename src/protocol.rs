#[derive(Debug)]
pub enum Resp {
    SimpleString(String),
    BulkString(String),
    Array(Vec<Resp>),
    Null,
    Error(String),
}

impl TryFrom<&str> for Resp {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, <Resp as TryFrom<&str>>::Error> {
        Ok(Self::parse(value)?.0)
    }
}

impl Resp {
    pub fn pong() -> Resp {
        Resp::SimpleString(String::from("PONG"))
    }

    pub fn ok() -> Resp {
        Resp::SimpleString(String::from("OK"))
    }

    pub fn render(&self) -> String {
        match self {
            Resp::SimpleString(s) => format!("+{s}{CRLF}"),
            Resp::Error(s) => format!("-{s}{CRLF}"),
            Resp::BulkString(s) => {
                let size = s.len();
                return format!("${size}{CRLF}{s}{CRLF}");
            }
            Resp::Null => format!("$-1{CRLF}"),
            Resp::Array(a) => {
                let size = a.len();
                let mut result = format!("*{size}{CRLF}");

                for element in a {
                    result += &(element.render() + CRLF);
                }

                return result;
            }
        }
    }

    fn parse(s: &str) -> Result<(Resp, &str), &'static str> {
        let (prefix, value) = s.split_at(1);

        match prefix {
            "+" => {
                let (value, rest) = Self::split_once_on_crlf(value)?;
                return Ok((Resp::SimpleString(value.to_owned()), rest));
            }

            "$" => {
                let (_, value) = Self::split_once_on_crlf(value)?;
                let (value, rest) = Self::split_once_on_crlf(value)?;
                return Ok((Resp::BulkString(value.to_owned()), rest));
            }

            "*" => {
                let (size, mut value) = Self::split_once_on_crlf(value)?;

                let size: i32 = size.parse().or(Err("Could not parse string"))?;

                let rest = "";
                let mut array = Vec::<Resp>::new();
                for _ in 0..size {
                    let parsed = Resp::parse(value)?;

                    array.push(parsed.0);
                    value = parsed.1;
                }

                return Ok((Resp::Array(array), rest));
            }

            _ => return Err("Unknown prefix"),
        };
    }

    fn split_once_on_crlf(s: &str) -> Result<(&str, &str), &'static str> {
        s.split_once(CRLF).ok_or("Error parsing on CRLF")
    }
}

const CRLF: &str = "\r\n";
