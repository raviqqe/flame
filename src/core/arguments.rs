use std::mem::replace;

use array_queue::ArrayQueue;
use futures::prelude::*;

use super::collection::MERGE;
use super::dictionary::Dictionary;
use super::error::Error;
use super::list::{List, FIRST, REST};
use super::normal::Normal;
use super::result::Result;
use super::string::Str;
use super::unsafe_ref::{Ref, RefMut};
use super::utils::papp;
use super::value::Value;

#[derive(Clone, Debug, Default)]
pub struct Arguments {
    positionals: ArrayQueue<[Value; 4]>,
    expanded_list: Option<Value>,
    keywords: ArrayQueue<[KeywordArgument; 4]>,
    expanded_dict: Option<Value>,
}

impl Arguments {
    pub fn new(ps: &[PositionalArgument], ks: &[KeywordArgument], ds: &[Value]) -> Arguments {
        let mut l = None;
        let mut pq = ArrayQueue::new();

        for (i, p) in ps.iter().enumerate() {
            if p.expanded || pq.push_back(&p.value.clone()).is_err() {
                l = Some(Self::merge_positional_arguments(&ps[i..]));
                break;
            }
        }

        let mut kq = ArrayQueue::new();
        let mut d = None;

        for (i, k) in ks.iter().enumerate() {
            if kq.push_back(&k.clone()).is_err() {
                d = Some(Self::merge_keyword_arguments(&ks[i..]).into());
                break;
            }
        }

        if !ds.is_empty() {
            let mut v: Value = d.unwrap_or(Dictionary::new().into());

            for d in ds {
                v = v.merge(d.clone());
            }

            d = Some(v)
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
        if let Some(v) = self.positionals.pop_front() {
            Some(v)
        } else if let Some(ref mut l) = self.expanded_list {
            let ll = l.clone();
            *l = papp(REST.clone(), &[l.clone()]).into();
            Some(papp(FIRST.clone(), &[ll]).into())
        } else {
            None
        }
    }

    pub fn rest_positionals(&mut self) -> Value {
        let ps = replace(&mut self.positionals, ArrayQueue::new());
        let l = replace(&mut self.expanded_list, None);

        match l {
            None => List::new(ps.into_iter()).into(),
            Some(l) => if ps.is_empty() {
                l
            } else {
                List::strict_prepend(ps.into_iter(), l).into()
            },
        }
    }

    #[async]
    pub fn search_keyword(mut this: RefMut<Self>, s: Str) -> Result<Value> {
        for k in &mut this.keywords {
            if s == k.name {
                return Ok(replace(k, KeywordArgument::new("".into(), Normal::Nil.into())).value);
            }
        }

        let v = this.expanded_dict
            .clone()
            .ok_or_else(|| Error::argument("cannot find a keyword argument"))?;
        let d = await!(v.dictionary())?;
        let v = await!(d.clone().find(s.clone().into()))?;

        this.expanded_dict = Some(await!(d.delete(s.into()))?.into());

        Ok(v)
    }

    pub fn rest_keywords(&mut self) -> Value {
        let ks = replace(&mut self.keywords, ArrayQueue::new());
        let d = replace(&mut self.expanded_dict, None);

        let mut v = d.unwrap_or(Value::from(Dictionary::new()));

        for k in &ks {
            let k = k.clone();
            v = v.insert(Value::from(k.name), k.value);
        }

        v
    }

    #[async]
    pub fn check_empty(this: Ref<Self>) -> Result<()> {
        if !this.positionals.is_empty() {
            return Err(Error::argument(&format!(
                "{} positional arguments are left.",
                this.positionals.len()
            )));
        }

        if let Some(v) = this.expanded_dict.clone() {
            let n = this.keywords.len() + await!(v.dictionary())?.size();

            if n > 0 {
                return Err(Error::argument(&format!(
                    "{} keyword arguments are left.",
                    n
                )));
            }
        }

        Ok(())
    }

    fn merge_positional_arguments(mut ps: &[PositionalArgument]) -> Value {
        let mut l = Value::from(List::Empty);

        if let Some(&PositionalArgument {
            value: ref v,
            expanded: true,
        }) = ps.last()
        {
            l = v.clone();
            ps = &ps[0..(ps.len() - 1)];
        }

        for p in ps.iter().rev() {
            if p.expanded {
                l = l.merge(p.value.clone());
            } else {
                l = List::cons(p.value.clone(), l).into();
            }
        }

        l
    }

    fn merge_keyword_arguments(ks: &[KeywordArgument]) -> Dictionary {
        let mut d = Dictionary::new();

        for k in ks {
            let k = k.clone();
            d = d.strict_insert(k.name, k.value);
        }

        d
    }

    pub fn merge(&self, a: &Self) -> Self {
        let mut ps = self.positionals.clone();
        let mut l = self.expanded_list.clone();
        let mut ks = self.keywords.clone();
        let mut d = self.expanded_dict.clone();

        match l {
            Some(ref mut l) => {
                *l = papp(
                    MERGE.clone(),
                    &[l.clone(), List::new(a.positionals.into_iter()).into()],
                );
            }
            None => for v in a.positionals.into_iter() {
                if ps.push_back(v).is_err() {
                    l = Some(List::strict_prepend(&[v.clone()], List::Empty));
                }
            },
        }

        match l {
            Some(ref mut l) => {
                if let Some(ll) = a.expanded_list.clone() {
                    *l = papp(MERGE.clone(), &[l.clone(), ll]);
                }
            }
            None => l = a.expanded_list.clone(),
        }

        match d {
            Some(ref mut d) => for k in a.keywords.into_iter() {
                *d = d.insert(k.name.clone(), k.value.clone());
            },
            None => {
                let mut dd = Dictionary::new();

                for k in a.keywords.into_iter() {
                    if ks.push_back(k).is_err() {
                        dd = dd.strict_insert(k.name.clone(), k.value.clone());
                    }
                }

                if dd.size() != 0 {
                    d = Some(dd.into());
                }
            }
        }

        match d {
            Some(ref mut d) => {
                if let Some(dd) = a.expanded_dict.clone() {
                    *d = papp(MERGE.clone(), &[d.clone(), dd]);
                }
            }
            None => d = a.expanded_dict.clone(),
        }

        Arguments {
            positionals: ps,
            expanded_list: l,
            keywords: ks,
            expanded_dict: d,
        }
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
    pub name: Str,
    pub value: Value,
}

impl KeywordArgument {
    pub fn new(s: Str, v: Value) -> Self {
        KeywordArgument { name: s, value: v }
    }
}

#[cfg(test)]
mod test {
    use std::mem::size_of;

    use super::*;

    #[test]
    fn new() {
        Arguments::new(&[], &[], &[]);
    }

    #[test]
    fn rest_positionals() {
        for (mut a, l) in vec![
            (Arguments::positionals(&[]), List::Empty),
            (
                Arguments::positionals(&[42.into()]),
                List::new(&[42.into()]),
            ),
            (
                Arguments::new(
                    &[PositionalArgument::new(List::Empty.into(), true)],
                    &[],
                    &[],
                ),
                List::Empty,
            ),
        ]: Vec<(Arguments, List)>
        {
            assert!(a.rest_positionals().equal(l.into()).wait().unwrap());
        }
    }

    #[test]
    fn size() {
        let s = size_of::<Arguments>();
        assert!(s < 800, "size of Arguments: {}", s);
    }
}
