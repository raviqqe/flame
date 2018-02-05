use std::fmt;
use std::fmt::{Debug, Formatter};
use std::sync::*;

use futures::prelude::*;

use super::arguments::Arguments;
use super::dictionary::Dictionary;
use super::list::List;
use super::result::Result;
use super::value::Value;

#[derive(Clone)]
pub enum Normal {
    Boolean(bool),
    Dictionary(Dictionary),
    Function(Arc<Fn(Arguments) -> Value + Send + Sync>),
    List(List),
    Nil,
    Number(f64),
    String(String),
}

impl Normal {
    #[async]
    pub fn to_string(self) -> Result<String> {
        Ok(match self {
            Normal::Boolean(b) => (if b { "true" } else { "false" }).to_string(),
            Normal::Dictionary(d) => await!(d.to_string())?,
            Normal::Function(_) => "<function>".to_string(),
            Normal::List(l) => await!(l.to_string())?,
            Normal::Number(n) => n.to_string(),
            Normal::Nil => "nil".to_string(),
            Normal::String(s) => s,
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

impl From<f64> for Normal {
    fn from(n: f64) -> Self {
        Normal::Number(n)
    }
}

impl From<List> for Normal {
    fn from(l: List) -> Self {
        Normal::List(l)
    }
}
