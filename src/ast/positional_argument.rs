use super::expression::Expression;

#[derive(Clone, Debug, PartialEq)]
pub struct PositionalArgument {
    value: Expression,
    expanded: bool,
}

impl PositionalArgument {
    pub fn new(value: Expression, expanded: bool) -> Self {
        PositionalArgument { value, expanded }
    }
}
