use std::fmt::{Debug, Error, Formatter};
use std::sync::*;

use arguments::*;
use value::Value;

use self::Normal::*;

#[derive(Clone)]
pub enum Normal {
    Func(Arc<Fn(Arguments) -> Value>),
    Num(f64),
}

impl Debug for Normal {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Func(_) => write!(f, "<func>"),
            Num(n) => write!(f, "{}", n),
        }
    }
}
