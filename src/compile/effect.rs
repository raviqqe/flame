use super::super::core::Value;

#[derive(Clone, Debug)]
pub struct Effect {
    pub value: Value,
    pub expanded: bool,
}

impl Effect {
    pub fn new(v: Value, e: bool) -> Self {
        Effect {
            value: v,
            expanded: e,
        }
    }
}
