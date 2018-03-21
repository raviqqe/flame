use futures::prelude::*;
use std::cmp::Ordering;
use std::convert::TryInto;

use super::collection::{INSERT, MERGE};
use super::dictionary::Dictionary;
use super::error::Error;
use super::function::Function;
use super::list::List;
use super::result::Result;
use super::string::Str;
use super::thunk::Thunk;
use super::utils::papp;

#[derive(Clone, Debug)]
pub enum Value {
    Boolean(bool),
    Dictionary(Dictionary),
    Function(Function),
    List(List),
    Nil,
    Number(f64),
    String(Str),

    Error(Error),
    Thunk(Thunk),
}

impl Value {
    #[async_move]
    pub fn pured(self) -> Result<Value> {
        match self {
            Value::Error(e) => Err(e),
            Value::Thunk(t) => await!(t.eval_pure()),
            v => Ok(v),
        }
    }

    #[async_move]
    pub fn impure(self) -> Result<Value> {
        match self {
            Value::Error(e) => Err(e),
            Value::Thunk(t) => await!(t.eval_impure()),
            v => Err(Error::pured()),
        }
    }

    #[async_move]
    pub fn boolean(self) -> Result<bool> {
        let v = await!(self.pured())?;

        match v {
            Value::Boolean(b) => Ok(b),
            _ => Err(await!(Error::not_boolean(v))?),
        }
    }

    #[async_move]
    pub fn dictionary(self) -> Result<Dictionary> {
        let v = await!(self.pured())?;

        match v {
            Value::Dictionary(d) => Ok(d),
            _ => Err(await!(Error::not_dictionary(v))?),
        }
    }

    #[async_move]
    pub fn function(self) -> Result<Function> {
        let v = await!(self.pured())?;

        match v {
            Value::Function(f) => Ok(f),
            _ => Err(await!(Error::not_function(v))?),
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
        let v = await!(self.pured())?;

        match v {
            Value::List(l) => Ok(l),
            _ => Err(await!(Error::not_list(v))?),
        }
    }

    #[async_move]
    pub fn number(self) -> Result<f64> {
        let v = await!(self.pured())?;

        match v {
            Value::Number(n) => Ok(n),
            _ => Err(await!(Error::not_number(v))?),
        }
    }

    #[async_move]
    pub fn string(self) -> Result<Str> {
        let v = await!(self.pured())?;

        match v {
            Value::String(s) => Ok(s),
            _ => Err(await!(Error::not_string(v))?),
        }
    }

    #[async_move]
    fn type_name(self) -> Result<Str> {
        let v = await!(self.pured())?;

        Ok(match v {
            Value::Boolean(_) => "boolean",
            Value::Dictionary(_) => "dictionary",
            Value::Function(_) => "function",
            Value::List(_) => "list",
            Value::Number(_) => "number",
            Value::Nil => "nil",
            Value::String(_) => "string",
            _ => unreachable!(),
        }.into())
    }

    #[async_move]
    pub fn to_string(self) -> Result<String> {
        let v = await!(self.pured())?;

        Ok(match v {
            Value::Boolean(b) => (if b { "true" } else { "false" }).to_string(),
            Value::Dictionary(d) => await!(d.to_string())?,
            Value::Function(_) => "<function>".to_string(),
            Value::List(l) => await!(l.to_string())?,
            Value::Number(n) => n.to_string(),
            Value::Nil => "nil".to_string(),
            Value::String(s) => ["\"".to_string(), s.try_into()?, "\"".to_string()].join(""),
            _ => unreachable!(),
        })
    }

    #[async_move]
    pub fn equal(self, w: Self) -> Result<bool> {
        let v = await!(self.pured())?;
        let w = await!(w.pured())?;

        Ok(match (v, w) {
            (Value::Boolean(x), Value::Boolean(y)) => x == y,
            (Value::Dictionary(x), Value::Dictionary(y)) => await!(x.equal(y))?,
            (Value::List(x), Value::List(y)) => await!(x.equal(y))?,
            (Value::Number(x), Value::Number(y)) => x == y,
            (Value::Nil, Value::Nil) => true,
            (Value::String(x), Value::String(y)) => x == y,
            (Value::Function(f), _) => return Err(await!(Error::not_equalable(f.into()))?),
            (_, Value::Function(f)) => return Err(await!(Error::not_equalable(f.into()))?),
            _ => false,
        })
    }

    #[async_move]
    pub fn compare(self, w: Self) -> Result<Ordering> {
        let v = await!(self.pured())?;
        let w = await!(w.pured())?;

        Ok(match (v, w) {
            (Value::List(x), Value::List(y)) => await!(x.compare(y))?,
            (Value::Number(x), Value::Number(y)) => if let Some(o) = x.partial_cmp(&y) {
                o
            } else {
                return Err(await!(Error::not_comparable(x.into(), y.into()))?);
            },
            (Value::String(x), Value::String(y)) => x.cmp(&y),
            (v, w) => return Err(await!(Error::not_comparable(v, w))?),
        })
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

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Boolean(b)
    }
}

impl From<Dictionary> for Value {
    fn from(d: Dictionary) -> Self {
        Value::Dictionary(d)
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Value::Number(n)
    }
}

impl From<Function> for Value {
    fn from(f: Function) -> Self {
        Value::Function(f)
    }
}

impl From<List> for Value {
    fn from(l: List) -> Self {
        Value::List(l)
    }
}

impl From<usize> for Value {
    fn from(u: usize) -> Self {
        Value::from(u as f64)
    }
}

impl<S: Into<Str>> From<S> for Value {
    fn from(s: S) -> Self {
        Value::String(s.into())
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

#[cfg(test)]
mod test {
    use std::mem::size_of;

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
            (Value::Nil, Value::Nil),
        ]: Vec<(Value, Value)>
        {
            assert!(block_on(v.clone().compare(w)).is_err());
        }
    }

    #[test]
    fn size() {
        let s = size_of::<Value>();

        assert!(
            s <= 2 * size_of::<u64>() + size_of::<usize>(),
            "size of Value: {}",
            s
        );
    }
}
