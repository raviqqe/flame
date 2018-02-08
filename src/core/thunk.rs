use std::cell::UnsafeCell;
use std::sync::Arc;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering::SeqCst;

use futures::prelude::*;
use futures_black_hole::BlackHole;

use super::arguments::Arguments;
use super::blur_normal::BlurNormal;
use super::result::Result;
use super::value::Value;

#[derive(Clone, Debug)]
pub struct Thunk(Arc<UnsafeCell<Inner>>);

impl Thunk {
    pub fn new(f: Value, a: Arguments) -> Thunk {
        Thunk(Arc::new(UnsafeCell::new(Inner::new(f, a))))
    }

    #[async(boxed_send)]
    pub fn eval(self) -> Result<BlurNormal> {
        if self.inner_mut().lock() {
            self.inner_mut().content = Content::Normal(match self.inner().content.clone() {
                Content::App(v, a) => await!(await!(await!(v.function())?.call(a))?.blur()),
                Content::Normal(_) => unreachable!(),
            });

            self.inner().black_hole.release()?;
        } else {
            // This block is basically:
            // await!(&self.inner_mut().black_hole)?;
            loop {
                match self.inner_mut().black_hole.poll()? {
                    Async::Ready(()) => break,
                    Async::NotReady => yield Async::NotReady,
                }
            }
        }

        match self.inner().content {
            Content::App(_, _) => unreachable!(),
            Content::Normal(ref r) => r.clone(),
        }
    }

    fn inner(&self) -> &Inner {
        unsafe { &*self.0.get() }
    }

    fn inner_mut(&self) -> &mut Inner {
        unsafe { &mut *self.0.get() }
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
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
enum Content {
    App(Value, Arguments),
    Normal(Result<BlurNormal>),
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
        ) == State::App
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::error::Error;

    #[test]
    fn new() {
        Thunk::new(Value::from(0.0), Arguments::new(&[], &[], &[]));
    }

    #[test]
    fn eval_error() {
        let e: Error = Thunk::new(Value::from(42.0), Arguments::new(&[], &[], &[]))
            .eval()
            .wait()
            .unwrap_err();

        assert_eq!(e.name, "TypeError");
        assert_eq!(e.message, "42 is not a function");
    }
}
