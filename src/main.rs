#![feature(arbitrary_self_types, conservative_impl_trait, generators, generator_trait,
           integer_atomics, proc_macro, str_escape, test, try_from, type_ascription,
           universal_impl_trait)]

extern crate array_queue;
extern crate docopt;
extern crate futures_await as futures;
extern crate futures_stable as futures_stable;
extern crate hamt_sync;
#[macro_use]
extern crate lazy_static;
extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate pin_api;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
extern crate test;

use futures::executor::block_on;

#[macro_use]
mod core;
mod ast;
mod builtin;
mod compile;
mod desugar;
mod ir;
mod parse;
mod run;
mod third;

use std::error::Error;
use std::fs::File;
use std::io::{self, stdin, Read};
use std::process::exit;

use compile::compile;
use desugar::desugar;
use docopt::Docopt;
use run::run;

const USAGE: &'static str = "
The interpreter of Flame programming language.

Usage:
  flame [-h] [<filename>]

Options:
  -h, --help  Show this help.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_filename: Option<String>,
}

fn main() {
    try_main().unwrap_or_else(|e| {
        eprintln!("{}", e);
        exit(1);
    });
}

fn try_main() -> Result<(), Box<Error>> {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.deserialize())?;

    Ok(block_on(run(compile(desugar(parse::main_module(
        &read_source(args.arg_filename)?,
    )?)?)?))?)
}

fn read_source(s: Option<String>) -> Result<String, io::Error> {
    match s {
        Some(n) => read_file(File::open(n)?),
        None => read_file(stdin()),
    }
}

fn read_file<R: Read>(mut r: R) -> Result<String, io::Error> {
    let mut s = String::new();

    r.read_to_string(&mut s)?;

    Ok(s)
}
