use super::expression::Expression;

#[derive(Clone, Debug, PartialEq)]
pub struct KeywordArgument {
    name: String,
    value: Expression,
}
