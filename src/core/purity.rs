use super::normal::Normal;

#[derive(Clone, Debug)]
pub enum Purity {
    Pure(Normal),
    Impure(Normal),
}
