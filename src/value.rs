use thunk::*;

#[derive(Debug)]
pub enum Value {
    Num(f64),
    Thunk(Thunk),
}
