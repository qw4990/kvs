use std::collections::HashMap;
use std::convert::From;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::result;

use clap::ErrorKind;
use failure::Fail;
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;

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

/// A KV store.
pub struct KvStore {
    dir: PathBuf,
    kvs: HashMap<String, String>,
}

impl KvStore {
    pub fn open(dir: &Path) -> Result<KvStore> {
        fs::create_dir_all(dir)?;
        let mut kv = KvStore { dir: dir.to_path_buf(), kvs: HashMap::new() };
        kv.load()?;
        Ok(kv)
    }

    fn db_file_path(&self) -> PathBuf {
        self.dir.join(Path::new("db.log"))
    }

    fn load(&mut self) -> Result<()> {
        let r = File::open(self.db_file_path().as_path());
        let f: File;
        match r {
            Err(e) => {
                match e.kind() {
                    std::io::ErrorKind::NotFound => {
                        return Ok(());
                    }
                    _ => {
                        println!("open file error: {:?} ", e);
                        exit(1);
                    }
                }
            }
            Ok(v) => {
                f = v
            }
        }
        let mut stream = Deserializer::from_reader(f).into_iter::<KvsCmd>();
        while let Some(cmd) = stream.next() {
            match cmd? {
                KvsCmd::Rm { key } => {
                    self.kvs.remove(&key);
                }
                KvsCmd::Set { key, val } => {
                    self.kvs.insert(key, val);
                }
            }
        }
        Ok(())
    }

    /// Set a new KV.
    pub fn set(&mut self, key: String, val: String) -> Result<()> {
        let cmd = KvsCmd::Set { key: key.clone(), val: val.clone() };
        let ser = serde_json::to_string(&cmd)?;
        fs::write(self.db_file_path().as_path(), ser)?;
        self.kvs.insert(key, val);
        Ok(())
    }

    /// Get ...
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        if !self.kvs.contains_key(&key) {
            println!("Key not found");
            exit(0);
        }
        Ok(None)
    }

    /// Remove ...
    pub fn remove(&mut self, key: String) -> Result<()> {
        if !self.kvs.contains_key(&key) {
            println!("Key not found");
            exit(0);
        }

        let cmd = KvsCmd::Rm { key: key.clone() };
        let ser = serde_json::to_string(&cmd)?;
        fs::write(self.db_file_path().as_path(), ser)?;
        self.kvs.remove(&key);
        Ok(())
    }
}
