extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate log;

use std::env;
use std::io::Write;
use std::net::TcpListener;
use std::path;
use std::process::exit;

use clap::{App, Arg};
use log::Level;
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;

use kvs::{KvsCmd, KvsEngine, KvsError, KvStore, Result};

fn main() -> Result<()> {
    env_logger::init();
    error!("servr starting...");
    // parse args
    let matches = App::new("kvs-server")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::with_name("addr").long("addr").takes_value(true).required(false))
        .arg(Arg::with_name("engine").long("engine").takes_value(true).required(false))
        .arg(Arg::with_name("V").short("V"))
        .get_matches();

    if matches.occurrences_of("V") > 0 {
        println!("Version {}", env!("CARGO_PKG_VERSION"));
        exit(0);
    }

    let addr: String;
    match matches.value_of("addr") {
        None => {
            addr = String::from("127.0.0.1:4000");
        }
        Some(val) => {
            addr = val.to_owned();
        }
    }

    let eng: String;
    match matches.value_of("engine") {
        None => {
            eng = String::from("kvs");
        }
        Some(val) => {
            eng = val.to_owned();
        }
    }

    if eng != "kvs" && eng != "sled" {
        panic!("invalid engine");
    }

    error!("Version: {:?}, Addr: {:?}, Engine: {:?}", env!("CARGO_PKG_VERSION"), addr, eng);

    // open DB and get a handler
    let p = path::Path::new("./");
    let mut db = KvStore::open(&p)?;

    // listen network
    let listener = TcpListener::bind(addr).unwrap();
    for stream in listener.incoming() {
        let mut conn = stream.unwrap();
        let mut stream = Deserializer::from_reader(&conn).into_iter::<KvsCmd>();
        if let Some(cmd) = stream.next() {
            match cmd? {
                KvsCmd::Rm { key } => {
                    db.remove(key)?;
                }
                KvsCmd::Set { key, val } => {
                    db.set(key, val)?;
                }
                KvsCmd::Get { key } => {
                    match db.get(key)? {
                        None => {
                            conn.write("<nil>".as_bytes());
                        }
                        Some(val) => {
                            conn.write(val.into_bytes().as_slice());
                        }
                    }
                }
            }
        }

        println!("Connection established!");
    }
    Ok(())
}
