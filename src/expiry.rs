use std::time::Instant;

#[derive(Debug)]
pub enum Expiry {
    Infinity,
    ExpiresAt(Instant),
}

impl Expiry {
    pub fn is_expired(&self) -> bool {
        match self {
            Expiry::ExpiresAt(expiration_instant) => {
                match Instant::now().checked_duration_since(*expiration_instant) {
                    Some(_) => return true,
                    None => return false,
                }
            }
            Expiry::Infinity => return false,
        };
    }
}
