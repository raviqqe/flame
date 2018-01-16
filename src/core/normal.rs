use std::fmt;
use std::fmt::{Debug, Formatter};
use std::sync::*;

use super::arguments::Arguments;
use super::error::Error;
use super::list;
use super::value::Value;

use self::Normal::*;

#[derive(Clone)]
pub enum Normal {
    Error(Error),
    Func(Arc<Fn(Arguments) -> Value>),
    List(list::List),
    Num(f64),
}

impl Debug for Normal {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match *self {
            Func(_) => write!(f, "<func>"),
            ref x => write!(f, "{:?}", x),
        }
    }
}
