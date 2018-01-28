use std::fmt;
use std::fmt::{Debug, Formatter};
use std::sync::*;

use super::arguments::Arguments;
use super::dictionary::Dictionary;
use super::error::Error;
use super::list::List;
use super::value::Value;

use self::Normal::*;

#[derive(Clone)]
pub enum Normal {
    Bool(bool),
    Dict(Dictionary),
    Error(Error),
    Func(Arc<Fn(Arguments) -> Value>),
    List(List),
    Nil,
    Num(f64),
    Str(String),
}

impl Debug for Normal {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match *self {
            Normal::Func(_) => write!(f, "<func>"),
            ref x => write!(f, "{:?}", x),
        }
    }
}

impl From<f64> for Normal {
    fn from(n: f64) -> Self {
        Normal::Num(n)
    }
}

impl From<List> for Normal {
    fn from(l: List) -> Self {
        Normal::List(l)
    }
}
