use futures::prelude::*;

use super::arguments::{Arguments, PositionalArgument};
use super::collection::MERGE;
use super::dictionary::Dictionary;
use super::error::Error;
use super::list::List;
use super::blur_normal::BlurNormal;
use super::result::Result;
use super::thunk::Thunk;
use super::normal::Normal;

#[derive(Clone, Debug)]
pub enum Value {
    Normal(Result<BlurNormal>),
    Thunk(Thunk),
}

impl Value {
    fn app(f: Self, a: Arguments) -> Self {
        Value::Thunk(Thunk::new(f, a))
    }

    #[async]
    pub fn blur(self) -> Result<BlurNormal> {
        match self {
            Value::Normal(p) => p,
            Value::Thunk(t) => await!(t.eval()),
        }
    }

    #[async]
    pub fn pured(self) -> Result<Normal> {
        match await!(self.blur())? {
            BlurNormal::Pure(n) => Ok(n),
            BlurNormal::Impure(_) => Err(Error::new(
                "ImpureError",
                "impure value detected in pure context",
            )),
        }
    }

    #[async]
    pub fn impure(self) -> Result<Normal> {
        match await!(self.blur())? {
            BlurNormal::Pure(_) => Err(Error::new(
                "PureError",
                "pure value detected in impure context",
            )),
            BlurNormal::Impure(n) => Ok(n),
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
    pub fn list(self) -> Result<List> {
        let n = await!(self.pured())?;

        match n {
            Normal::List(l) => Ok(l),
            _ => Err(await!(Error::not_list(n))?),
        }
    }

    #[async]
    pub fn to_string(self) -> Result<String> {
        await!(await!(self.pured())?.to_string())
    }

    pub fn merge(self, v: Self) -> Self {
        Self::app(
            MERGE,
            Arguments::new(
                &[
                    PositionalArgument::new(self, false),
                    PositionalArgument::new(v, false),
                ],
                &[],
                &[],
            ),
        )
    }
}

impl From<Dictionary> for Value {
    fn from(d: Dictionary) -> Self {
        Value::from(Normal::from(d))
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
        Value::Normal(Ok(BlurNormal::Pure(n)))
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
