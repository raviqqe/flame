use std::fmt::{Debug, Error, Formatter};
use std::sync::*;

use super::arguments::*;
use super::list;
use super::value::Value;

use self::Normal::*;

#[derive(Clone)]
pub enum Normal {
    Func(Arc<Fn(Arguments) -> Value>),
    List(list::List),
    Num(f64),
}

impl Debug for Normal {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Func(_) => write!(f, "<func>"),
            ref x => write!(f, "{:?}", x),
        }
    }
}
