use std::cell::UnsafeCell;
use std::sync::Arc;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering::SeqCst;

use arguments::*;
use value::*;
use self::Content::*;

#[derive(Clone, Debug)]
pub struct Thunk(Arc<UnsafeCell<Inner>>);

impl Thunk {
    pub fn normal(v: Value) -> Thunk {
        return Thunk(Arc::new(UnsafeCell::new(Inner {
            state: AtomicU8::new(State::Normal as u8),
            content: Content::Normal(v),
        })));
    }

    pub fn eval(mut self) -> Value {
        unsafe { &mut *self.0.get() }.eval()
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
