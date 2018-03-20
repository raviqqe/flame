use super::expansion::Expansion;
use super::expression::Expression;
use super::keyword_argument::KeywordArgument;

#[derive(Clone, Debug, PartialEq)]
pub struct Arguments {
    pub positionals: Vec<Expansion<Expression>>,
    pub keywords: Vec<Expansion<KeywordArgument>>,
}

impl Arguments {
    pub fn new(
        positionals: Vec<Expansion<Expression>>,
        keywords: Vec<Expansion<KeywordArgument>>,
    ) -> Self {
        Arguments {
            positionals,
            keywords,
        }
    }
}
