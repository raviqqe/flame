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
    #[async_move]
    pub fn vague(self) -> Result<VagueNormal> {
        match self {
            Value::Normal(p) => p,
            Value::Thunk(t) => await!(t.eval()),
        }
    }

    #[async_move]
    pub fn pured(self) -> Result<Normal> {
        match await!(self.vague())? {
            VagueNormal::Pure(n) => Ok(n),
            VagueNormal::Impure(_) => Err(Error::new(
                "ImpureError",
                "impure value detected in pure context",
            )),
        }
    }

    #[async_move]
    pub fn impure(self) -> Result<Normal> {
        match await!(self.vague())? {
            VagueNormal::Pure(_) => Err(Error::new(
                "PureError",
                "pure value detected in impure context",
            )),
            VagueNormal::Impure(n) => Ok(n),
        }
    }

    #[async_move]
    pub fn boolean(self) -> Result<bool> {
        let n = await!(self.pured())?;

        match n {
            Normal::Boolean(b) => Ok(b),
            _ => Err(await!(Error::not_boolean(n))?),
        }
    }

    #[async_move]
    pub fn dictionary(self) -> Result<Dictionary> {
        let n = await!(self.pured())?;

        match n {
            Normal::Dictionary(d) => Ok(d),
            _ => Err(await!(Error::not_dictionary(n))?),
        }
    }

    #[async_move]
    pub fn function(self) -> Result<Function> {
        let n = await!(self.pured())?;

        match n {
            Normal::Function(f) => Ok(f),
            _ => Err(await!(Error::not_function(n))?),
        }
    }

    #[async_move]
    pub fn index(self) -> Result<usize> {
        let n = await!(self.number())?;

        if n % 1.0 == 0.0 && n >= 1.0 {
            Ok(n as usize)
        } else {
            Err(Error::value(&format!("{} is not an integer", n)))
        }
    }

    #[async_move]
    pub fn list(self) -> Result<List> {
        let n = await!(self.pured())?;

        match n {
            Normal::List(l) => Ok(l),
            _ => Err(await!(Error::not_list(n))?),
        }
    }

    #[async_move]
    pub fn number(self) -> Result<f64> {
        let n = await!(self.pured())?;

        match n {
            Normal::Number(n) => Ok(n),
            _ => Err(await!(Error::not_number(n))?),
        }
    }

    #[async_move]
    pub fn string(self) -> Result<Str> {
        let n = await!(self.pured())?;

        match n {
            Normal::String(s) => Ok(s),
            _ => Err(await!(Error::not_string(n))?),
        }
    }

    #[async_move]
    fn type_name(self) -> Result<Str> {
        Ok(await!(self.pured())?.type_name())
    }

    #[async_move]
    pub fn to_string(self) -> Result<String> {
        await!(await!(self.pured())?.to_string())
    }

    #[async_move]
    pub fn equal(self, v: Self) -> Result<bool> {
        let m = await!(self.pured())?;
        let n = await!(v.pured())?;
        await!(m.equal(n))
    }

    #[async_move]
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

impl From<Thunk> for Value {
    fn from(t: Thunk) -> Self {
        Value::Thunk(t)
    }
}

#[cfg(test)]
mod test {
    use futures::executor::block_on;

    use super::*;

    use super::super::list::List;
    use super::super::utils::TEST_FUNCTION;

    #[test]
    fn from_f64() {
        Value::from(123.0);
    }

    #[test]
    fn to_string() {
        for (v, s) in vec![
            (true.into(), "true"),
            (false.into(), "false"),
            (Dictionary::new().into(), "{}"),
            (
                Dictionary::new().strict_insert("foo", 42.into()).into(),
                "{\"foo\" 42}",
            ),
            (
                Dictionary::new()
                    .strict_insert("foo", 1.into())
                    .strict_insert("bar", 2.into())
                    .into(),
                "{\"bar\" 2 \"foo\" 1}",
            ),
            (TEST_FUNCTION.clone(), "<function>"),
            (List::Empty.into(), "[]"),
            (List::new(&[42.into()]).into(), "[42]"),
            (List::new(&[0.into(), 42.into()]).into(), "[0 42]"),
            (Normal::Nil.into(), "nil"),
            (42.into(), "42"),
            (1.5.into(), "1.5"),
            ("foo".into(), "\"foo\""),
        ]: Vec<(Value, &str)>
        {
            assert_eq!(&block_on(v.clone().to_string()).unwrap(), s);
        }
    }

