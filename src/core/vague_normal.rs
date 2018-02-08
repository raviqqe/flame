use super::normal::Normal;

#[derive(Clone, Debug)]
pub enum VagueNormal {
    Pure(Normal),
    Impure(Normal),
}
