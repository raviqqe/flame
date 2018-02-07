use futures::prelude::*;

use super::arguments::Arguments;
use super::error::Error;
use super::result;
use super::signature::Signature;
use super::value::Value;

type RawFunction = fn(vs: Vec<Value>) -> Box<Future<Item = Value, Error = Error>>;

pub type Result = result::Result<Value>;

#[derive(Clone, Debug)]
pub struct Function {
    signature: Signature,
    function: RawFunction,
}

impl Function {
    pub fn new(s: Signature, f: RawFunction) -> Self {
        Function {
            signature: s,
            function: f,
        }
    }

    #[async(boxed)]
    pub fn call(self, a: Arguments) -> Result {
        Ok(await!((self.function)(await!(self.signature.bind(a))?))?)
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
        #[async(boxed)]
        fn $f(vs: Vec<Value>) -> ::core::Result {
            let n = await!(await!($r(vs))?.pured())?;
            Ok(::core::Value::from(::core::BlurNormal::Impure(n)))
        }

        lazy_static! {
            pub static ref $i: Value = ::core::Value::from(::core::Function::new($e, $f));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    pure_function!(TEST_FUNC, Default::default(), test_func);
    impure_function!(
        TEST_FUNC_IMPURE,
        test_func_impure,
        Default::default(),
        test_func
    );

    #[async(boxed)]
    fn test_func(vs: Vec<Value>) -> Result {
        unimplemented!()
    }
}
