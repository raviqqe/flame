use super::list::List;
use super::value::Value;

#[derive(Clone, Debug)]
pub struct Arguments {
    positionals: Vec<Value>,
    expanded_list: Value,
    keywords: Vec<KeywordArgument>,
    expanded_dicts: Vec<Value>,
}

impl Arguments {
    pub fn new(ps: Vec<PositionalArgument>, ks: Vec<KeywordArgument>, ds: Vec<Value>) -> Arguments {
        let mut l = Value::Invalid;
        let mut vs = vec![];

        for (i, p) in ps.iter().enumerate() {
            if p.expanded {
                l = Self::merge_positional_arguments(&ps.as_slice()[i..]);
                break;
            }

            vs.push(p.value.clone());
        }

        Arguments {
            positionals: vs,
            expanded_list: l,
            keywords: ks,
            expanded_dicts: ds,
        }
    }

    fn merge_positional_arguments(ps: &[PositionalArgument]) -> Value {
        let mut l = Value::from(List::Empty);

        if let Some(&PositionalArgument {
            value: ref v,
            expanded: true,
        }) = ps.last()
        {
            l = v.clone()
        }

        for p in ps.iter().rev() {
            if p.expanded {
                l = unimplemented!();
            } else {
                l = List::cons(p.value.clone(), l)
            }
        }

        l
    }
}

#[derive(Clone, Debug)]
pub struct PositionalArgument {
    pub value: Value,
    pub expanded: bool,
}

impl PositionalArgument {
    pub fn new(s: String, v: Value, e: bool) -> Self {
        PositionalArgument {
            value: v,
            expanded: e,
        }
    }
}

#[derive(Clone, Debug)]
pub struct KeywordArgument {
    pub name: String,
    pub value: Value,
}

impl KeywordArgument {
    pub fn new(s: String, v: Value) -> Self {
        KeywordArgument { name: s, value: v }
    }
}
