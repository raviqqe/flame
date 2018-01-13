use std::cell::*;
use std::sync::*;
use std::sync::atomic::*;
use std::sync::atomic::Ordering::*;

use arguments::*;
use value::*;
use self::Content::*;

#[derive(Clone, Debug)]
pub struct Thunk(Arc<RefCell<Inner>>);

impl Thunk {
    pub fn normal(v: Value) -> Thunk {
        return Thunk(Arc::new(RefCell::new(Inner {
            state: AtomicU8::new(State::Normal as u8),
            content: Content::Normal(v),
        })));
    }
}

#[derive(Debug, Eq, PartialEq)]
enum State {
    App,
    Normal,
}

impl From<u8> for State {
    fn from(u: u8) -> Self {
        match u {
            1 => State::App,
            0 => State::Normal,
            _ => panic!(""),
        }
    }
}

#[derive(Clone, Debug)]
enum Content {
    App(Value, Arguments),
    Normal(Value),
}

#[derive(Debug)]
struct Inner {
    state: AtomicU8,
    content: Content,
}

impl Inner {
    pub fn eval(&mut self) -> Value {
        match self.content.clone() {
            App(v, a) => v,
            Normal(v) => return v,
        }
    }

    fn lock(&mut self, old: State, new: State) -> bool {
        State::from(self.state.compare_and_swap(old as u8, new as u8, SeqCst)) != State::Normal
    }
}
