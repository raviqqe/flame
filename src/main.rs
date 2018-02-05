#![feature(conservative_impl_trait, generators, integer_atomics, proc_macro)]

extern crate array_queue;
extern crate docopt;
extern crate futures_await as futures;
extern crate futures_black_hole;
extern crate futures_cpupool;
extern crate hamt_sync;
#[macro_use]
extern crate lazy_static;
extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod ast;
mod compile;
mod core;
mod desugar;
mod parse;
mod run;

use std::error::Error;
use std::fs::File;
use std::io::{stdin, Read};
use std::process::exit;

use docopt::Docopt;
use run::run;

const USAGE: &'static str = "
The interpreter of Flame programming language.

Usage:
  flame [-h] [-v] [<filename>]

Options:
  -h --help     Show this help.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_filename: Option<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    println!("{}", read_source(args.arg_filename));

    // run(vec![]);
}

fn read_source(s: Option<String>) -> String {
    match s {
        Some(n) => read_file(File::open(n).unwrap_or_else(fail)),
        None => read_file(stdin()),
    }
}

fn read_file<R: Read>(mut r: R) -> String {
    let mut s = String::new();

    r.read_to_string(&mut s).unwrap_or_else(fail);

    s
}

fn fail<E: Error, R>(e: E) -> R {
    eprintln!("{}", e);
    exit(1);
}
