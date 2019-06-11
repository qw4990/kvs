extern crate clap;

use std::env;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path;
use std::process::exit;

use clap::{App, Arg};

use kvs::{KvsEngine, KvsError, KvStore, Result};

fn main() -> Result<()> {
    let matches = App::new("kvs-client")
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

    // connect to database
    let mut db_stream = TcpStream::connect("127.0.0.1:34254")?;
    let argc = env::args().len();
    match matches.value_of("cmd").unwrap() {
        "get" => {
            if argc != 3 {
                panic!("invalid");
            }
            let key = matches.value_of("key").unwrap().to_owned();
            match get(&mut db_stream, key)? {
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
            set(&mut db_stream, key, val)?;
        }
        "rm" => {
            let key = matches.value_of("key").unwrap().to_owned();
            rm(&mut db_stream, key)?;
        }
        _ => {
            panic!("unimplemented");
        }
    }
    Ok(())
}

fn get(db_stream: &mut TcpStream, key: String) -> Result<Option<String>> {
    Ok(Some("xxx".to_owned()))
}

fn set(db_stream: &mut TcpStream, key: String, val: String) -> Result<()> {
    Ok(())
}

fn rm(db_stream: &mut TcpStream, key: String) -> Result<()> {
    Ok(())
}