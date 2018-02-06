use futures::prelude::*;

use super::arguments::Arguments;
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
