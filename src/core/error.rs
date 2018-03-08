use std::error;

use futures::prelude::*;

use super::normal::Normal;
use super::result::Result;
use super::value::Value;

#[derive(Clone, Debug)]
pub struct Error {
    pub name: String,
    pub message: String,
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

    pub fn runtime(m: &str) -> Error {
        Self::new("RuntimeError", m)
    }

    pub fn value(m: &str) -> Error {
        Self::new("ValueError", m)
    }

    pub fn empty_list() -> Error {
        Self::value("list is empty")
    }

    #[async_move]
    pub fn key_not_found(v: Value) -> Result<Error> {
        let s = await!(v.to_string())?;

        Ok(Self::new(
            "KeyNotFoundError",
            &format!("key, {} is not found key in dictionary", s),
        ))
    }

    pub fn typ_raw(s: &str) -> Self {
        Self::new("TypeError", s)
    }

    #[async_move]
    pub fn typ(n: Normal, t: String) -> Result<Error> {
        let s = await!(n.to_string())?;
        Ok(Self::typ_raw(&format!("{} is not a {}", s, t)))
    }

    #[async_move]
    pub fn not_boolean(n: Normal) -> Result<Error> {
        await!(Self::typ(n, "boolean".to_string()))
    }

    #[async_move]
    pub fn not_comparable(m: Normal, n: Normal) -> Result<Self> {
        let s = await!(m.to_string())?;
        let t = await!(n.to_string())?;
        Ok(Self::typ_raw(&format!("{} and {} is not comparable", s, t)))
    }

    #[async_move]
    pub fn not_equalable(n: Normal) -> Result<Self> {
        Ok(Self::typ_raw(&format!(
            "{} is not equalable",
            await!(n.to_string())?
        )))
    }

    #[async_move]
    pub fn not_collection(n: Normal) -> Result<Error> {
        await!(Self::typ(n, "collection".to_string()))
    }

    #[async_move]
    pub fn not_dictionary(n: Normal) -> Result<Error> {
        await!(Self::typ(n, "dictionary".to_string()))
    }

    #[async_move]
    pub fn not_function(n: Normal) -> Result<Error> {
        await!(Self::typ(n, "function".to_string()))
    }

    #[async_move]
    pub fn not_list(n: Normal) -> Result<Error> {
        await!(Self::typ(n, "list".to_string()))
    }

    #[async_move]
    pub fn not_nil(n: Normal) -> Result<Error> {
        await!(Self::typ(n, "nil".to_string()))
    }

    #[async_move]
    pub fn not_number(n: Normal) -> Result<Error> {
        await!(Self::typ(n, "number".to_string()))
    }

    #[async_move]
    pub fn not_string(n: Normal) -> Result<Error> {
        await!(Self::typ(n, "string".to_string()))
    }
}

impl<E: error::Error> From<E> for Error {
    fn from(e: E) -> Self {
        Error::runtime(e.description())
    }
}
