use super::expression::Expression;

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Effect { value: Expression, expanded: bool },
}

impl Statement {
    pub fn effect(v: Expression, e: bool) -> Self {
        Statement::Effect {
            value: v,
            expanded: e,
        }
    }
}
