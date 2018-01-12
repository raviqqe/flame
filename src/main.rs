#![feature(integer_atomics)]

mod arguments;
mod thunk;
mod value;

use thunk::*;
use value::*;

fn main() {
    println!("{:#?}", Thunk::normal(Value::Num(42.0)));
}