    #[test]
    fn equal() {
        for (v, w, b) in vec![
            (true.into(), true.into(), true),
            (false.into(), false.into(), true),
            (true.into(), false.into(), false),
            (true.into(), false.into(), false),
            (Dictionary::new().into(), Dictionary::new().into(), true),
            (
                Dictionary::new().strict_insert("foo", 42.into()).into(),
                Dictionary::new().strict_insert("foo", 42.into()).into(),
                true,
            ),
            (
                Dictionary::new().into(),
                Dictionary::new().strict_insert("foo", 42.into()).into(),
                false,
            ),
            (
                Dictionary::new().strict_insert("foo", 42.into()).into(),
                Dictionary::new()
                    .strict_insert("foo", 42.into())
                    .strict_insert("bar", 42.into())
                    .into(),
                false,
            ),
            (List::default().into(), List::default().into(), true),
            (
                List::new(&[0.into()]).into(),
                List::new(&[0.into()]).into(),
                true,
            ),
            (List::default().into(), List::new(&[0.into()]).into(), false),
            (
                List::new(&[0.into()]).into(),
                List::new(&[1.into()]).into(),
                false,
            ),
            (Normal::Nil.into(), Normal::Nil.into(), true),
            (Normal::Nil.into(), 0.into(), false),
            (0.into(), 0.into(), true),
            (0.into(), 1.into(), false),
            ("a".into(), "a".into(), true),
            ("a".into(), "b".into(), false),
        ]: Vec<(Value, Value, bool)>
        {
            assert_eq!(block_on(v.clone().equal(w)).unwrap(), b);
        }
    }

    #[test]
    fn equal_error() {
        for (v, w) in vec![
            (TEST_FUNCTION.clone(), TEST_FUNCTION.clone()),
            (0.into(), TEST_FUNCTION.clone()),
            (TEST_FUNCTION.clone(), 0.into()),
        ]: Vec<(Value, Value)>
        {
            assert!(block_on(v.clone().equal(w)).is_err());
        }
    }

    #[test]
    fn compare() {
        for (v, w, o) in vec![
            (0.into(), 0.into(), Ordering::Equal),
            (0.into(), 1.into(), Ordering::Less),
            (1.into(), 0.into(), Ordering::Greater),
            ("a".into(), "a".into(), Ordering::Equal),
            ("a".into(), "b".into(), Ordering::Less),
            ("b".into(), "a".into(), Ordering::Greater),
            (
                List::default().into(),
                List::default().into(),
                Ordering::Equal,
            ),
            (
                List::new(&[0.into()]).into(),
                List::new(&[0.into()]).into(),
                Ordering::Equal,
            ),
            (
                List::default().into(),
                List::new(&[0.into()]).into(),
                Ordering::Less,
            ),
            (
                List::new(&[0.into()]).into(),
                List::new(&[1.into()]).into(),
                Ordering::Less,
            ),
            (
                List::new(&[0.into()]).into(),
                List::default().into(),
                Ordering::Greater,
            ),
            (
                List::new(&[1.into()]).into(),
                List::new(&[0.into()]).into(),
                Ordering::Greater,
            ),
        ]: Vec<(Value, Value, Ordering)>
        {
            assert_eq!(block_on(v.clone().compare(w)).unwrap(), o);
        }
    }

    #[test]
    fn compare_error() {
        for (v, w) in vec![
            (0.into(), "a".into()),
            (0.into(), List::default().into()),
            (true.into(), true.into()),
            (TEST_FUNCTION.clone(), TEST_FUNCTION.clone()),
            (Normal::Nil.into(), Normal::Nil.into()),
        ]: Vec<(Value, Value)>
        {
            assert!(block_on(v.clone().compare(w)).is_err());
        }
    }
}
