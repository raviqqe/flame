use std::boxed::PinBox;
use std::sync::Arc;

use futures::prelude::*;

use super::arguments::Arguments;
use super::error::Error;
use super::result;
use super::signature::Signature;
use super::unsafe_ref::{Ref, RefMut};
use super::utils::app;
use super::value::Value;

type ResultFuture = PinBox<Future<Item = Value, Error = Error> + Send>;
type SubFunction = fn(vs: Vec<Value>) -> ResultFuture;

pub type Result = result::Result<Value>;

#[derive(Clone, Debug)]
pub enum Function {
    Closure(Arc<(Value, Arguments)>),
    Builtin(Arc<(Signature, SubFunction, bool)>),
}

impl Function {
    pub fn new(s: Signature, f: SubFunction, p: bool) -> Self {
        Function::Builtin(Arc::new((s, f, p)))
    }

    pub fn closure(f: Value, a: Arguments) -> Self {
        Function::Closure(Arc::new((f, a)))
    }

    pub fn is_pure(&self) -> bool {
        match *self {
            Function::Closure(_) => true,
            Function::Builtin(ref r) => r.2,
        }
    }

    pub fn call(self, a: RefMut<Arguments>) -> Result {
        Ok(match self {
            Function::Closure(r) => {
                let (f, vs) = (*r).clone();
                app(f, vs.merge(&a))
            }
            Function::Builtin(r) => await!(r.1(await!(Ref(&r.0).bind(a))?))?,
        })
    }
}

macro_rules! pure_function {
    ($i:ident, $e:expr, $f:ident) => {
        lazy_static! {
            pub static ref $i: Value = ::core::Value::from(::core::Function::new($e, $f, true));
        }
    };
}

macro_rules! impure_function {
    ($i:ident, $e:expr, $f:ident) => {
        lazy_static! {
            pub static ref $i: Value = ::core::Value::from(::core::Function::new($e, $f, false));
        }
    };
}

#[cfg(test)]
mod test {
    use std::mem::size_of;

    use futures::stable::block_on_stable;
    use test::Bencher;

    use super::*;

    use super::super::utils::{papp, IDENTITY};

    pure_function!(TEST_FUNC, Default::default(), test_func);
    impure_function!(TEST_FUNC_IMPURE, Default::default(), test_func);

    async fn test_func(vs: Vec<Value>) -> Result {
        Ok(Value::from(42.0))
    }

    #[test]
    fn closure() {
        let f = Function::closure(IDENTITY.clone(), Arguments::positionals(&[42.into()]));

        assert_eq!(block_on_stable(papp(f.into(), &[]).number()).unwrap(), 42.0);
    }

    #[test]
    fn pure_function_call() {
        block_on_stable(papp(TEST_FUNC.clone(), &[]).pured()).unwrap();
        block_on_stable(papp(IDENTITY.clone(), &[papp(TEST_FUNC.clone(), &[])]).pured()).unwrap();
    }

    #[test]
    fn impure_function_call() {
        block_on_stable(papp(TEST_FUNC_IMPURE.clone(), &[]).impure()).unwrap();
        block_on_stable(papp(IDENTITY.clone(), &[papp(TEST_FUNC_IMPURE.clone(), &[])]).impure())
            .unwrap();
    }

    #[test]
    fn size() {
        let s = size_of::<Function>();
        assert!(s <= 2 * size_of::<usize>(), "size of Function: {}", s);
    }

    #[bench]
    fn function_call(b: &mut Bencher) {
        b.iter(|| {
            block_on_stable(papp(IDENTITY.clone(), &[1000.into()]).pured()).unwrap();
        });
    }
}
