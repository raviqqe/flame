#![feature(conservative_impl_trait, generators, integer_atomics, proc_macro)]

extern crate futures_await as futures;
extern crate futures_black_hole;

mod arguments;
mod thunk;
mod value;
mod normal;

use value::*;
use normal::*;

fn main() {
    println!("{:#?}", Value::Normal(Normal::Num(42.0)));
}
