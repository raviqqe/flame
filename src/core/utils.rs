use futures::prelude::*;

use super::arguments::Arguments;
use super::normal::Normal;
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
    TEST_FUNCTION,
    Signature::new(
        vec![],
        vec![],
        "elemsAndList".into(),
        vec![],
        vec![],
        "".into()
    ),
    test_function
);

#[async(boxed_send)]
fn test_function(_: Vec<Value>) -> Result<Value> {
    Ok(Normal::Nil.into())
}
