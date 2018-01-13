use std::fmt::{Debug, Error, Formatter};
use std::sync::*;

use arguments::*;
use thunk;
use Value::*;

#[derive(Clone)]
pub enum Value {
    Func(Arc<Fn(Arguments) -> Value>),
    Num(f64),
    Thunk(thunk::Thunk),
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Func(_) => write!(f, "<func>"),
            Num(n) => write!(f, "{}", n),
            Thunk(ref t) => write!(f, "{:?}", t),
        }
    }
}
