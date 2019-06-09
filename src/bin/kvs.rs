extern crate clap;

use std::env;
use std::path;
use std::process::exit;

use clap::{App, Arg};

use kvs::{KvsError, KvStore, Result};

fn main() -> Result<()> {
    let matches = App::new("kvs")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::with_name("cmd").index(1).required(false))
        .arg(Arg::with_name("key").index(2).required(false))
        .arg(Arg::with_name("val").index(3).required(false))
        .arg(Arg::with_name("V").short("V"))
        .get_matches();

    if matches.occurrences_of("V") > 0 {
        println!("Version {}", env!("CARGO_PKG_VERSION"));
        exit(0);
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
