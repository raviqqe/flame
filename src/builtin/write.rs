use futures::prelude::*;

use super::super::core::{Result, Signature, Value};

impure_function!(WRITE, write_impure, Signature::default(), write);

#[async]
fn write(vs: Vec<Value>) -> Result {
    unimplemented!()
}
