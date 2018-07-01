use super::super::core::{Result, Signature, Value};

impure_function!(WRITE, Signature::default(), write);

async fn write(vs: Vec<Value>) -> Result {
    unimplemented!()
}
