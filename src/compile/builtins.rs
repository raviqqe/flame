use std::collections::HashMap;

use super::super::builtin::*;
use super::super::core::{Str, Value};
use super::super::core::functions::*;

pub fn builtins() -> HashMap<Str, Value> {
    let mut h = HashMap::new();

    for (s, v) in vec![
        ("*", MULTIPLY.clone()),
        ("+", ADD.clone()),
        ("-", SUBTRACT.clone()),
        ("/", DIVIDE.clone()),
        ("first", FIRST.clone()),
        ("if", IF.clone()),
        ("insert", INSERT.clone()),
        ("merge", MERGE.clone()),
        ("rest", REST.clone()),
        ("write", WRITE.clone()),
    ] {
        h.insert(s.into(), v);
    }

    h
}
