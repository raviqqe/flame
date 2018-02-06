use std::fmt::{self, Debug, Formatter};
use std::sync::Arc;

use futures::prelude::*;

use super::dictionary::Dictionary;
use super::function::Function;
use super::list::List;
use super::result::Result;

#[derive(Clone)]
pub enum Normal {
    Boolean(bool),
    Dictionary(Dictionary),
    Function(Arc<Function>),
    List(List),
    Nil,
    Number(f64),
    String(String),
}

impl Normal {
    // TODO: Use just #[async].
    #[async(boxed)]
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
        Normal::Function(Arc::new(f))
    }
}

impl From<List> for Normal {
    fn from(l: List) -> Self {
        Normal::List(l)
    }
}
