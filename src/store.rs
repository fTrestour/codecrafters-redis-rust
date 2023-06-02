use crate::expiry::Expiry;
use std::collections::HashMap;

type Value = (String, Expiry);
pub struct Store {
    data: HashMap<String, Value>,
}

impl Store {
    pub fn new() -> Store {
        return Store {
            data: HashMap::<String, Value>::new(),
        };
    }

    pub fn set(&mut self, k: String, v: String) -> () {
        self.set_with_expiry(k, v, Expiry::Infinity);
    }

    pub fn set_with_expiry(&mut self, k: String, v: String, expiry: Expiry) -> () {
        self.data.insert(k, (v, expiry));
    }

    pub fn get(&self, k: String) -> Option<String> {
        return self.data.get(&k).and_then(|(v, expiry)| {
            if expiry.is_expired() {
                return None;
            }

            return Some(v.clone());
        });
    }
}
