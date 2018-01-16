use std::fmt;
use std::fmt::{Display, Formatter};

use super::list::List;
use super::thunk;
use super::normal::Normal;

#[derive(Clone, Debug)]
pub enum Value {
    Invalid,
    Normal(Normal),
    Thunk(thunk::Thunk),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl From<List> for Value {
    fn from(l: List) -> Self {
        Value::Normal(Normal::List(l))
    }
}
