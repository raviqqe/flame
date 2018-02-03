use super::super::core::Value;

#[derive(Clone, Debug)]
pub struct Effect {
    value: Value,
    expanded: bool,
}
