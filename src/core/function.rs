use std::sync::Arc;

use futures::prelude::*;
use pin_api::Unpin;

use super::arguments::Arguments;
use super::error::Error;
use super::result;
use super::signature::Signature;
use super::unsafe_ref::{Ref, RefMut};
use super::utils::app;
use super::value::Value;

type ResultFuture = Box<Future<Item = Value, Error = Error> + Send + Unpin>;
type SubFunction = fn(vs: Vec<Value>) -> ResultFuture;

pub type Result = result::Result<Value>;

#[derive(Clone, Debug)]
pub enum Function {
    Closure(Arc<(Value, Arguments)>),
    Signatured(Arc<(Signature, SubFunction)>),
}

impl Function {
    pub fn new(s: Signature, f: SubFunction) -> Self {
        Function::Signatured(Arc::new((s, f)))
    }

    pub fn closure(f: Value, a: Arguments) -> Self {
        Function::Closure(Arc::new((f, a)))
    }

    #[async_move]
    pub fn call(self, a: RefMut<Arguments>) -> Result {
        Ok(match self {
            Function::Closure(r) => {
                let (f, vs) = (*r).clone();
                app(f, vs.merge(&a))
            }
            Function::Signatured(r) => await!(r.1(await!(Signature::bind(Ref(&r.0), a))?))?,
        })
    }
}

macro_rules! pure_function {
    ($i:ident,  $e:expr, $f:ident) => {
        lazy_static! {
            pub static ref $i: Value = ::core::Value::from(::core::Function::new($e, $f));
        }
    }
}

macro_rules! impure_function {
    ($i:ident, $f:ident, $e:expr, $r:ident) => {
        #[async_move(boxed_send)]
        fn $f(vs: Vec<Value>) -> ::core::Result {
            let n = await!(await!($r(vs))?.pured())?;
            Ok(::core::Value::from(::core::VagueNormal::Impure(n)))
        }

        lazy_static! {
            pub static ref $i: Value = ::core::Value::from(::core::Function::new($e, $f));
        }
    }
}

#[cfg(test)]
mod test {
    use std::mem::size_of;

    use futures::executor::block_on;
    use test::Bencher;

    use super::*;

    use super::super::utils::{papp, IDENTITY};

    pure_function!(TEST_FUNC, Default::default(), test_func);
    impure_function!(
        TEST_FUNC_IMPURE,
        test_func_impure,
        Default::default(),
        test_func
    );

    #[async_move(boxed_send)]
    fn test_func(vs: Vec<Value>) -> Result {
        Ok(Value::from(42.0))
    }

    #[test]
    fn closure() {
        let f = Function::closure(IDENTITY.clone(), Arguments::positionals(&[42.into()]));

        assert_eq!(block_on(papp(f.into(), &[]).number()).unwrap(), 42.0);
    }

    #[test]
    fn size() {
        let s = size_of::<Function>();
        assert!(s <= 2 * size_of::<usize>(), "size of Function: {}", s);
    }

    #[bench]
    fn function_call(b: &mut Bencher) {
        b.iter(|| {
            block_on(papp(IDENTITY.clone(), &[1000.into()]).pured()).unwrap();
        });
    }
}
