use super::normal::Normal;

#[derive(Clone, Debug)]
pub enum VagueNormal {
    Pure(Normal),
    Impure(Normal),
}

impl From<Normal> for VagueNormal {
    fn from(n: Normal) -> Self {
        VagueNormal::Pure(n)
    }
}
