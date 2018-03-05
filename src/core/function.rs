use std::sync::Arc;

use futures::prelude::*;

use super::arguments::Arguments;
use super::error::Error;
use super::result;
use super::signature::Signature;
use super::unsafe_ref::Ref;
use super::utils::app;
use super::value::Value;

type ResultFuture = Box<Future<Item = Value, Error = Error> + Send>;
type SubFunction = fn(vs: Vec<Value>) -> ResultFuture;

pub type Result = result::Result<Value>;

#[derive(Clone, Debug)]
pub enum Function {
    Closure(Arc<(Value, Arguments)>),
    Raw(fn(Arguments) -> ResultFuture),
    Signatured(Arc<(Signature, SubFunction)>),
}

impl Function {
    pub fn new(s: Signature, f: SubFunction) -> Self {
        Function::Signatured(Arc::new((s, f)))
    }

    pub fn raw(f: fn(Arguments) -> ResultFuture) -> Self {
        Function::Raw(f)
    }

    pub fn closure(f: Value, a: Arguments) -> Self {
        Function::Closure(Arc::new((f, a)))
    }

    #[async(boxed_send)]
    pub fn call(self, a: Arguments) -> Result {
        Ok(match self {
            Function::Closure(r) => {
                let (f, vs) = (*r).clone();
                app(f, vs.merge(&a))
            }
            Function::Raw(f) => await!(f(a))?,
            Function::Signatured(r) => await!(r.1(await!(Signature::bind(Ref(&r.0), a))?))?,
        })
    }
}

macro_rules! raw_function {
    ($i:ident, $f:ident) => {
        lazy_static! {
            pub static ref $i: Value = ::core::Function::raw($f).into();
        }
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

    use super::*;

    use super::super::utils::papp;

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

    #[async(boxed_send)]
    fn test_raw_function(mut a: Arguments) -> Result {
        Ok(a.next_positional().unwrap())
    }

    #[test]
    fn raw() {
        let f = Function::raw(test_raw_function);

        assert!(
            papp(f.into(), &[42.into()])
                .equal(42.into())
                .wait()
                .unwrap()
        );
    }

    pure_function!(
        TEST_IDENTITY_FUNC,
        Signature::new(
            vec!["x".into()],
            vec![],
            "".into(),
            vec![],
            vec![],
            "".into()
        ),
        test_identity_func
    );

    #[async(boxed_send)]
    fn test_identity_func(vs: Vec<Value>) -> Result {
        Ok(vs[0].clone())
    }

    #[test]
    fn closure() {
        let f = Function::closure(
            TEST_IDENTITY_FUNC.clone(),
            Arguments::positionals(&[42.into()]),
        );

        assert_eq!(papp(f.into(), &[]).number().wait().unwrap(), 42.0);
    }

    #[test]
    fn size() {
        let s = size_of::<Function>();
        assert!(s <= 2 * size_of::<usize>(), "size of Function: {}", s);
    }
}
