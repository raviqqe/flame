use std::mem::replace;

use futures::prelude::*;

use super::error::Error;
use super::list::List;
use super::result::Result;
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

    pub fn search_keyword(&mut self, s: &str) -> Result<Value> {
        for k in &mut self.keywords {
            if s == k.name {
                return Ok(replace(k, Default::default()).value);
            }
        }

        for v in &mut self.expanded_dicts {
            unimplemented!()
        }

        Err(Error::argument("cannot find a keyword argument"))
    }

    pub fn rest_keywords(&mut self) -> Value {
        unimplemented!()
    }

    #[async]
    pub fn check_empty(self) -> Result<()> {
        if !self.positionals.is_empty() {
            return Err(Error::argument(&format!(
                "{} positional arguments are left.",
                self.positionals.len()
            )));
        }

        let mut n = 0;

        for v in &self.expanded_dicts {
            // let d = await!(v.clone().dictionary())?;
            unimplemented!()
        }

        if n != 0 || self.keywords.len() > 0 {
            return Err(Error::argument(&format!(
                "{} keyword arguments are left.",
                self.keywords.len() + n
            )));
        }

        Ok(())
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

impl Default for KeywordArgument {
    fn default() -> Self {
        KeywordArgument {
            name: String::from(""),
            value: Value::Invalid,
        }
    }
}
