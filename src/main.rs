#![feature(conservative_impl_trait, generators, integer_atomics, proc_macro)]

extern crate futures_await as futures;
extern crate futures_black_hole;
extern crate hamt_sync;

mod core;

use core::*;

fn main() {
    println!("{:#?}", Value::Normal(Normal::Number(42.0)));
}
