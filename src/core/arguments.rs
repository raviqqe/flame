use std::mem::replace;

use fixed_size_vector::ArrayVec;
use futures::prelude::*;

use super::error::Error;
use super::list::List;
use super::result::Result;
use super::value::Value;

#[derive(Clone, Debug)]
pub struct Arguments {
    positionals: ArrayVec<[Value; 4]>,
    expanded_list: Value,
    keywords: ArrayVec<[KeywordArgument; 4]>,
    expanded_dict: Value,
}

impl Arguments {
    pub fn new(ps: &[PositionalArgument], ks: &[KeywordArgument], ds: &[Value]) -> Arguments {
        let mut l = Value::Invalid;
        let mut pv = ArrayVec::new();

        for (i, p) in ps.iter().enumerate() {
            if p.expanded || !p.expanded && pv.push(&p.value).is_err() {
                l = Self::merge_positional_arguments(&ps[i..]);
                break;
            }
        }

        let mut kv = ArrayVec::new();
        let mut d = Value::Invalid;

        for (i, k) in ks.iter().enumerate() {
            if kv.push(k).is_err() {
                d = Self::merge_keyword_arguments(&ks[i..], ds);
                break;
            }
        }

        for d in ds {
            unimplemented!()
        }

        Arguments {
            positionals: pv,
            expanded_list: l,
            keywords: kv,
            expanded_dict: d,
        }
    }

    pub fn search_keyword(&mut self, s: &str) -> Result<Value> {
        for k in &mut self.keywords {
            if s == k.name {
                return Ok(replace(k, Default::default()).value);
            }
        }

        unimplemented!(); // Search in self.expanded_dict.

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

        // let d = await!(v.clone().dictionary())?;
        unimplemented!();

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

    fn merge_keyword_arguments(ks: &[KeywordArgument], ds: &[Value]) -> Value {
        unimplemented!()
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
