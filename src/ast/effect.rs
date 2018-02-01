use super::expression::Expression;

#[derive(Clone, Debug)]
pub struct Effect<'a> {
    value: Expression<'a>,
    expanded: bool,
}
