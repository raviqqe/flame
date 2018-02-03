use std::cell::UnsafeCell;
use std::sync::Arc;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering::SeqCst;

use futures::prelude::*;
use futures_black_hole::BlackHole;

use super::arguments::Arguments;
use super::error::Error;
use super::normal::Normal;
use super::result::Result;
use super::value::Value;

#[derive(Clone, Debug)]
pub struct Thunk(Arc<UnsafeCell<Inner>>);

impl Thunk {
    pub fn new(f: Value, a: Arguments) -> Thunk {
        Thunk(Arc::new(UnsafeCell::new(Inner::new(f, a))))
    }

    #[async]
    pub fn eval(self) -> Result<Normal> {
        let i = unsafe { &mut *self.0.get() };

        if i.lock() {
            let c = i.content.clone();

            let v = match c {
                Content::App(v, _) => await!(v.normal())?,
                Content::Normal(_) => unreachable!(),
            };

            i.content = Content::Normal(v.clone());

            i.black_hole.release();
        } else {
            await!(i.black_hole.clone()).unwrap();
        }

        match i.content.clone() {
            Content::App(_, _) => unreachable!(),
            Content::Normal(v) => Ok(v),
        }
    }
}

impl Future for Thunk {
    type Item = Normal;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        // TODO: Remove *.clone() if possible.
        self.clone().eval().poll()
    }
}

unsafe impl Send for Thunk {}
unsafe impl Sync for Thunk {}

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
    Normal(Normal),
}

#[derive(Debug)]
struct Inner {
    state: AtomicU8,
    content: Content,
    black_hole: BlackHole,
}

impl Inner {
    pub fn new(f: Value, a: Arguments) -> Self {
        Inner {
            state: AtomicU8::new(State::App as u8),
            content: Content::App(f, a),
            black_hole: BlackHole::new(),
        }
    }

    fn lock(&mut self) -> bool {
        State::from(
            self.state
                .compare_and_swap(State::App as u8, State::Normal as u8, SeqCst),
        ) != State::App
    }
}

#[cfg(test)]
mod test {
    use std::thread::spawn;

    use super::*;

    #[test]
    fn new() {
        Thunk::new(Value::from(0.0), Arguments::new(&[], &[], &[]));
    }

    #[test]
    fn send_and_sync() {
        let t = Thunk::new(Value::from(0.0), Arguments::new(&[], &[], &[]));
        spawn(move || t);
    }
}
