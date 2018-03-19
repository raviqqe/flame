use super::expression::Expression;

#[derive(Clone, Debug, PartialEq)]
pub enum Expansion<T> {
    Unexpanded(T),
    Expanded(Expression),
}
