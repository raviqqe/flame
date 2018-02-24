use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result};

use pest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsingError(String);

impl Display for ParsingError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ParsingError {
    fn description(&self) -> &str {
        &self.0
    }
}

impl<'a, T: Debug> From<pest::Error<'a, T>> for ParsingError {
    fn from(e: pest::Error<T>) -> Self {
        ParsingError(format!("{}", e))
    }
}
