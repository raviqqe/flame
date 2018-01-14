use thunk;
use normal::Normal;

#[derive(Clone, Debug)]
pub enum Value {
    Normal(Normal),
    Thunk(thunk::Thunk),
}
