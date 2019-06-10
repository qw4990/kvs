extern crate clap;

use std::env;
use std::net::TcpListener;
use std::path;
use std::process::exit;

use clap::{App, Arg};

use kvs::{KvsEngine, KvsError, KvStore, Result};

fn main() -> Result<()> {
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

    // open DB and get a handler
    let p = path::Path::new("./");
    let mut db = KvStore::open(&p)?;

    // listen network
    let listener = TcpListener::bind(addr).unwrap();
    println!("Server start");
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
    Ok(())
}
