use std::sync::Arc;

use futures::prelude::*;

use super::arguments::Arguments;
use super::error::Error;
use super::result;
use super::signature::Signature;
use super::unsafe_ref::{Ref, RefMut};
use super::utils::app;
use super::value::Value;

type ResultFuture = Box<Future<Item = Value, Error = Error> + Send>;
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

    #[async(boxed_send)]
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
        #[async(boxed_send)]
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

    use test::Bencher;

    use super::*;

    use super::super::normal::Normal;
    use super::super::utils::{papp, IDENTITY};

    pure_function!(TEST_FUNC, Default::default(), test_func);
    impure_function!(
        TEST_FUNC_IMPURE,
        test_func_impure,
        Default::default(),
        test_func
    );

    #[async(boxed_send)]
    fn test_func(vs: Vec<Value>) -> Result {
        Ok(Value::from(42.0))
    }

    #[test]
    fn closure() {
        let f = Function::closure(IDENTITY.clone(), Arguments::positionals(&[42.into()]));

        assert_eq!(papp(f.into(), &[]).number().wait().unwrap(), 42.0);
    }

    #[test]
    fn size() {
        let s = size_of::<Function>();
        assert!(s <= 2 * size_of::<usize>(), "size of Function: {}", s);
    }

    #[bench]
    fn function_call(b: &mut Bencher) {
        b.iter(|| {
            papp(IDENTITY.clone(), &[1000.into()])
                .pured()
                .wait()
                .unwrap();
        });
    }

    fn normal_function() -> Result {
        Ok(Normal::Nil.into())
    }

    #[bench]
    fn bench_normal_function(b: &mut Bencher) {
        b.iter(|| normal_function().unwrap());
    }

    #[async]
    fn async_function() -> Result {
        Ok(Normal::Nil.into())
    }

    #[bench]
    fn bench_async_function(b: &mut Bencher) {
        b.iter(|| async_function().wait().unwrap());
    }

    #[async(boxed_send)]
    fn boxed_async_function() -> Result {
        Ok(Normal::Nil.into())
    }

    #[bench]
    fn bench_boxed_async_function(b: &mut Bencher) {
        b.iter(|| boxed_async_function().wait().unwrap());
    }
}
