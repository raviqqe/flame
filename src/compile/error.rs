use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompileError(String);

impl CompileError {
    pub fn new(s: String) -> Self {
        CompileError(s)
    }
}

impl Display for CompileError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

impl Error for CompileError {
    fn description(&self) -> &str {
        &self.0
    }
}
