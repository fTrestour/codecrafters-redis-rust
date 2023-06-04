use std::collections::HashMap;

use crate::expiry::Expiry;

type Key = String;
type Value = (String, Expiry);
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
        println!("Value {v} set for key {k}");
        self.data.insert(k, (v, expiry));
    }

    pub fn get(&self, k: String) -> Option<String> {
        let (v, expiry) = self.data.get(&k)?;

        if expiry.is_expired() {
            println!("Value for key {k} is expired");
            return None;
        }

        println!("Value for key {k} is {v}");
        return Some(v.clone());
    }
}
