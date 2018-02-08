use futures::prelude::*;

use super::function::Function;
use super::result::Result;
use super::value::Value;

lazy_static! {
    pub static ref MERGE: Value = Value::from(Function::new(Default::default(), merge));
    pub static ref INSERT: Value = Value::from(Function::new(Default::default(), insert));
}

#[async(boxed_send)]
fn insert(vs: Vec<Value>) -> Result<Value> {
    unimplemented!()
}

#[async(boxed_send)]
fn merge(vs: Vec<Value>) -> Result<Value> {
    unimplemented!()
}
