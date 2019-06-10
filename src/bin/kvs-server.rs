extern crate clap;

use std::env;
use std::path;
use std::process::exit;

use clap::{App, Arg};

use kvs::{KvsEngine, KvsError, KvStore, Result};

fn main() -> Result<()> {
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

    let p = path::Path::new("./");
    let mut db = KvStore::open(&p)?;
    let argc = env::args().len();

    match matches.value_of("cmd").unwrap() {
        "get" => {
            if argc != 3 {
                panic!("invalid");
            }
            let key = matches.value_of("key").unwrap().to_owned();
            match db.get(key)? {
                None => {
                    println!("Key not found");
                    exit(0);
                }
                Some(val) => {
                    println!("{}", val);
                }
            }
        }
        "set" => {
            let key = matches.value_of("key").unwrap().to_owned();
            let val = matches.value_of("val").unwrap().to_owned();
            db.set(key, val)?;
        }
        "rm" => {
            let key = matches.value_of("key").unwrap().to_owned();
            db.remove(key)?;
        }
        _ => {
            panic!("unimplemented");
        }
    }

    Ok(())
}
