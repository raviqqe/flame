use super::keyword_argument::KeywordArgument;
use super::positional_argument::PositionalArgument;

#[derive(Clone, Debug, PartialEq)]
pub struct Arguments {
    positionals: Vec<PositionalArgument>,
    keywords: Vec<KeywordArgument>,
}

impl Arguments {
    pub fn new(positionals: Vec<PositionalArgument>, keywords: Vec<KeywordArgument>) -> Self {
        Arguments {
            positionals,
            keywords,
        }
    }
}
