use std::cell::UnsafeCell;
use std::convert::TryInto;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::Arc;

use super::black_hole::BlackHole;
use futures::prelude::*;

use core::arguments::Arguments;
use core::error::Error;
use core::normal::Normal;
use core::result::Result;
use core::unsafe_ref::{Ref, RefMut};
use core::utils::IDENTITY;
use core::value::Value;

#[derive(Clone, Debug)]
pub struct Thunk(Arc<UnsafeCell<Inner>>);

impl Thunk {
    pub fn new(f: Value, a: Arguments) -> Self {
        Thunk(Arc::new(UnsafeCell::new(Inner::new(f, a))))
    }

    #[async]
    pub fn eval_pure(self) -> Result<Normal> {
        let n = await!(self.eval())?;

        match n {
            VagueNormal::Pure(n) => Ok(n),
            _ => Err(Error::impure()),
        }
    }

    #[async]
    pub fn eval_impure(self) -> Result<Normal> {
        let n = await!(self.eval())?;

        match n {
            VagueNormal::Impure(n) => Ok(n),
            _ => Err(Error::pured()),
        }
    }

    #[async(boxed, send)]
    fn eval(self) -> Result<VagueNormal> {
        if self.inner_mut().lock(State::Normal) {
            let mut purity = true;

            self.inner_mut().content = Content::Normal(loop {
                let (f, a) = match self.inner_mut().content {
                    Content::App(ref f, ref mut a) => (f.clone(), RefMut(a)),
                    Content::Normal(_) => unreachable!(),
                };

                let f = match await!(f.function()) {
                    Err(e) => break Err(e),
                    Ok(f) => f,
                };

                if !f.is_pure() && purity {
                    purity = false;
                } else if !f.is_pure() {
                    break Err(Error::impure());
                }

                match await!(f.call(a)) {
                    Err(e) => break Err(e),
                    Ok(Value::Thunk(t)) => if !t.delegate_evaluation(&self) {
                        break match await!(t.eval()) {
                            Err(e) => Err(e),
                            Ok(VagueNormal::Pure(n)) => Ok(if purity {
                                VagueNormal::Pure
                            } else {
                                VagueNormal::Impure
                            }(n)),
                            Ok(VagueNormal::Impure(n)) => if purity {
                                Ok(VagueNormal::Impure(n))
                            } else {
                                Err(Error::impure())
                            },
                        };
                    },
                    Ok(v) => {
                        break Ok(if purity {
                            VagueNormal::Pure
                        } else {
                            VagueNormal::Impure
                        }(v.try_into().unwrap()))
                    }
                }
            });

            self.inner().black_hole.release()?;
        } else {
            await!(Ref(&self.inner().black_hole))?;
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

    fn delegate_evaluation(&self, t: &Thunk) -> bool {
        if !self.inner_mut().lock(State::SpinLock) {
            return false;
        }

        let (f, a) = match self.inner_mut().content {
            Content::App(ref mut f, ref mut a) => (f, a),
            Content::Normal(_) => unreachable!(),
        };

        t.inner_mut().content = Content::App(f.clone(), a.clone());

        *f = IDENTITY.clone();
        *a = Arguments::positionals(&[t.clone().into()]);

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

#[derive(Clone, Debug)]
pub enum VagueNormal {
    Pure(Normal),
    Impure(Normal),
}

#[cfg(test)]
mod test {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use futures::stable::block_on_stable;
    use test::Bencher;

    use core::signature::Signature;
    use core::utils::{papp, IDENTITY};

    use super::*;

    #[test]
    fn new() {
        Thunk::new(Value::from(0.0), Arguments::new(&[], &[]));
    }

    #[test]
    fn eval_error() {
        let e =
            block_on_stable(Thunk::new(42.into(), Arguments::new(&[], &[])).eval()).unwrap_err();

        assert_eq!(e.name(), "TypeError");
        assert_eq!(e.message(), "42 is not a function");
    }

    lazy_static! {
        static ref SUM: AtomicUsize = AtomicUsize::new(0);
    }

    pure_function!(
        INCREMENT,
        Signature::new(vec![], "".into(), vec![], "".into()),
        increment
    );

    #[async(boxed, send)]
    fn increment(_: Vec<Value>) -> Result<Value> {
        loop {
            let s = SUM.load(Ordering::SeqCst);

            if SUM.compare_and_swap(s, s + 1, Ordering::SeqCst) == s {
                break;
            }
        }

        Ok(Value::Number(SUM.load(Ordering::SeqCst) as f64).into())
    }

    #[test]
    fn run_function_only_once() {
        let v = papp(INCREMENT.clone(), &[]);

        for _ in 0..1000 {
            assert_eq!(block_on_stable(v.clone().number()).unwrap(), 1.0);
        }
    }

    #[bench]
    fn bench_thunk_new(b: &mut Bencher) {
        b.iter(|| Thunk::new(IDENTITY.clone(), Arguments::positionals(&[1000.into()])));
    }

    #[bench]
    fn bench_thunk_eval(b: &mut Bencher) {
        b.iter(|| {
            block_on_stable(
                Thunk::new(IDENTITY.clone(), Arguments::positionals(&[1000.into()])).eval(),
            ).unwrap()
        });
    }
}
