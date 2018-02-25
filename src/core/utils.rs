use super::arguments::Arguments;
use super::thunk::Thunk;
use super::value::Value;

pub fn app(f: Value, a: Arguments) -> Value {
    Value::Thunk(Thunk::new(f, a))
}

pub fn papp(f: Value, vs: &[Value]) -> Value {
    Value::Thunk(Thunk::new(f, Arguments::positionals(vs)))
}
