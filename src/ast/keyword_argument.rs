use super::expression::Expression;

#[derive(Clone, Debug, PartialEq)]
pub struct KeywordArgument<'a> {
    name: &'a str,
    value: Expression<'a>,
}
