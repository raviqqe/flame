use super::expression::Expression;

#[derive(Clone, Debug)]
pub struct KeywordArgument<'a> {
    name: &'a str,
    value: Expression<'a>,
}
