use super::expression::Expression;

#[derive(Clone, Debug, PartialEq)]
pub enum Statement<'a> {
    Effect {
        value: Expression<'a>,
        expanded: bool,
    },
}

impl<'a> Statement<'a> {
    pub fn effect(v: Expression<'a>, e: bool) -> Self {
        Statement::Effect {
            value: v,
            expanded: e,
        }
    }
}
