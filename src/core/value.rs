use super::list::List;
use super::thunk;
use super::normal::Normal;

#[derive(Clone, Debug)]
pub enum Value {
    Invalid,
    Normal(Normal),
    Thunk(thunk::Thunk),
}

impl From<List> for Value {
    fn from(l: List) -> Self {
        Value::Normal(Normal::List(l))
    }
}
