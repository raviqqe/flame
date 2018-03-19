use super::super::core::Str;

use super::expression::Expression;

#[derive(Clone, Debug, PartialEq)]
pub struct KeywordArgument {
    name: Str,
    value: Expression,
}

impl KeywordArgument {
    pub fn new(name: Str, value: Expression) -> Self {
        KeywordArgument { name, value }
    }
}
