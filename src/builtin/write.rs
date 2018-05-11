use futures::prelude::*;

use super::super::core::{Result, Signature, Value};

impure_function!(WRITE, Signature::default(), write);

#[async(boxed, send)]
fn write(vs: Vec<Value>) -> Result {
    unimplemented!()
}
