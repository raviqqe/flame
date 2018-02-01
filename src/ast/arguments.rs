use super::expression::Expression;
use super::keyword_argument::KeywordArgument;
use super::positional_argument::PositionalArgument;

#[derive(Clone, Debug)]
pub struct Arguments<'a> {
    positionals: Vec<PositionalArgument<'a>>,
    keywords: Vec<KeywordArgument<'a>>,
    expanded_dicts: Vec<Expression<'a>>,
}
