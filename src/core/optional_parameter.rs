use super::string::Str;
use super::value::Value;

#[derive(Clone, Debug)]
pub struct OptionalParameter {
    pub name: Str,
    pub value: Value,
}

impl OptionalParameter {
    pub fn new(n: impl Into<Str>, v: impl Into<Value>) -> Self {
        OptionalParameter {
            name: n.into(),
            value: v.into(),
        }
    }
}
