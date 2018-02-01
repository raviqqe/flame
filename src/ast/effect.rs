use super::expression::Expression;

#[derive(Clone, Debug)]
pub struct Effect<'a> {
    value: Expression<'a>,
    expanded: bool,
}

impl<'a> Effect<'a> {
    pub fn new(v: Expression<'a>, e: bool) -> Self {
        Effect {
            value: v,
            expanded: e,
        }
    }
}
