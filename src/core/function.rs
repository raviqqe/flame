use futures::prelude::*;

use super::arguments::Arguments;
use super::error::Error;
use super::result;
use super::signature::Signature;
use super::value::Value;

type ResultFuture = Box<Future<Item = Value, Error = Error> + Send>;
type SubFunction = fn(vs: Vec<Value>) -> ResultFuture;

pub type Result = result::Result<Value>;

#[derive(Clone, Debug)]
pub enum Function {
    Raw(fn(Arguments) -> ResultFuture),
    Signatured(Signature, SubFunction),
}

impl Function {
    pub fn new(s: Signature, f: SubFunction) -> Self {
        Function::Signatured(s, f)
    }

    pub fn raw(f: fn(Arguments) -> ResultFuture) -> Self {
        Function::Raw(f)
    }

    #[async(boxed_send)]
    pub fn call(self, a: Arguments) -> Result {
        match self {
            Function::Raw(f) => Ok(await!(f(a))?),
            Function::Signatured(s, f) => Ok(await!(f(await!(s.bind(a))?))?),
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
}
