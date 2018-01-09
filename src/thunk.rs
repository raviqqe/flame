use std::cell::*;
use std::sync::*;
use std::sync::atomic::*;

use value::*;

#[derive(Debug)]
pub struct Thunk(Arc<RefCell<ThunkInner>>);

#[derive(Debug)]
enum State {
    Normal,
}

#[derive(Debug)]
struct ThunkInner {
    state: AtomicI8,
    content: Content,
}

#[derive(Debug)]
enum Content {
    Normal(Value),
}

impl Thunk {
    pub fn normal(v: Value) -> Thunk {
        return Thunk(Arc::new(RefCell::new(ThunkInner {
            state: AtomicI8::new(State::Normal as i8),
            content: Content::Normal(v),
        })));
    }
}
