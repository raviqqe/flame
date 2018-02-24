use super::expression::Expression;

#[derive(Clone, Debug, PartialEq)]
pub struct OptionalParameter {
    name: String,
    value: Expression,
}
