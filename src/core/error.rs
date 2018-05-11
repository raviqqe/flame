use std::error;
use std::sync::Arc;

use futures::prelude::*;

use super::result::Result;
use super::value::Value;

#[derive(Clone, Debug)]
pub struct Error(Arc<Inner>);

#[derive(Clone, Debug)]
pub struct Inner {
    pub name: String,
    pub message: String,
    // callTrace: Vec<T>,
}

impl Error {
    pub fn new(n: &str, m: &str) -> Self {
        Error(Arc::new(Inner {
            name: n.into(),
            message: m.into(),
        }))
    }

    pub fn name(&self) -> &str {
        &self.0.name
    }

    pub fn message(&self) -> &str {
        &self.0.message
    }

    pub fn argument(m: &str) -> Self {
        Self::new("ArgumentError", m)
    }

    pub fn runtime(m: &str) -> Self {
        Self::new("RuntimeError", m)
    }

    pub fn unreachable() -> Self {
        Self::new("UnreachableError", "unreachable code")
    }

    pub fn pured() -> Self {
        Self::new("PureError", "pure value detected in impure context")
    }

    pub fn impure() -> Self {
        Self::new("ImpureError", "impure value detected in pure context")
    }

    pub fn value(m: &str) -> Self {
        Self::new("ValueError", m)
    }

    pub fn empty_list() -> Self {
        Self::value("list is empty")
    }

    #[async]
    pub fn key_not_found(v: Value) -> Result<Self> {
        let s = await!(v.to_string())?;

        Ok(Self::new(
            "KeyNotFoundError",
            &format!("key, {} is not found key in dictionary", s),
        ))
    }

    pub fn typ_raw(s: &str) -> Self {
        Self::new("TypeError", s)
    }

    #[async]
    pub fn typ(v: Value, t: String) -> Result<Self> {
        let s = await!(v.to_string())?;
        Ok(Self::typ_raw(&format!("{} is not a {}", s, t)))
    }

    #[async]
    pub fn not_boolean(v: Value) -> Result<Self> {
        await!(Self::typ(v, "boolean".to_string()))
    }

    #[async]
    pub fn not_comparable(v: Value, w: Value) -> Result<Self> {
        let s = await!(v.to_string())?;
        let t = await!(w.to_string())?;
        Ok(Self::typ_raw(&format!("{} and {} is not comparable", s, t)))
    }

    #[async]
    pub fn not_equalable(v: Value) -> Result<Self> {
        Ok(Self::typ_raw(&format!(
            "{} is not equalable",
            await!(v.to_string())?
        )))
    }

    #[async]
    pub fn not_collection(v: Value) -> Result<Self> {
        await!(Self::typ(v, "collection".to_string()))
    }

    #[async]
    pub fn not_dictionary(v: Value) -> Result<Self> {
        await!(Self::typ(v, "dictionary".to_string()))
    }

    #[async]
    pub fn not_function(v: Value) -> Result<Self> {
        await!(Self::typ(v, "function".to_string()))
    }

    #[async]
    pub fn not_list(v: Value) -> Result<Self> {
        await!(Self::typ(v, "list".to_string()))
    }

    #[async]
    pub fn not_nil(v: Value) -> Result<Self> {
        await!(Self::typ(v, "nil".to_string()))
    }

    #[async]
    pub fn not_number(v: Value) -> Result<Self> {
        await!(Self::typ(v, "number".to_string()))
    }

    #[async]
    pub fn not_string(v: Value) -> Result<Self> {
        await!(Self::typ(v, "string".to_string()))
    }
}

impl<E: error::Error> From<E> for Error {
    fn from(e: E) -> Self {
        Self::runtime(e.description())
    }
}
