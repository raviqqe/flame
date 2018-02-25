use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeError(String);

impl RuntimeError {
    pub fn new(s: String) -> Self {
        RuntimeError(s)
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

impl Error for RuntimeError {
    fn description(&self) -> &str {
        &self.0
    }
}
