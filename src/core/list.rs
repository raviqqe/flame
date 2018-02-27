use std::sync::Arc;

use futures::prelude::*;

use super::arguments::{Arguments, PositionalArgument};
use super::collection::MERGE;
use super::error::Error;
use super::result::Result;
use super::signature::Signature;
use super::utils::{app, papp};
use super::value::Value;

#[derive(Clone, Debug)]
pub struct Cons(Value, Value);

#[derive(Clone, Debug)]
pub enum List {
    Cons(Arc<Cons>),
    Empty,
}

impl List {
    pub fn new<'a, I: IntoIterator<Item = &'a Value>>(vs: I) -> List {
        let mut l = List::Empty;

        for v in vs {
            l = List::Cons(Arc::new(Cons(v.clone(), Value::from(l))));
        }

        l
    }

    pub fn cons(v: impl Into<Value>, l: impl Into<Value>) -> List {
        List::Cons(Arc::new(Cons(v.into(), l.into())))
    }

    pub fn strict_prepend<'a, I: IntoIterator<Item = &'a Value>>(
        vs: I,
        l: impl Into<Value>,
    ) -> Value {
        let mut l = l.into();

        for v in vs {
            l = Self::cons(v.clone(), l).into();
        }

        l
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

                Ok(Value::from(Self::cons(f, papp(MERGE.clone(), &[r, v]))))
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
    l.first()
}

pure_function!(
    REST,
    Signature::new(
        vec!["list".into()],
        vec![],
        "".into(),
        vec![],
        vec![],
        "".into()
    ),
    rest
);

#[async(boxed_send)]
fn rest(vs: Vec<Value>) -> Result<Value> {
    let l = await!(vs[0].clone().list())?;
    Ok(await!(l.rest())?.into())
}

pure_function!(
    PREPEND,
    Signature::new(
        vec![],
        vec![],
        "elemsAndList".into(),
        vec![],
        vec![],
        "".into()
    ),
    prepend
);

#[async(boxed_send)]
fn prepend(vs: Vec<Value>) -> Result<Value> {
    let l = await!(vs[0].clone().list())?;

    if let List::Empty = l {
        l.first()
    } else {
        Ok(List::cons(
            l.first()?,
            app(
                PREPEND.clone(),
                Arguments::new(
                    &[PositionalArgument::new(await!(l.rest())?.into(), true)],
                    &[],
                    &[],
                ),
            ),
        ).into())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        List::new(&[]);
        List::new(&[42.into()]);
        List::new(&[42.into(), 42.into()]);
    }

    #[test]
    fn first() {
        let n = papp(FIRST.clone(), &[List::new(&[42.into()]).into()])
            .number()
            .wait()
            .unwrap();

        assert_eq!(n, 42.0);
    }

    #[test]
    fn rest() {
        papp(REST.clone(), &[List::new(&[42.into()]).into()])
            .list()
            .wait()
            .unwrap();
    }

    #[test]
    fn prepend() {
        papp(
            PREPEND.clone(),
            &[List::new(&[42.into(), List::Empty.into()]).into()],
        ).list()
            .wait()
            .unwrap();
    }
}
