#![feature(conservative_impl_trait, generators, integer_atomics, proc_macro)]

extern crate futures_await as futures;
extern crate futures_black_hole;

mod arguments;
mod thunk;
mod value;

use value::*;

fn main() {
    println!("{:#?}", Value::Num(42.0));
}
