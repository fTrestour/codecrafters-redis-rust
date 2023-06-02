use std::time::Instant;

#[derive(Debug)]
pub enum Expiry {
    Infinity,
    ExpiresAt(Instant),
}

impl Expiry {
    pub fn is_expired(&self) -> bool {
        match self {
            Expiry::ExpiresAt(t) => return Instant::now() < *t,
            Expiry::Infinity => return false,
        };
    }
}
