use futures::prelude::*;

use super::arguments::Arguments;
use super::blur_normal::BlurNormal;
use super::error::Error;
use super::result::Result;
use super::signature::Signature;
use super::value::Value;

type RawFunction = fn(vs: Vec<Value>) -> Box<Future<Item = Value, Error = Error>>;

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
    pub fn call(self, a: Arguments) -> Result<Value> {
        Ok(await!((self.function)(await!(self.signature.bind(a))?))?)
    }
}

macro_rules! impure_function {
    ($i:ident, $f:ident, $e:expr, $r:ident) => {
        #[async(boxed)]
        fn $f(vs: Vec<Value>) -> Result<Value> {
            let n = await!(await!($r(vs))?.pured())?;
            Ok(Value::from(BlurNormal::Impure(n)))
        }

        lazy_static! {
            pub static ref $i: Function = Function::new($e, $f);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    impure_function!(TEST_FUNC, test_func_impure, Default::default(), test_func);

    #[async]
    fn test_func(vs: Vec<Value>) -> Result<Value> {
        unimplemented!()
    }
}
