use std::convert::TryFrom;
use std::hash::{Hash, Hasher};
use std::mem::transmute;
use std::sync::Arc;

use futures::prelude::*;
use hamt_sync::Map;

use super::error::Error;
use super::normal::Normal;
use super::result::Result;
use super::value::Value;

#[derive(Clone, Debug, PartialEq)]
pub enum Key {
    Nil,
    Number(f64),
    String(Vec<u8>),
}

impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match *self {
            Key::Nil => state.write_u8(0),
            Key::Number(n) => state.write_u64(unsafe { transmute(n) }),
            Key::String(ref s) => state.write(&s),
        }
    }
}

impl TryFrom<Normal> for Key {
    type Error = Error;

    fn try_from(n: Normal) -> Result<Self> {
        match n {
            Normal::Nil => Ok(Key::Nil),
            Normal::Number(n) => Ok(Key::Number(n)),
            Normal::String(s) => Ok(Key::String(s)),
            _ => Err(Error::value("{} cannot be a key in dictionaries")),
        }
    }
}

impl Into<Normal> for Key {
    fn into(self) -> Normal {
        match self {
            Key::Nil => Normal::Nil,
            Key::Number(n) => Normal::Number(n),
            Key::String(s) => Normal::String(s),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Dictionary(Arc<Map<Key, Value>>);

impl Dictionary {
    pub fn new() -> Self {
        Dictionary(Arc::new(Map::new()))
    }

    #[async]
    pub fn to_string(self) -> Result<String> {
        let mut ss = vec![];

        let kvs: Vec<(Key, Value)> = self.0
            .into_iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        for (k, v) in kvs {
            let n: Normal = k.into();
            let k = await!(n.to_string())?;
            ss.push(k);

            let v = await!(v.to_string())?;
            ss.push(v);
        }

        Ok(["{", &ss.join(" ".into()), "}"].join("").to_string())
    }

    pub fn size(&self) -> usize {
        self.0.size()
    }

    #[async]
    pub fn insert(self, k: Value, v: Value) -> Result<Dictionary> {
        let k = Key::try_from(await!(k.pured())?)?;
        Ok(Dictionary::from(self.0.insert(k, v)))
    }

    pub fn merge(&self, d: &Self) -> Self {
        let mut m = (*self.0).clone();

        for (k, v) in &*d.0 {
            m = m.insert(k.clone(), v.clone());
        }

        Dictionary::from(m)
    }
}

impl From<Map<Key, Value>> for Dictionary {
    fn from(m: Map<Key, Value>) -> Self {
        Dictionary(Arc::new(m))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        Dictionary::new();
    }

    #[test]
    fn merge() {
        // TODO: Test filled dictionaries.
        let d = Dictionary::new();
        d.merge(&d);
    }
}
