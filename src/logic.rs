use crate::protocol::Resp;

pub fn compute_response(message: Resp) -> Resp {
    match message {
        Resp::SimpleString(_) => Resp::pong(),

        Resp::BulkString(_) => Resp::pong(),

        Resp::Array(a) => {
            let mut result = Resp::pong();

            let first = a.get(0).expect("Received empty array");
            if let Resp::BulkString(s) = first {
                if s.eq(&"ECHO") {
                    let second = a.get(1).expect("Received size 1 array");

                    if let Resp::BulkString(second) = second {
                        result = Resp::SimpleString(second);
                    }
                }
            };
            result
        }
    }
}
