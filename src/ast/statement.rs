use super::effect::Effect;

#[derive(Clone, Debug)]
pub enum Statement<'a> {
    Effect(Effect<'a>),
}
