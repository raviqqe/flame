use futures::prelude::*;

use super::normal::Normal;
use super::result::Result;

#[derive(Clone, Debug)]
pub struct Error {
    name: String,
    message: String,
    // callTrace: Vec<T>,
}

impl Error {
    pub fn new(n: &str, m: &str) -> Error {
        Error {
            name: String::from(n),
            message: String::from(m),
        }
    }

    pub fn argument(m: &str) -> Error {
        Self::new("ArgumentError", m)
    }

    pub fn value(m: &str) -> Error {
        Self::new("ValueError", m)
    }

    #[async]
    pub fn typ(n: Normal, t: String) -> Result<Error> {
        let s = await!(n.to_string())?;
        Ok(Self::new( "TypeError", &format!("{} is not a {}.", s, t)))
    }

    #[async]
    pub fn not_boolean(n: Normal) -> Result<Error> {
        await!(Self::typ(n, "boolean".to_string()))
    }

    #[async]
    pub fn not_dictionary(n: Normal) -> Result<Error> {
        await!(Self::typ(n, "dictionary".to_string()))
    }

    #[async]
    pub fn not_function(n: Normal) -> Result<Error> {
        await!(Self::typ(n, "function".to_string()))
    }

    #[async]
    pub fn not_list(n: Normal) -> Result<Error> {
        await!(Self::typ(n, "list".to_string()))
    }

    #[async]
    pub fn not_nil(n: Normal) -> Result<Error> {
        await!(Self::typ(n, "nil".to_string()))
    }

    #[async]
    pub fn not_number(n: Normal) -> Result<Error> {
        await!(Self::typ(n, "number".to_string()))
    }

    #[async]
    pub fn not_string(n: Normal) -> Result<Error> {
        await!(Self::typ(n, "string".to_string()))
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}
