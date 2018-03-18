use super::super::core::Str;

use super::expression::Expression;

#[derive(Clone, Debug, PartialEq)]
pub struct LetVariable {
    name: Str,
    value: Expression,
}

impl LetVariable {
    pub fn new(name: Str, value: Expression) -> Self {
        LetVariable { name, value }
    }
}
