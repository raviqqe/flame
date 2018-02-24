use super::expression::Expression;

#[derive(Clone, Debug, PartialEq)]
pub struct PositionalArgument {
    value: Expression,
    expanded: bool,
}
