use futures::prelude::*;

use super::arguments::Arguments;
use super::collection::{INSERT, MERGE};
use super::dictionary::Dictionary;
use super::error::Error;
use super::function::Function;
use super::list::List;
use super::vague_normal::VagueNormal;
use super::result::Result;
use super::thunk::Thunk;
use super::normal::Normal;

#[derive(Clone, Debug)]
pub enum Value {
    Normal(Result<VagueNormal>),
    Thunk(Thunk),
}

impl Value {
    pub fn app(f: Self, a: Arguments) -> Self {
        Value::Thunk(Thunk::new(f, a))
    }

    pub fn papp(f: Self, vs: &[Value]) -> Self {
        Value::Thunk(Thunk::new(f, Arguments::positionals(vs)))
    }

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
    pub fn list(self) -> Result<List> {
        let n = await!(self.pured())?;

        match n {
            Normal::List(l) => Ok(l),
            _ => Err(await!(Error::not_list(n))?),
        }
    }

    #[async]
    pub fn string(self) -> Result<Vec<u8>> {
        let n = await!(self.pured())?;

        match n {
            Normal::String(s) => Ok(s),
            _ => Err(await!(Error::not_string(n))?),
        }
    }

    #[async]
    pub fn to_string(self) -> Result<String> {
        await!(await!(self.pured())?.to_string())
    }

    pub fn insert(&self, k: Self, v: Self) -> Self {
        Self::papp(INSERT.clone(), &[self.clone(), k, v])
    }

    pub fn merge(&self, v: Self) -> Self {
        Self::papp(MERGE.clone(), &[self.clone(), v])
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
