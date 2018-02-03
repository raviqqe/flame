use super::expression::Expression;

#[derive(Clone, Debug, PartialEq)]
pub struct PositionalArgument<'a> {
    value: Expression<'a>,
    expanded: bool,
}
