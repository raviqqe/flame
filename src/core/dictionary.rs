use std::hash::{Hash, Hasher};
use std::mem::transmute;
use std::sync::Arc;

use futures::prelude::*;
use hamt_sync::Map;

use super::normal::Normal;
use super::result::Result;
use super::value::Value;

#[derive(Clone, Debug, PartialEq)]
pub enum Key {
    Nil,
    Number(f64),
    String(String),
}

impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match *self {
            Key::Nil => state.write_u8(0),
            Key::Number(n) => state.write_u64(unsafe { transmute(n) }),
            Key::String(ref s) => state.write(s.as_bytes()),
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
        let mut m = (*self.0).clone();
        let mut ss = vec!["{".to_string()];

        while let Some((k, v, mm)) = m.first_rest() {
            let n: Normal = k.clone().into();
            let v = v.clone();
            let mm = mm.clone();
            ss.push(await!(n.to_string())?);
            ss.push(await!(v.to_string())?);
            m = mm;
        }

        ss.push("}".to_string());

        Ok(String::from(ss.join(" ")))
    }

    pub fn size(&self) -> usize {
        self.0.size()
    }
}
