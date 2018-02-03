#![feature(conservative_impl_trait, generators, integer_atomics, proc_macro)]

extern crate array_queue;
extern crate futures_await as futures;
extern crate futures_black_hole;
extern crate futures_cpupool;
extern crate hamt_sync;
#[macro_use]
extern crate lazy_static;
extern crate pest;
#[macro_use]
extern crate pest_derive;

mod ast;
mod compile;
mod core;
mod desugar;
mod parse;
mod run;

use core::*;

fn main() {
    println!("{:#?}", Value::Normal(Normal::Number(42.0)));
}
