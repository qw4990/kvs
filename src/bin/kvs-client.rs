extern crate clap;

use std::env;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path;
use std::process::exit;

use clap::{App, Arg};
use serde_json::Deserializer;

use kvs::{KvsCmd, KvsEngine, KvsError, KvsResp, KvStore, Result};

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
    let mut db_stream = TcpStream::connect("127.0.0.1:4000")?;
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

fn request(db_stream: &mut TcpStream, cmd: &KvsCmd) -> Result<KvsResp> {
    let data = serde_json::to_string(cmd)?;
    db_stream.write_all(data.as_bytes())?;
    let mut ret_data = String::new();
    db_stream.read_to_string(&mut ret_data)?;
    let mut stream = Deserializer::from_slice(&ret_data.as_bytes()).into_iter::<KvsResp>();
    if let Some(cmd) = stream.next() {
        return Ok(cmd?);
    }
    panic!("cannot happen");
}

fn get(db_stream: &mut TcpStream, key: String) -> Result<Option<String>> {
    let resp = request(db_stream, &KvsCmd::Get { key })?;
    if resp.not_found {
        Ok(None)
    } else {
        Ok(Some(resp.val))
    }
}

fn set(db_stream: &mut TcpStream, key: String, val: String) -> Result<()> {
    let resp = request(db_stream, &KvsCmd::Set { key, val })?;
    Ok(())
}

fn rm(db_stream: &mut TcpStream, key: String) -> Result<()> {
    let resp = request(db_stream, &KvsCmd::Rm { key })?;
    Ok(())
}