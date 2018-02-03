use std::fmt;
use std::fmt::{Display, Formatter};

use futures::prelude::*;

use super::dictionary::Dictionary;
use super::error::Error;
use super::list::List;
use super::result::Result;
use super::thunk;
use super::normal::Normal;

#[derive(Clone, Debug)]
pub enum Value {
    Invalid,
    Normal(Normal),
    Thunk(thunk::Thunk),
}

impl Value {
    #[async]
    pub fn normal(self) -> Result<Normal> {
        Ok(match self {
            Value::Invalid => unreachable!(),
            Value::Normal(n) => n,
            Value::Thunk(t) => await!(t.eval())?,
        })
    }

    #[async]
    pub fn boolean(self) -> Result<bool> {
        let n = await!(self.normal())?;

        match n {
            Normal::Boolean(b) => Ok(b),
            _ => Err(await!(Error::not_boolean(n))?),
        }
    }

    #[async]
    pub fn dictionary(self) -> Result<Dictionary> {
        let n = await!(self.normal())?;

        match n {
            Normal::Dictionary(d) => Ok(d),
            _ => Err(await!(Error::not_dictionary(n))?),
        }
    }

    #[async]
    pub fn list(self) -> Result<List> {
        let n = await!(self.normal())?;

        match n {
            Normal::List(l) => Ok(l),
            _ => Err(await!(Error::not_list(n))?),
        }
    }

    #[async]
    pub fn to_string(self) -> Result<String> {
        await!(await!(self.normal())?.to_string())
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Invalid
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Value::from(Normal::from(n))
    }
}

impl From<List> for Value {
    fn from(l: List) -> Self {
        Value::from(Normal::from(l))
    }
}

impl From<Normal> for Value {
    fn from(n: Normal) -> Self {
        Value::Normal(n)
    }
}

impl Future for Value {
    type Item = Normal;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match *self {
            Value::Invalid => unreachable!(),
            Value::Normal(ref n) => Ok(Async::Ready(n.clone())),
            Value::Thunk(ref mut t) => t.poll(),
        }
    }
}
