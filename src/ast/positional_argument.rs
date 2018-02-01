use super::expression::Expression;

#[derive(Clone, Debug)]
pub struct PositionalArgument<'a> {
    value: Expression<'a>,
    expanded: bool,
}
