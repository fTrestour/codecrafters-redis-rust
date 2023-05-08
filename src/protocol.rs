#[derive(Debug)]
pub enum Resp<'a> {
    SimpleString(&'a str),
    // Error(&'a str),
    // Integer(&'a i64),
    BulkString(&'a str),
    Array(Vec<Resp<'a>>),
}

impl Resp<'_> {
    pub fn pong() -> Resp<'static> {
        Resp::SimpleString("PONG")
    }

    pub fn from_str<'a>(s: &'a str) -> Resp<'a> {
        Self::parse(s).0
    }

    pub fn to_str(&self) -> String {
        match self {
            Resp::SimpleString(s) => {
                let mut result = String::new();
                result.push_str("+");

                result.push_str(&s);
                result.push_str(CRLF);
                result
            }
            Resp::BulkString(s) => {
                let mut result = String::new();
                result.push_str("$");

                result.push_str(&s.len().to_string());
                result.push_str(CRLF);

                result.push_str(&s);
                result.push_str(CRLF);

                result
            }
            Resp::Array(a) => {
                let mut result: String = String::new();
                result.push_str("*");

                result.push_str(&a.len().to_string());
                result.push_str(CRLF);

                for element in a {
                    let s = element.to_str();
                    result.push_str(&s);
                    result.push_str(CRLF);
                }

                result
            }
        }
    }

    fn parse(s: &str) -> (Resp, &str) {
        let (prefix, value) = s.split_at(1);

        if prefix.eq("+") {
            let (value, rest) = split_once_on_crlf(value);

            return (Resp::SimpleString(value), rest);
        } else if prefix.eq("$") {
            let (_, value) = split_once_on_crlf(value);

            let (value, rest) = split_once_on_crlf(value);

            return (Resp::BulkString(value), rest);
        } else if prefix.eq("*") {
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
        } else {
            return (Resp::SimpleString("PONG"), "");
        }
    }
}

const CRLF: &str = "\r\n";

fn split_once_on_crlf(s: &str) -> (&str, &str) {
    s.split_once(CRLF).expect("Error parsing on CRLF")
}
