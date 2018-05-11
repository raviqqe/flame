use std::cmp::Ordering;
use std::iter::DoubleEndedIterator;
use std::sync::Arc;

use futures::prelude::*;

use super::arguments::{Arguments, Expansion};
use super::collection::MERGE;
use super::error::Error;
use super::result::Result;
use super::signature::Signature;
use super::utils::app;
use super::value::Value;

#[derive(Clone, Debug)]
pub struct Cons(Value, Value);

#[derive(Clone, Debug)]
pub enum List {
    Cons(Arc<Cons>),
    Empty,
}

impl List {
    pub fn new<'a, I: IntoIterator<Item = &'a Value>>(vs: I) -> List
    where
        <I as IntoIterator>::IntoIter: DoubleEndedIterator,
    {
        let mut l = List::Empty;

        for v in vs.into_iter().rev() {
            l = Self::cons(v.clone(), l);
        }

        l
    }

    pub fn cons(v: impl Into<Value>, l: impl Into<Value>) -> List {
        List::Cons(Arc::new(Cons(v.into(), l.into())))
    }

    pub fn strict_prepend<'a, I: IntoIterator<Item = &'a Value>>(
        vs: I,
        l: impl Into<Value>,
    ) -> Value
    where
        <I as IntoIterator>::IntoIter: DoubleEndedIterator,
    {
        let mut l = l.into();

        for v in vs.into_iter().rev() {
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

    #[async]
    pub fn merge(self, v: Value) -> Result<Value> {
        if await!(v.clone().list())?.is_empty() {
            return Ok(self.clone().into());
        }

        Ok(match self {
            List::Empty => app(
                MERGE.clone(),
                Arguments::new(&[Expansion::Expanded(v)], &[]),
            ).into(),
            List::Cons(ref c) => {
                let Cons(f, r) = (**c).clone();

                Self::cons(
                    f,
                    app(
                        MERGE.clone(),
                        Arguments::new(&[Expansion::Unexpanded(r), Expansion::Expanded(v)], &[]),
                    ),
                ).into()
            }
        })
    }

    pub fn is_empty(&self) -> bool {
        if let List::Empty = *self {
            true
        } else {
            false
        }
    }

    #[async(boxed, send)]
    pub fn to_string(mut self) -> Result<String> {
        let mut ss = vec![];

        while let List::Cons(a) = self {
            let Cons(v, l) = (*a).clone();
            let s = await!(v.clone().to_string())?;
            ss.push(s);
            self = await!(l.clone().list())?;
        }

        Ok(["[", &ss.join(" ".into()), "]"].concat().to_string())
    }

    #[async(boxed, send)]
    pub fn equal(mut self, mut l: Self) -> Result<bool> {
        loop {
            match (self.clone(), l.clone()) {
                (List::Empty, List::Empty) => return Ok(true.into()),
                (List::Empty, List::Cons(_)) => return Ok(false.into()),
                (List::Cons(_), List::Empty) => return Ok(false.into()),
                _ => {
                    let x = self.first()?;
                    let y = l.first()?;

                    if !await!(x.equal(y))? {
                        return Ok(false.into());
                    }

                    self = await!(self.rest())?;
                    l = await!(l.rest())?;
                }
            }
        }
    }

    #[async(boxed, send)]
    pub fn compare(mut self, mut l: Self) -> Result<Ordering> {
        loop {
            match (self.clone(), l.clone()) {
                (List::Empty, List::Empty) => return Ok(Ordering::Equal),
                (List::Empty, List::Cons(_)) => return Ok(Ordering::Less),
                (List::Cons(_), List::Empty) => return Ok(Ordering::Greater),
                _ => {
                    let x = self.first()?;
                    let y = l.first()?;

                    let o = await!(x.compare(y))?;

                    if o != Ordering::Equal {
                        return Ok(o);
                    }

                    self = await!(self.rest())?;
                    l = await!(l.rest())?;
                }
            }
        }
    }
}

impl Default for List {
    fn default() -> Self {
        List::Empty
    }
}

impl<'a> From<&'a [Value]> for List {
    fn from(vs: &'a [Value]) -> Self {
        Self::new(&(vs.into_iter().map(|v| v.clone()).collect(): Vec<Value>))
    }
}

pure_function!(
    FIRST,
    Signature::new(vec!["list".into()], "".into(), vec![], "".into()),
    first
);

#[async(boxed, send)]
fn first(vs: Vec<Value>) -> Result<Value> {
    let l = await!(vs[0].clone().list())?;
    l.first()
}

pure_function!(
    REST,
    Signature::new(vec!["list".into()], "".into(), vec![], "".into()),
    rest
);

#[async(boxed, send)]
fn rest(vs: Vec<Value>) -> Result<Value> {
    let l = await!(vs[0].clone().list())?;
    Ok(await!(l.rest())?.into())
}

pure_function!(
    PREPEND,
    Signature::new(vec![], "elemsAndList".into(), vec![], "".into()),
    prepend
);

#[async(boxed, send)]
fn prepend(vs: Vec<Value>) -> Result<Value> {
    let l = await!(vs[0].clone().list())?;
    let f = l.first()?;
    let r = await!(l.rest())?;

    if let List::Empty = r {
        return Ok(f);
    }

    Ok(List::cons(
        f,
        app(
            PREPEND.clone(),
            Arguments::new(&[Expansion::Expanded(r.into())], &[]),
        ),
    ).into())
}

#[cfg(test)]
mod test {
    use futures::stable::block_on_stable;

    use super::*;

    use super::super::utils::papp;

    #[test]
    fn new() {
        List::new(&[]);
        List::new(&[42.into()]);
        List::new(&[42.into(), 42.into()]);
    }

    #[test]
    fn first() {
        let n = block_on_stable(papp(FIRST.clone(), &[List::new(&[42.into()]).into()]).number())
            .unwrap();

        assert_eq!(n, 42.0);
    }

    #[test]
    fn rest() {
        assert!(
            block_on_stable(
                papp(REST.clone(), &[List::new(&[42.into()]).into()]).equal(List::Empty.into())
            ).unwrap()
        );
    }

    #[test]
    fn prepend() {
        assert!(
            block_on_stable(
                papp(PREPEND.clone(), &[42.into(), List::Empty.into()])
                    .equal(List::new(&[42.into()]).into())
            ).unwrap()
        );
    }

    #[test]
    fn from_slice() {
        List::from(&[42.0.into()] as &[Value]);
    }
}
