use super::expression::Expression;

#[derive(Clone, Debug, PartialEq)]
pub struct OptionalParameter<'a> {
    name: &'a str,
    value: Expression<'a>,
}
