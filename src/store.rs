use std::collections::HashMap;

use crate::expiry::Expiry;

pub type Key = String;
pub type Value = (String, Expiry);

pub struct Store {
    data: HashMap<Key, Value>,
}

impl Store {
    pub fn new() -> Store {
        return Store {
            data: HashMap::<Key, Value>::new(),
        };
    }

    pub fn set(&mut self, k: String, v: String, expiry: Expiry) -> () {
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
