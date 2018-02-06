use futures::prelude::*;

use super::arguments::Arguments;
use super::function::Callable;
use super::result::Result;
use super::value::Value;

lazy_static! {
    pub static ref MERGE: Value = Value::from(Box::new(Merge) as Box<Callable>);
}

#[derive(Clone, Debug)]
pub struct Merge;

impl Callable for Merge {
    #[async(boxed)]
    fn call(self, a: Arguments) -> Result<Value> {
        unimplemented!()
    }
}
