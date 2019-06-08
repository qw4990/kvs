extern crate clap;

use std::process::exit;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("kvs").version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::with_name("cmd").index(1).required(false))
        .arg(Arg::with_name("key").index(2).required(false))
        .arg(Arg::with_name("val").index(3).required(false))
        .arg(Arg::with_name("V").short("V"))
        .get_matches();

    if matches.occurrences_of("V") > 0 {
        println!("Version {}", env!("CARGO_PKG_VERSION"));
        exit(0);
    }
    exit(1);
}
