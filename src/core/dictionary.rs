use std::hash::{Hash, Hasher};
use std::mem::transmute;
use std::sync::Arc;

use hamt_sync::Map;

use super::normal::Normal;
use super::value::Value;

use self::Key::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Key {
    Num(f64),
}

impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match *self {
            Num(n) => state.write_u64(unsafe { transmute(n) }),
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
