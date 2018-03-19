use super::super::core::Str;

use super::expression::Expression;

#[derive(Clone, Debug, PartialEq)]
pub enum KeywordArgument {
    Unexpanded(Str, Expression),
    Expanded(Expression),
}

impl KeywordArgument {
    pub fn new(n: Str, v: Expression) -> Self {
        KeywordArgument::Unexpanded(n, v)
    }

    pub fn expanded(v: Expression) -> Self {
        KeywordArgument::Expanded(v)
    }
}
