extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("kvs").arg(Arg::with_name("cmd"));

    println!("Hello, world!");
}
