use std::collections::HashMap;

pub struct KvStore {
    m: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore { m: HashMap::new() }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.m.insert(key, value);
    }

    pub fn get(&mut self, key: String) -> Option<String> {
        match self.m.get(&key) {
            None => None,
            Some(val) => Some(val.to_owned()),
        }
    }

    pub fn remove(&mut self, key: String) {
        self.m.remove(&key);
    }
}