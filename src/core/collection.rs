use futures::prelude::*;

use super::function::Function;
use super::result::Result;
use super::value::Value;

lazy_static! {
    pub static ref MERGE: Value = Value::from(Function::new(Default::default(), merge));
}

#[async(boxed)]
fn merge(vs: Vec<Value>) -> Result<Value> {
    unimplemented!()
}
