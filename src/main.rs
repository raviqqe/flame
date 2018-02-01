#![feature(conservative_impl_trait, generators, integer_atomics, proc_macro)]

extern crate array_queue;
extern crate futures_await as futures;
extern crate futures_black_hole;
extern crate hamt_sync;
#[macro_use]
extern crate nom;

mod ast;
mod core;
mod parse;

use core::*;

fn main() {
    println!("{:#?}", Value::Normal(Normal::Number(42.0)));
}
