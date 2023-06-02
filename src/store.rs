use std::collections::HashMap;

pub struct Store {
    data: HashMap<String, String>,
}

impl Store {
    pub fn new() -> Store {
        return Store {
            data: HashMap::<String, String>::new(),
        };
    }

    pub fn set(&mut self, k: String, v: String) -> () {
        self.data.insert(k, v);
    }

    pub fn get(&self, k: String) -> Option<String> {
        return self.data.get(&k).cloned();
    }
}
