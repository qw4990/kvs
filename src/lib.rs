use std::collections::HashMap;
use std::io;
use std::path::Path;
use std::result;

use failure::Fail;

/// A KV store.
pub struct KvStore {
    m: HashMap<String, String>,
}

/// Error type for kvs
#[derive(Fail, Debug)]
pub enum KvsError {
    #[fail(display = "Unexpected command type")]
    UnexpectedCommandType,
}

/// Result type for kvs
pub type Result<T> = std::result::Result<T, KvsError>;

impl KvStore {
    /// Creates a KvStore.
    pub fn new() -> KvStore {
        KvStore { m: HashMap::new() }
    }

    pub fn open(p: &Path) -> Result<KvStore> {
        panic!("xxx");
    }

    /// Set a new KV.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.m.insert(key, value);
        Ok(())
    }

    /// Get ...
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        Ok(self.m.get(&key).cloned())
    }

    /// Remove ...
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.m.remove(&key);
        Ok(())
    }
}
