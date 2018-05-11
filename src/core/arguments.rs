use std::mem::replace;

use array_queue::ArrayQueue;
use futures::prelude::*;

use super::collection::MERGE;
use super::dictionary::Dictionary;
use super::error::Error;
use super::list::{List, FIRST, REST};
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
    pub fn new(ps: &[Expansion<Value>], ks: &[Expansion<KeywordArgument>]) -> Self {
        let mut l = None;
        let mut pq = ArrayQueue::new();

        for (i, p) in ps.iter().enumerate() {
            if match *p {
                Expansion::Expanded(_) => true,
                Expansion::Unexpanded(ref v) => pq.push_back(v).is_err(),
            } {
                l = Some(Self::merge_positional_arguments(&ps[i..]));
                break;
            }
        }

        let mut kq = ArrayQueue::new();
        let mut d = None;

        for (i, k) in ks.iter().enumerate() {
            if match *k {
                Expansion::Expanded(_) => true,
                Expansion::Unexpanded(ref k) => kq.push_back(k).is_err(),
            } {
                d = Some(Self::merge_keyword_arguments(&ks[i..]));
                break;
            }
        }

        Arguments {
            positionals: pq,
            expanded_list: l,
            keywords: kq,
            expanded_dict: d,
        }
    }

    pub fn positionals(vs: &[Value]) -> Self {
        let ps: Vec<Expansion<Value>> = vs.iter()
            .map(|v| Expansion::Unexpanded(v.clone()))
            .collect();
        Self::new(&ps, &[])
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
            Some(l) => List::strict_prepend(ps.into_iter(), l).into(),
        }
    }

    #[async]
    pub fn search_keyword(mut self: RefMut<Self>, s: Str) -> Result<Value> {
        for k in &mut self.keywords {
            if s == k.name {
                return Ok(replace(k, KeywordArgument::new("", Value::Nil)).value);
            }
        }

        let v = self.expanded_dict
            .clone()
            .ok_or_else(|| Error::argument("cannot find a keyword argument"))?;
        let d = await!(v.dictionary())?;
        let v = await!(d.clone().find(s.clone().into()))?;

        self.expanded_dict = Some(await!(d.delete(s.into()))?.into());

        Ok(v)
    }

    pub fn rest_keywords(&mut self) -> Value {
        let ks = replace(&mut self.keywords, ArrayQueue::new());
        let d = replace(&mut self.expanded_dict, None);

        let mut v = d.unwrap_or(Dictionary::new().into());

        for k in &ks {
            let k = k.clone();
            v = v.insert(k.name, k.value);
        }

        v
    }

    #[async]
    pub fn check_empty(self: Ref<Self>) -> Result<()> {
        if !self.positionals.is_empty() {
            return Err(Error::argument(&format!(
                "{} positional arguments are left",
                self.positionals.len()
            )));
        }

        if let Some(v) = self.expanded_dict.clone() {
            let n = self.keywords.len() + await!(v.dictionary())?.size();

            if n > 0 {
                return Err(Error::argument(&format!(
                    "{} keyword arguments are left",
                    n
                )));
            }
        }

        Ok(())
    }

    fn merge_positional_arguments(mut ps: &[Expansion<Value>]) -> Value {
        let mut l = List::Empty.into();

        if let Some(&Expansion::Expanded(ref v)) = ps.last() {
            l = v.clone();
            ps = &ps[0..(ps.len() - 1)];
        }

        for p in ps.iter().rev() {
            match *p {
                Expansion::Expanded(ref v) => l = v.merge(l),
                Expansion::Unexpanded(ref v) => l = List::cons(v.clone(), l).into(),
            }
        }

        l
    }

    fn merge_keyword_arguments(mut ks: &[Expansion<KeywordArgument>]) -> Value {
        let mut d = Dictionary::new().into();

        if let Some(&Expansion::Expanded(ref v)) = ks.first() {
            d = v.clone();
            ks = &ks[1..];
        }

        for k in ks {
            match k.clone() {
                Expansion::Expanded(v) => d = d.merge(v),
                Expansion::Unexpanded(k) => d = d.insert(k.name, k.value),
            }
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
pub enum Expansion<T> {
    Unexpanded(T),
    Expanded(Value),
}

#[derive(Clone, Debug)]
pub struct KeywordArgument {
    name: Str,
    value: Value,
}

impl KeywordArgument {
    pub fn new(s: impl Into<Str>, v: impl Into<Value>) -> Self {
        KeywordArgument {
            name: s.into(),
            value: v.into(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::mem::size_of;

    use futures::stable::block_on_stable;
    use test::{black_box, Bencher};

    use super::*;

    #[test]
    fn new() {
        Arguments::new(&[], &[]);
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
                Arguments::new(&[Expansion::Expanded(List::Empty.into())], &[]),
                List::Empty,
            ),
        ]: Vec<(Arguments, List)>
        {
            assert!(block_on_stable(a.rest_positionals().equal(l.into())).unwrap());
        }
    }

    #[test]
    fn size() {
        let s = size_of::<Arguments>();
        assert!(s < 512, "size of Arguments: {}", s);
    }

    #[bench]
    fn bench_arguments_new(b: &mut Bencher) {
        b.iter(|| black_box(Arguments::new(&[], &[])));
    }

    #[bench]
    fn bench_arguments_rest_positionals(b: &mut Bencher) {
        b.iter(|| black_box(Arguments::new(&[], &[])).rest_positionals());
    }
}
