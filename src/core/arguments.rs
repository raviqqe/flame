use std::mem::replace;

use array_queue::ArrayQueue;
use futures::prelude::*;

use super::error::Error;
use super::dictionary::Dictionary;
use super::list::List;
use super::result::Result;
use super::value::Value;

#[derive(Clone, Debug, Default)]
pub struct Arguments {
    positionals: ArrayQueue<[Option<Value>; 4]>,
    expanded_list: Option<Value>,
    keywords: ArrayQueue<[Option<KeywordArgument>; 4]>,
    expanded_dict: Option<Value>,
}

impl Arguments {
    pub fn new(ps: &[PositionalArgument], ks: &[KeywordArgument], ds: &[Value]) -> Arguments {
        let mut l = None;
        let mut pq = ArrayQueue::new();

        for (i, p) in ps.iter().enumerate() {
            if p.expanded || !p.expanded && pq.push_back(&Some(p.value.clone())).is_err() {
                l = Some(Self::merge_positional_arguments(&ps[i..]));
                break;
            }
        }

        let mut kq = ArrayQueue::new();
        let mut d = None;

        for (i, k) in ks.iter().enumerate() {
            if kq.push_back(&Some(k.clone())).is_err() {
                d = Some(Self::merge_keyword_arguments(&ks[i..]));
                break;
            }
        }

        if !ds.is_empty() {
            d = {
                let mut v = d.unwrap_or(Value::from(Dictionary::new()));

                for d in ds {
                    v = v.merge(d.clone());
                }

                Some(v)
            };
        }

        Arguments {
            positionals: pq,
            expanded_list: l,
            keywords: kq,
            expanded_dict: d,
        }
    }

    pub fn positionals(vs: &[Value]) -> Self {
        let ps: Vec<PositionalArgument> = vs.iter()
            .map(|v| PositionalArgument::new(v.clone(), false))
            .collect();
        Arguments::new(&ps, &[], &[])
    }

    pub fn next_positional(&mut self) -> Option<Value> {
        unimplemented!()
    }

    pub fn rest_positionals(&mut self) -> Value {
        unimplemented!()
    }

    pub fn search_keyword(&mut self, s: &str) -> Result<Value> {
        for o in &mut self.keywords {
            let n = o.clone().unwrap().name;

            if s == n {
                return Ok(replace(o, None).unwrap().value);
            }
        }

        unimplemented!(); // Search in self.expanded_dict.

        Err(Error::argument("cannot find a keyword argument"))
    }

    pub fn rest_keywords(&mut self) -> Value {
        let ks = replace(&mut self.keywords, ArrayQueue::new());
        let mut d = replace(&mut self.expanded_dict, None);

        let mut v = d.unwrap_or(Value::from(Dictionary::new()));

        for k in &ks {
            let k = k.clone().unwrap();
            v = v.insert(Value::from(k.name), k.value);
        }

        v
    }

    #[async]
    pub fn check_empty(self) -> Result<()> {
        if !self.positionals.is_empty() {
            return Err(Error::argument(&format!(
                "{} positional arguments are left.",
                self.positionals.len()
            )));
        }

        if let Some(v) = self.expanded_dict {
            let n = self.keywords.len() + await!(v.dictionary())?.size();

            if n > 0 {
                return Err(Error::argument(&format!(
                    "{} keyword arguments are left.",
                    n
                )));
            }
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

    fn merge_keyword_arguments(ks: &[KeywordArgument]) -> Value {
        unimplemented!()
    }
}

#[derive(Clone, Debug)]
pub struct PositionalArgument {
    pub value: Value,
    pub expanded: bool,
}

impl PositionalArgument {
    pub fn new(v: Value, e: bool) -> Self {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        Arguments::new(&[], &[], &[]);
    }
}
