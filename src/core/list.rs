use std::sync::Arc;

use super::value::*;
use self::List::*;

#[derive(Clone, Debug)]
struct Cons(Value, Value);

#[derive(Clone, Debug)]
pub enum List {
    Cons(Arc<Cons>),
    Empty,
}

impl List {
    fn new(vs: &[Value]) -> List {
        let mut l = Empty;

        for v in vs.iter().rev() {
            l = List::Cons(Arc::new(Cons(v.clone(), Value::from(l))));
        }

        l
    }
}
