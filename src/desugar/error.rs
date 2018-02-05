use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DesugarError(String);

impl DesugarError {
    pub fn new(s: String) -> Self {
        DesugarError(s)
    }
}

impl Display for DesugarError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

impl Error for DesugarError {
    fn description(&self) -> &str {
        &self.0
    }
}
