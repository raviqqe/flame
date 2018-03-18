use super::expression::Expression;

#[derive(Clone, Debug, PartialEq)]
pub struct Effect {
    value: Expression,
    expanded: bool,
}

impl Effect {
    pub fn new(value: Expression, expanded: bool) -> Self {
        Effect { value, expanded }
    }
}
