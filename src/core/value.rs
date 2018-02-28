use std::cmp::Ordering;
use futures::prelude::*;

use super::collection::{INSERT, MERGE};
use super::dictionary::Dictionary;
use super::error::Error;
use super::function::Function;
use super::list::List;
use super::normal::Normal;
use super::result::Result;
use super::string::Str;
use super::thunk::Thunk;
use super::utils::papp;
use super::vague_normal::VagueNormal;

#[derive(Clone, Debug)]
pub enum Value {
    Normal(Result<VagueNormal>),
    Thunk(Thunk),
}

impl Value {
    #[async]
    pub fn vague(self) -> Result<VagueNormal> {
        match self {
            Value::Normal(p) => p,
            Value::Thunk(t) => await!(t.eval()),
        }
    }

    #[async]
    pub fn pured(self) -> Result<Normal> {
        match await!(self.vague())? {
            VagueNormal::Pure(n) => Ok(n),
            VagueNormal::Impure(_) => Err(Error::new(
                "ImpureError",
                "impure value detected in pure context",
            )),
        }
    }

    #[async]
    pub fn impure(self) -> Result<Normal> {
        match await!(self.vague())? {
            VagueNormal::Pure(_) => Err(Error::new(
                "PureError",
                "pure value detected in impure context",
            )),
            VagueNormal::Impure(n) => Ok(n),
        }
    }

    #[async]
    pub fn boolean(self) -> Result<bool> {
        let n = await!(self.pured())?;

        match n {
            Normal::Boolean(b) => Ok(b),
            _ => Err(await!(Error::not_boolean(n))?),
        }
    }

    #[async]
    pub fn dictionary(self) -> Result<Dictionary> {
        let n = await!(self.pured())?;

        match n {
            Normal::Dictionary(d) => Ok(d),
            _ => Err(await!(Error::not_dictionary(n))?),
        }
    }

    #[async]
    pub fn function(self) -> Result<Function> {
        let n = await!(self.pured())?;

        match n {
            Normal::Function(f) => Ok(f),
            _ => Err(await!(Error::not_function(n))?),
        }
    }

    #[async]
    pub fn index(self) -> Result<usize> {
        let n = await!(self.number())?;

        if n % 1.0 == 0.0 && n >= 1.0 {
            Ok(n as usize)
        } else {
            Err(Error::value(&format!("{} is not an integer", n)))
        }
    }

    #[async]
    pub fn list(self) -> Result<List> {
        let n = await!(self.pured())?;

        match n {
            Normal::List(l) => Ok(l),
            _ => Err(await!(Error::not_list(n))?),
        }
    }

    #[async]
    pub fn number(self) -> Result<f64> {
        let n = await!(self.pured())?;

        match n {
            Normal::Number(n) => Ok(n),
            _ => Err(await!(Error::not_number(n))?),
        }
    }

    #[async]
    pub fn string(self) -> Result<Str> {
        let n = await!(self.pured())?;

        match n {
            Normal::String(s) => Ok(s),
            _ => Err(await!(Error::not_string(n))?),
        }
    }

    #[async]
    fn type_name(self) -> Result<Str> {
        Ok(await!(self.pured())?.type_name())
    }

    #[async]
    pub fn to_string(self) -> Result<String> {
        await!(await!(self.pured())?.to_string())
    }

    #[async(boxed_send)]
    pub fn equal(self, v: Self) -> Result<bool> {
        let m = await!(self.pured())?;
        let n = await!(v.pured())?;
        await!(m.equal(n))
    }

    #[async(boxed_send)]
    pub fn compare(self, v: Self) -> Result<Ordering> {
        let m = await!(self.pured())?;
        let n = await!(v.pured())?;
        await!(m.compare(n))
    }

    pub fn insert(&self, k: impl Into<Self>, v: impl Into<Self>) -> Self {
        papp(INSERT.clone(), &[self.clone(), k.into(), v.into()])
    }

    pub fn merge(&self, v: Self) -> Self {
        papp(MERGE.clone(), &[self.clone(), v])
    }
}

impl<T: Into<Normal>> From<T> for Value {
    fn from(x: T) -> Self {
        Value::from(VagueNormal::from(x.into()))
    }
}

impl From<VagueNormal> for Value {
    fn from(b: VagueNormal) -> Self {
        Value::Normal(Ok(b))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_f64() {
        Value::from(123.0);
    }
}
