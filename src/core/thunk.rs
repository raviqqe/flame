use std::cell::UnsafeCell;
use std::mem::replace;
use std::sync::Arc;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering::SeqCst;

use futures::prelude::*;
use futures_black_hole::BlackHole;

use super::arguments::Arguments;
use super::normal::Normal;
use super::vague_normal::VagueNormal;
use super::result::Result;
use super::utils::IDENTITY;
use super::value::Value;

#[derive(Clone, Debug)]
pub struct Thunk(Arc<UnsafeCell<Inner>>);

impl Thunk {
    pub fn new(f: Value, a: Arguments) -> Thunk {
        Thunk(Arc::new(UnsafeCell::new(Inner::new(f, a))))
    }

    #[async(boxed_send)]
    pub fn eval(self) -> Result<VagueNormal> {
        if self.inner_mut().lock(State::Normal) {
            loop {
                let (v, a) = self.app();

                match await!(v.function()) {
                    Err(e) => {
                        self.inner_mut().content = Content::Normal(Err(e));
                        break;
                    }
                    Ok(f) => match await!(f.call(a)) {
                        Err(e) => {
                            self.inner_mut().content = Content::Normal(Err(e));
                            break;
                        }
                        Ok(Value::Normal(n)) => {
                            self.inner_mut().content = Content::Normal(n);
                            break;
                        }
                        Ok(Value::Thunk(t)) => {
                            if !t.delegate_evaluation(&self) {
                                self.inner_mut().content = Content::Normal(await!(t.eval()));
                                break;
                            }
                        }
                    },
                }
            }

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

        self.normal()
    }

    fn inner(&self) -> &Inner {
        unsafe { &*self.0.get() }
    }

    fn inner_mut(&self) -> &mut Inner {
        unsafe { &mut *self.0.get() }
    }

    fn app(&self) -> (Value, Arguments) {
        match self.inner_mut().content {
            Content::App(ref mut f, ref mut a) => {
                let f = replace(f, Normal::Nil.into());
                let a = replace(a, Arguments::default());

                (f, a)
            }
            Content::Normal(_) => unreachable!(),
        }
    }

    fn normal(&self) -> Result<VagueNormal> {
        match self.inner().content {
            Content::App(_, _) => unreachable!(),
            Content::Normal(ref r) => r.clone(),
        }
    }

    fn delegate_evaluation(&self, t: &Thunk) -> bool {
        if !self.inner_mut().lock(State::SpinLock) {
            return false;
        }

        let (v, a) = self.app();

        t.inner_mut().content = Content::App(v, a);
        self.inner_mut().content = Content::App(
            IDENTITY.clone(),
            Arguments::positionals(&[t.clone().into()]),
        );

        return true;
    }
}

unsafe impl Send for Thunk {}
unsafe impl Sync for Thunk {}

#[derive(Debug, Eq, PartialEq)]
#[repr(u8)]
enum State {
    App = 0,
    Normal = 1,
    SpinLock = 2,
}

impl From<u8> for State {
    fn from(u: u8) -> Self {
        match u {
            0 => State::App,
            1 => State::Normal,
            2 => State::SpinLock,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
enum Content {
    App(Value, Arguments),
    Normal(Result<VagueNormal>),
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

    fn lock(&mut self, s: State) -> bool {
        loop {
            match State::from(self.state.load(SeqCst)) {
                State::Normal => break false,
                State::App => {
                    break State::from(self.state.compare_and_swap(
                        State::App as u8,
                        s as u8,
                        SeqCst,
                    )) == State::App
                }
                State::SpinLock => continue,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use futures_cpupool::CpuPool;

    use super::super::normal::Normal;
    use super::super::signature::Signature;
    use super::super::utils::papp;
    use super::*;

    #[test]
    fn new() {
        Thunk::new(Value::from(0.0), Arguments::new(&[], &[], &[]));
    }

    #[test]
    fn eval_error() {
        let e = Thunk::new(Value::from(42.0), Arguments::new(&[], &[], &[]))
            .eval()
            .wait()
            .unwrap_err();

        assert_eq!(e.name, "TypeError");
        assert_eq!(e.message, "42 is not a function");
    }

    lazy_static! {
        static ref SUM: AtomicUsize = AtomicUsize::new(0);
    }

    pure_function!(
        INCREMENT,
        Signature::new(vec![], vec![], "".into(), vec![], vec![], "".into()),
        increment
    );

    #[async(boxed_send)]
    fn increment(_: Vec<Value>) -> Result<Value> {
        loop {
            let s = SUM.load(Ordering::SeqCst);

            if SUM.compare_and_swap(s, s + 1, Ordering::SeqCst) == s {
                break;
            }
        }

        Ok(Normal::Number(SUM.load(Ordering::SeqCst) as f64).into())
    }

    #[test]
    fn run_function_only_once() {
        let p = CpuPool::new_num_cpus();
        let v = papp(INCREMENT.clone(), &[]);

        let mut fs = vec![];

        for _ in 0..1000 {
            fs.push(p.spawn(v.clone().number()));
        }

        for f in fs {
            assert_eq!(f.wait().unwrap(), 1.0);
        }
    }
}
