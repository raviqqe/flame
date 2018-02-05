use super::normal::Normal;

#[derive(Clone, Debug)]
pub enum BlurNormal {
    Pure(Normal),
    Impure(Normal),
}
