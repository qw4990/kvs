use std::collections::HashMap;
use std::convert::From;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::result;

use clap::ErrorKind;
use failure::Fail;
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;

pub trait KvsEngine {
    fn set(&mut self, key: String, value: String) -> Result<()>;
    fn get(&mut self, key: String) -> Result<Option<String>>;
    fn remove(&mut self, key: String) -> Result<()>;
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
    #[fail(display = "None")]
    None,
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
    Get { key: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KvsResp {
    pub val: String,
    pub not_found: bool,
    pub err_code: i32,
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
        let mut f: File;
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
        f.seek(SeekFrom::Start(0))?;
        let mut stream = Deserializer::from_reader(f).into_iter::<KvsCmd>();
        while let Some(cmd) = stream.next() {
            match cmd? {
                KvsCmd::Rm { key } => {
                    self.kvs.remove(&key);
                }
                KvsCmd::Set { key, val } => {
                    self.kvs.insert(key, val);
                }
                _ => {
                    panic!("cannot happen");
                }
            }
        }
        Ok(())
    }
}

impl KvsEngine for KvStore {
    /// Set a new KV.
    fn set(&mut self, key: String, val: String) -> Result<()> {
        let cmd = KvsCmd::Set { key: key.clone(), val: val.clone() };
        let ser = serde_json::to_string(&cmd)?;
        let mut f = fs::OpenOptions::new().create(true).
            write(true).append(true).open(self.db_file_path().as_path())?;
        f.write_all(ser.into_bytes().as_slice())?;
        self.kvs.insert(key, val);
        Ok(())
    }

    /// Get ...
    fn get(&mut self, key: String) -> Result<Option<String>> {
        Ok(self.kvs.get(&key).cloned())
    }

    /// Remove ...
    fn remove(&mut self, key: String) -> Result<()> {
        if !self.kvs.contains_key(&key) {
            println!("Key not found");
            exit(1);
        }

        let cmd = KvsCmd::Rm { key: key.clone() };
        let ser = serde_json::to_string(&cmd)?;
        fs::write(self.db_file_path().as_path(), ser)?;
        self.kvs.remove(&key);
        Ok(())
    }
}