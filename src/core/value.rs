use futures::prelude::*;
use std;
use std::cmp::Ordering;
use std::convert::TryInto;

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

#[derive(Clone, Debug)]
pub enum Value {
    // from Normal
    Boolean(bool),
    Dictionary(Dictionary),
    Function(Function),
    List(List),
    Nil,
    Number(f64),
    String(Str),

    Thunk(Thunk),
}

impl Value {
    #[async]
    pub fn pured(self) -> Result<Normal> {
        match self {
            Value::Thunk(t) => await!(t.eval_pure()),
            v => Ok(v.try_into().unwrap()),
        }
    }

    #[async]
    pub fn impure(self) -> Result<Normal> {
        match self {
            Value::Thunk(t) => await!(t.eval_impure()),
            _ => Err(Error::pured()),
        }
    }

    #[async]
    pub fn boolean(self) -> Result<bool> {
        let n = await!(self.pured())?;

        match n {
            Normal::Boolean(b) => Ok(b),
            _ => Err(await!(Error::not_boolean(n.into()))?),
        }
    }

    #[async]
    pub fn dictionary(self) -> Result<Dictionary> {
        let n = await!(self.pured())?;

        match n {
            Normal::Dictionary(d) => Ok(d),
            _ => Err(await!(Error::not_dictionary(n.into()))?),
        }
    }

    #[async]
    pub fn function(self) -> Result<Function> {
        let n = await!(self.pured())?;

        match n {
            Normal::Function(f) => Ok(f),
            _ => Err(await!(Error::not_function(n.into()))?),
        }
    }

    #[async]
    pub fn index(self) -> Result<usize> {
        let n = await!(self.number())?;

        if n % 1.0 == 0.0 && n >= 1.0 {
            Ok(n as usize)
        } else {
            Err(Error::value(&format!("{} is not an integer", n)))
        }
    }

    #[async]
    pub fn list(self) -> Result<List> {
        let n = await!(self.pured())?;

        match n {
            Normal::List(l) => Ok(l),
            _ => Err(await!(Error::not_list(n.into()))?),
        }
    }

    #[async]
    pub fn number(self) -> Result<f64> {
        let n = await!(self.pured())?;

        match n {
            Normal::Number(n) => Ok(n),
            _ => Err(await!(Error::not_number(n.into()))?),
        }
    }

    #[async]
    pub fn string(self) -> Result<Str> {
        let n = await!(self.pured())?;

        match n {
            Normal::String(s) => Ok(s),
            _ => Err(await!(Error::not_string(n.into()))?),
        }
    }

    #[async]
    fn type_name(self) -> Result<Str> {
        Ok(await!(self.pured())?.type_name())
    }

    #[async]
    pub fn to_string(self) -> Result<String> {
        await!(await!(self.pured())?.to_string())
    }

    #[async]
    pub fn equal(self, v: Self) -> Result<bool> {
        let m = await!(self.pured())?;
        let n = await!(v.pured())?;
        await!(m.equal(n))
    }

    #[async]
    pub fn compare(self, v: Self) -> Result<Ordering> {
        let m = await!(self.pured())?;
        let n = await!(v.pured())?;
        await!(m.compare(n))
    }

    // TODO: Any methods should be eager.
    #[deprecated]
    pub fn insert(&self, k: impl Into<Self>, v: impl Into<Self>) -> Self {
        papp(INSERT.clone(), &[self.clone(), k.into(), v.into()])
    }

    #[deprecated]
    pub fn merge(&self, v: Self) -> Self {
        papp(MERGE.clone(), &[self.clone(), v])
    }
}

impl<T: Into<Normal>> From<T> for Value {
    fn from(x: T) -> Self {
        match x.into() {
            Normal::Boolean(b) => Value::Boolean(b),
            Normal::Dictionary(d) => Value::Dictionary(d),
            Normal::Function(f) => Value::Function(f),
            Normal::List(l) => Value::List(l),
            Normal::Nil => Value::Nil,
            Normal::Number(n) => Value::Number(n),
            Normal::String(s) => Value::String(s),
        }
    }
}

impl From<Thunk> for Value {
    fn from(t: Thunk) -> Self {
        Value::Thunk(t)
    }
}

impl TryInto<Str> for Value {
    type Error = Error;

    fn try_into(self) -> Result<Str> {
        match self {
            Value::String(s) => Ok(s),
            _ => Err(Error::unreachable()),
        }
    }
}

impl TryInto<Normal> for Value {
    type Error = Error;

    fn try_into(self) -> std::result::Result<Normal, Self::Error> {
        match self {
            Value::Boolean(b) => Ok(b.into()),
            Value::Dictionary(d) => Ok(d.into()),
            Value::Function(f) => Ok(f.into()),
            Value::List(l) => Ok(l.into()),
            Value::Nil => Ok(Normal::Nil),
            Value::Number(n) => Ok(n.into()),
            Value::String(s) => Ok(s.into()),
            _ => Err(Error::unreachable()),
        }
    }
}

#[cfg(test)]
mod test {
    use std::mem::size_of;

    use futures::stable::block_on_stable;

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
                Dictionary::new().strict_insert("foo", 42).into(),
                "{\"foo\" 42}",
            ),
            (
                Dictionary::new()
                    .strict_insert("foo", 1)
                    .strict_insert("bar", 2)
                    .into(),
                "{\"bar\" 2 \"foo\" 1}",
            ),
            (TEST_FUNCTION.clone(), "<function>"),
            (List::Empty.into(), "[]"),
            (List::new(&[42.into()]).into(), "[42]"),
            (List::new(&[0.into(), 42.into()]).into(), "[0 42]"),
            (Value::Nil, "nil"),
            (42.into(), "42"),
            (1.5.into(), "1.5"),
            ("foo".into(), "\"foo\""),
        ]: Vec<(Value, &str)>
        {
            assert_eq!(&block_on_stable(v.clone().to_string()).unwrap(), s);
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
                Dictionary::new().strict_insert("foo", 42).into(),
                Dictionary::new().strict_insert("foo", 42).into(),
                true,
            ),
            (
                Dictionary::new().into(),
                Dictionary::new().strict_insert("foo", 42).into(),
                false,
            ),
            (
                Dictionary::new().strict_insert("foo", 42).into(),
                Dictionary::new()
                    .strict_insert("foo", 42)
                    .strict_insert("bar", 42)
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
            (Value::Nil, Value::Nil, true),
            (Value::Nil, 0.into(), false),
            (0.into(), 0.into(), true),
            (0.into(), 1.into(), false),
            ("a".into(), "a".into(), true),
            ("a".into(), "b".into(), false),
        ]: Vec<(Value, Value, bool)>
        {
            assert_eq!(block_on_stable(v.clone().equal(w)).unwrap(), b);
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
            assert!(block_on_stable(v.clone().equal(w)).is_err());
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
            assert_eq!(block_on_stable(v.clone().compare(w)).unwrap(), o);
        }
    }

    #[test]
    fn compare_error() {
        for (v, w) in vec![
            (0.into(), "a".into()),
            (0.into(), List::default().into()),
            (true.into(), true.into()),
            (TEST_FUNCTION.clone(), TEST_FUNCTION.clone()),
            (Value::Nil, Value::Nil),
        ]: Vec<(Value, Value)>
        {
            assert!(block_on_stable(v.clone().compare(w)).is_err());
        }
    }

    #[test]
    fn size() {
        assert_eq!(size_of::<Value>(), size_of::<Normal>());
    }
}
