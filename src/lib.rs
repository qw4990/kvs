use std::collections::HashMap;
use std::convert::From;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::result;

use failure::Fail;
use serde::{Deserialize, Serialize};

/// A KV store.
pub struct KvStore {
    dir: PathBuf,
}

/// Error type for kvs
#[derive(Fail, Debug)]
pub enum KvsError {
    #[fail(display = "Unexpected command type")]
    UnexpectedCommandType,
    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::Error),
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),
}

impl From<serde_json::error::Error> for KvsError {
    fn from(e: serde_json::error::Error) -> Self {
        KvsError::Serde(e)
    }
}

impl From<io::Error> for KvsError {
    fn from(e: io::Error) -> Self {
        KvsError::Io(e)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum KvsCmd {
    Rm { key: String },
    Set { key: String, val: String },
}

/// Result type for kvs
pub type Result<T> = std::result::Result<T, KvsError>;

impl KvStore {
    pub fn open(dir: &Path) -> Result<KvStore> {
        Ok(KvStore { dir: dir.to_path_buf() })
    }

    fn db_file_path(&self) -> PathBuf {
        self.dir.join(Path::new("data.db"))
    }

    fn recover_index(&self) -> HashMap<String, String> {
        HashMap::new()
    }

    /// Set a new KV.
    pub fn set(&mut self, key: String, val: String) -> Result<()> {
        let cmd = KvsCmd::Set { key, val };
        let ser = serde_json::to_string(&cmd)?;
        fs::write(self.db_file_path().as_path(), ser)?;
        Ok(())
    }

    /// Get ...
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        Ok(None)
    }

    /// Remove ...
    pub fn remove(&mut self, key: String) -> Result<()> {
        Ok(())
    }
}
