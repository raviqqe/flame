use thunk;
use normal::Normal;

#[derive(Clone, Debug)]
pub enum Value {
    Invalid,
    Normal(Normal),
    Thunk(thunk::Thunk),
}
