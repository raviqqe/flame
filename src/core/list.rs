use std::sync::Arc;

use futures::prelude::*;

use super::result::Result;
use super::value::Value;

#[derive(Clone, Debug)]
struct Cons(Value, Value);

#[derive(Clone, Debug)]
pub enum List {
    Cons(Arc<Cons>),
    Empty,
}

impl List {
    fn new(vs: &[Value]) -> List {
        let mut l = List::Empty;

        for v in vs.iter().rev() {
            l = List::Cons(Arc::new(Cons(v.clone(), Value::from(l))));
        }

        l
    }

    pub fn cons(v: Value, l: Value) -> Value {
        Value::from(List::Cons(Arc::new(Cons(v, l))))
    }

    #[async]
    pub fn to_string(mut self) -> Result<String> {
        let mut ss = vec!["[".to_string()];

        while let List::Cons(a) = self {
            let Cons(v, l) = (*a).clone();
            let s = await!(v.clone().to_string())?;
            ss.push(s);
            self = await!(l.clone().list())?;
        }

        ss.push("]".to_string());

        Ok(ss.join(" "))
    }
}
