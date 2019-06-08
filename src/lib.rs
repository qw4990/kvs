use std::collections::HashMap;

/// A KV store.
pub struct KvStore {
    m: HashMap<String, String>,
}

impl KvStore {
    /// Creates a KvStore.
    pub fn new() -> KvStore {
        KvStore { m: HashMap::new() }
    }

    /// Set a new KV.
    pub fn set(&mut self, key: String, value: String) {
        self.m.insert(key, value);
    }

    /// Get ...
    pub fn get(&mut self, key: String) -> Option<String> {
        self.m.get(&key).cloned()
    }

    /// Remove ...
    pub fn remove(&mut self, key: String) {
        self.m.remove(&key);
    }
}
