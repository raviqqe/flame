use super::expression::Expression;

#[derive(Clone, Debug)]
pub struct OptionalParameter<'a> {
    name: &'a str,
    value: Expression<'a>,
}
