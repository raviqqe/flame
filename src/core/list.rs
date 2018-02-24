use std::sync::Arc;

use futures::prelude::*;

use super::collection::MERGE;
use super::error::Error;
use super::result::Result;
use super::signature::Signature;
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

    pub fn cons(v: impl Into<Value>, l: impl Into<Value>) -> List {
        List::Cons(Arc::new(Cons(v.into(), l.into())))
    }

    pub fn first(&self) -> Result<Value> {
        match *self {
            List::Cons(ref c) => Ok(c.0.clone()),
            List::Empty => Err(Error::empty_list()),
        }
    }

    #[async]
    pub fn rest(self) -> Result<List> {
        match self {
            List::Cons(c) => Ok(await!(c.1.clone().list())?),
            List::Empty => Err(Error::empty_list()),
        }
    }

    pub fn insert(&self, i: usize, v: Value) -> Result<List> {
        if i == 1 {
            return Ok(Self::cons(v, self.clone()));
        }

        match *self {
            List::Cons(ref c) => {
                let Cons(f, r) = (**c).clone();
                Ok(Self::cons(f, r.insert(i - 1, v)))
            }
            List::Empty => Err(Error::empty_list()),
        }
    }

    pub fn merge(&self, v: Value) -> Result<Value> {
        match *self {
            List::Empty => Ok(v),
            List::Cons(ref c) => {
                let Cons(f, r) = (**c).clone();

                Ok(Value::from(Self::cons(
                    f,
                    Value::papp(MERGE.clone(), &[r, v]),
                )))
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        if let List::Empty = *self {
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

pure_function!(
    FIRST,
    Signature::new(
        vec!["list".into()],
        vec![],
        "".into(),
        vec![],
        vec![],
        "".into()
    ),
    first
);

#[async(boxed_send)]
fn first(vs: Vec<Value>) -> Result<Value> {
    let l = await!(vs[0].clone().list())?;
    Ok(l.first()?)
}
