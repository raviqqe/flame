use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result};

use pest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsingError(String);

impl Display for ParsingError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "ParsingError: {}", self.0)
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fmt() {
        assert_eq!(
            "ParsingError: foo",
            format!("{}", ParsingError("foo".into()))
        );
    }

    #[test]
    fn description() {
        assert_eq!(
            "foo",
            format!("{}", ParsingError("foo".into()).description())
        );
    }
}
