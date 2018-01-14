use value::*;

#[derive(Clone, Debug)]
pub struct Arguments {
    positionals: Vec<Value>,
    expanded_list: Value,
    keywords: Vec<KeywordArgument>,
    expanded_dicts: Vec<Value>,
}

impl Arguments {
    pub fn new(ps: Vec<Value>, ks: Vec<KeywordArgument>, ds: Vec<Value>) -> Arguments {
        Arguments {
            positionals: ps,
            expanded_list: Value::Invalid,
            keywords: ks,
            expanded_dicts: ds,
        }
    }
}

#[derive(Clone, Debug)]
pub struct KeywordArgument {
    name: String,
    value: Value,
}

impl KeywordArgument {
    pub fn new(s: String, v: Value) -> Self {
        KeywordArgument { name: s, value: v }
    }
}
