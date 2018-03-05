use std::cmp::Ordering;
use std::convert::TryInto;
use std::fmt::{self, Debug, Formatter};

use futures::prelude::*;

use super::dictionary::Dictionary;
use super::error::Error;
use super::function::Function;
use super::list::List;
use super::result::Result;
use super::string::Str;

#[derive(Clone)]
pub enum Normal {
    Boolean(bool),
    Dictionary(Dictionary),
    Function(Function),
    List(List),
    Nil,
    Number(f64),
    // TODO: Optimize String embedding small ones into Normal.
    String(Str),
}

impl Normal {
    #[async(boxed_send)]
    pub fn to_string(self) -> Result<String> {
        Ok(match self {
            Normal::Boolean(b) => (if b { "true" } else { "false" }).to_string(),
            Normal::Dictionary(d) => await!(d.to_string())?,
            Normal::Function(_) => "<function>".to_string(),
            Normal::List(l) => await!(l.to_string())?,
            Normal::Number(n) => n.to_string(),
            Normal::Nil => "nil".to_string(),
            Normal::String(s) => ["\"".to_string(), s.try_into()?, "\"".to_string()].join(""),
        })
    }

    pub fn type_name(&self) -> Str {
        match *self {
            Normal::Boolean(_) => "boolean",
            Normal::Dictionary(_) => "dictionary",
            Normal::Function(_) => "function",
            Normal::List(_) => "list",
            Normal::Number(_) => "number",
            Normal::Nil => "nil",
            Normal::String(_) => "string",
        }.into()
    }

    #[async]
    pub fn equal(self, n: Self) -> Result<bool> {
        Ok(match (self.clone(), n.clone()) {
            (Normal::Boolean(x), Normal::Boolean(y)) => x == y,
            (Normal::Dictionary(x), Normal::Dictionary(y)) => await!(x.equal(y))?,
            (Normal::List(x), Normal::List(y)) => await!(x.equal(y))?,
            (Normal::Number(x), Normal::Number(y)) => x == y,
            (Normal::Nil, Normal::Nil) => true,
            (Normal::String(x), Normal::String(y)) => x == y,
            (Normal::Function(_), _) => return Err(await!(Error::not_equalable(self))?),
            (_, Normal::Function(_)) => return Err(await!(Error::not_equalable(n))?),
            _ => false,
        })
    }

    #[async]
    pub fn compare(self, n: Self) -> Result<Ordering> {
        Ok(match (self.clone(), n.clone()) {
            (Normal::List(x), Normal::List(y)) => await!(x.compare(y))?,
            (Normal::Number(x), Normal::Number(y)) => if let Some(o) = x.partial_cmp(&y) {
                o
            } else {
                return Err(await!(Error::not_comparable(self, n))?);
            },
            (Normal::String(x), Normal::String(y)) => x.cmp(&y),
            _ => return Err(await!(Error::not_comparable(self, n))?),
        })
    }
}

impl Debug for Normal {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Normal::Function(_) => write!(f, "<function>"),
            ref x => write!(f, "{:?}", x),
        }
    }
}

impl From<bool> for Normal {
    fn from(b: bool) -> Self {
        Normal::Boolean(b)
    }
}

impl From<Dictionary> for Normal {
    fn from(d: Dictionary) -> Self {
        Normal::Dictionary(d)
    }
}

impl From<f64> for Normal {
    fn from(n: f64) -> Self {
        Normal::Number(n)
    }
}

impl From<Function> for Normal {
    fn from(f: Function) -> Self {
        Normal::Function(f)
    }
}

impl From<List> for Normal {
    fn from(l: List) -> Self {
        Normal::List(l)
    }
}

impl From<usize> for Normal {
    fn from(u: usize) -> Self {
        Normal::from(u as f64)
    }
}

impl<S: Into<Str>> From<S> for Normal {
    fn from(s: S) -> Self {
        Normal::String(s.into())
    }
}

#[cfg(test)]
mod test {
    use std::mem::size_of;

    use super::*;

    #[test]
    fn size() {
        for s in vec![
            size_of::<bool>(),
            size_of::<Dictionary>(),
            size_of::<Function>(),
            size_of::<List>(),
            size_of::<f64>(),
            size_of::<Str>(),
        ] {
            assert!(s <= 2 * size_of::<usize>());
        }

        let s = size_of::<Normal>();
        // TODO: Why not 3 times?
        assert!(s <= 4 * size_of::<usize>(), "size of Normal: {}", s);
    }
}
