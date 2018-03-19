use super::expression::Expression;
use super::keyword_argument::KeywordArgument;
use super::positional_argument::PositionalArgument;

#[derive(Clone, Debug, PartialEq)]
pub struct Arguments {
    positionals: Vec<PositionalArgument>,
    keywords: Vec<KeywordArgument>,
    expanded_dicts: Vec<Expression>,
}

impl Arguments {
    pub fn new(
        positionals: Vec<PositionalArgument>,
        keywords: Vec<KeywordArgument>,
        expanded_dicts: Vec<Expression>,
    ) -> Self {
        Arguments {
            positionals,
            keywords,
            expanded_dicts,
        }
    }
}
