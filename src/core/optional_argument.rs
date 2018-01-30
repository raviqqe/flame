use super::value::Value;

#[derive(Clone, Debug)]
pub struct OptionalArgument {
    pub name: String,
    pub value: Value,
}

impl OptionalArgument {
    pub fn new(n: String, v: Value) -> Self {
        OptionalArgument { name: n, value: v }
    }
}
