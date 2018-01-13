#![feature(integer_atomics)]

mod arguments;
mod thunk;
mod value;

use value::*;

fn main() {
    println!("{:#?}", Value::Num(42.0));
}
