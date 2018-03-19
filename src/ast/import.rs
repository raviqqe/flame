#[derive(Clone, Debug, PartialEq)]
pub struct Import(String);

impl Import {
    pub fn new(s: String) -> Self {
        Import(s)
    }
}
