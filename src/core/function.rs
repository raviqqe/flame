use futures::prelude::*;

use super::arguments::Arguments;
use super::error::Error;
use super::result::Result;
use super::signature::Signature;
use super::value::Value;

pub type Function = Box<Callable>;

pub type FunctionResult = Box<Future<Item = Value, Error = Error>>;

pub trait Callable: Send + Sync {
    fn call(self, a: Arguments) -> FunctionResult;
}

#[derive(Clone, Debug)]
pub struct PureFunction {
    signature: Signature,
    function: fn(vs: Vec<Value>) -> FunctionResult,
}

impl Callable for PureFunction {
    #[async(boxed)]
    fn call(self, a: Arguments) -> Result<Value> {
        Ok(await!((self.function)(await!(self.signature.bind(a))?))?)
    }
}
