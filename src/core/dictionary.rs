use std::hash::{Hash, Hasher};
use std::mem::transmute;
use std::sync::Arc;

use hamt_sync::Map;

use super::value::Value;

use self::Key::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Key {
    Nil,
    Num(f64),
    Str(String),
}

impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match *self {
            Nil => state.write_u8(0),
            Num(n) => state.write_u64(unsafe { transmute(n) }),
            Str(ref s) => state.write(s.as_bytes()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Dictionary(Arc<Map<Key, Value>>);

impl Dictionary {
    fn new() -> Self {
        Dictionary(Arc::new(Map::new()))
    }
}
