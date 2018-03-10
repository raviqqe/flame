use super::string::Str;
use super::value::Value;

#[derive(Clone, Debug)]
pub struct OptionalArgument {
    pub name: Str,
    pub value: Value,
}

impl OptionalArgument {
    pub fn new(n: impl Into<Str>, v: impl Into<Value>) -> Self {
        OptionalArgument {
            name: n.into(),
            value: v.into(),
        }
    }
}
