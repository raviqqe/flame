use futures::prelude::*;

use super::arguments::Arguments;
use super::result::Result;
use super::signature::Signature;
use super::thunk::Thunk;
use super::value::Value;

pub fn app(f: Value, a: Arguments) -> Value {
    Value::Thunk(Thunk::new(f, a))
}

pub fn papp(f: Value, vs: &[Value]) -> Value {
    Value::Thunk(Thunk::new(f, Arguments::positionals(vs)))
}

pure_function!(
    IDENTITY,
    Signature::new(vec!["x".into()], "".into(), vec![], "".into()),
    identity
);

#[async(boxed, send)]
fn identity(vs: Vec<Value>) -> Result<Value> {
    Ok(vs[0].clone())
}

pure_function!(
    TEST_FUNCTION,
    Signature::new(vec![], "elemsAndList".into(), vec![], "".into()),
    test_function
);

#[async(boxed, send)]
fn test_function(_: Vec<Value>) -> Result<Value> {
    Ok(Value::Nil)
}
