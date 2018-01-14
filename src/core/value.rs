use super::thunk;
use super::normal::Normal;

#[derive(Clone, Debug)]
pub enum Value {
    Invalid,
    Normal(Normal),
    Thunk(thunk::Thunk),
}
