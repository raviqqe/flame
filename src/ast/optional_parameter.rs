use super::expression::Expression;

use super::super::core::Str;

#[derive(Clone, Debug, PartialEq)]
pub struct OptionalParameter {
    name: Str,
    value: Expression,
}

impl OptionalParameter {
    pub fn new(name: Str, value: Expression) -> Self {
        OptionalParameter { name, value }
    }
}
