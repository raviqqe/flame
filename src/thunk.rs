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
    pub fn new(f: Value, a: Arguments) -> Thunk {
        return Thunk(Arc::new(UnsafeCell::new(Inner::new(f, a))));
    }

    pub fn eval(self) -> Value {
        unsafe { &mut *self.0.get() }.eval()
    }
}

#[derive(Debug, Eq, PartialEq)]
#[repr(u8)]
enum State {
    App = 0,
    Normal = 1,
}

impl From<u8> for State {
    fn from(u: u8) -> Self {
        match u {
            0 => State::App,
            1 => State::Normal,
            _ => panic!("Invalid value"),
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
    pub fn new(f: Value, a: Arguments) -> Self {
        Inner {
            state: AtomicU8::new(State::App as u8),
            content: App(f, a),
        }
    }

    pub fn eval(&mut self) -> Value {
        match self.content.clone() {
            App(v, a) => v,
            Normal(v) => v,
        }
    }

    fn lock(&mut self, old: State, new: State) -> bool {
        State::from(self.state.compare_and_swap(old as u8, new as u8, SeqCst)) != State::Normal
    }
}
