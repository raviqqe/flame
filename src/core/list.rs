use std::sync::Arc;

use futures::prelude::*;

use super::collection::MERGE;
use super::error::Error;
use super::result::Result;
use super::value::Value;

#[derive(Clone, Debug)]
pub struct Cons(Value, Value);

#[derive(Clone, Debug)]
pub enum List {
    Cons(Arc<Cons>),
    Empty,
}

impl List {
    fn new(vs: &[Value]) -> List {
        let mut l = List::Empty;

        for v in vs.iter().rev() {
            l = List::Cons(Arc::new(Cons(v.clone(), Value::from(l))));
        }

        l
    }

    pub fn cons(v: Value, l: Value) -> Value {
        Value::from(List::Cons(Arc::new(Cons(v, l))))
    }

    pub fn first(self) -> Result<Value> {
        match self {
            List::Cons(c) => Ok(c.0.clone()),
            List::Empty => Err(Error::empty_list()),
        }
    }

    pub fn rest(self) -> Result<Value> {
        match self {
            List::Cons(c) => Ok(c.1.clone()),
            List::Empty => Err(Error::empty_list()),
        }
    }

    #[async]
    pub fn merge(self, v: Value) -> Result<Value> {
        match self {
            List::Empty => Ok(v),
            List::Cons(c) => {
                let Cons(f, r) = (*c).clone();

                Ok(Value::from(Self::cons(
                    f,
                    Value::papp(MERGE.clone(), &[r, v]),
                )))
            }
        }
    }

    pub fn is_empty(self) -> bool {
        if let List::Empty = self {
            true
        } else {
            false
        }
    }

    #[async]
    pub fn to_string(mut self) -> Result<String> {
        let mut ss = vec![];

        while let List::Cons(a) = self {
            let Cons(v, l) = (*a).clone();
            let s = await!(v.clone().to_string())?;
            ss.push(s);
            self = await!(l.clone().list())?;
        }

        Ok(["[", &ss.join(" ".into()), "]"].join("").to_string())
    }
}
