use futures::prelude::*;

use super::arguments::Arguments;
use super::error::Error;
use super::result::Result;
use super::string::Str;
use super::unsafe_ref::{Ref, RefMut};
use super::value::Value;

#[derive(Clone, Debug, Default)]
pub struct PositionalParameters {
    parameters: Vec<Str>,
    rest: Str,
}

impl PositionalParameters {
    pub fn new(rs: Vec<Str>, r: Str) -> Self {
        PositionalParameters {
            parameters: rs,
            rest: r,
        }
    }

    pub fn bind(&self, args: &mut Arguments, vs: &mut Vec<Value>) -> Result<()> {
        for p in &self.parameters {
            match args.next_positional() {
                Some(v) => vs.push(v),
                None => return Err(Error::argument(&format!("argument {:?} is missing", p))),
            }
        }

        if self.rest != "" {
            vs.push(args.rest_positionals());
        }

        Ok(())
    }

    pub fn arity(&self) -> usize {
        self.parameters.len() + (self.rest == "") as usize
    }
}

#[derive(Clone, Debug, Default)]
pub struct KeywordParameters {
    parameters: Vec<OptionalParameter>,
    rest: Str,
}

impl KeywordParameters {
    pub fn new(ps: Vec<OptionalParameter>, r: Str) -> Self {
        KeywordParameters {
            parameters: ps,
            rest: r,
        }
    }

    #[async]
    pub fn bind(
        self: Ref<Self>,
        mut args: RefMut<Arguments>,
        mut vs: RefMut<Vec<Value>>,
    ) -> Result<()> {
        // TODO: Use an iterator with an immovable generator.
        for i in 0..self.parameters.len() {
            let o = self.parameters[i].clone();
            let r = await!(args.search_keyword(o.name));
            vs.push(r.unwrap_or(o.value));
        }

        if self.rest != "" {
            vs.push(args.rest_keywords());
        }

        Ok(())
    }

    pub fn arity(&self) -> usize {
        self.parameters.len() + (self.rest == "") as usize
    }
}

#[derive(Clone, Debug)]
pub struct OptionalParameter {
    pub name: Str,
    pub value: Value,
}

impl OptionalParameter {
    pub fn new(n: impl Into<Str>, v: impl Into<Value>) -> Self {
        OptionalParameter {
            name: n.into(),
            value: v.into(),
        }
    }
}

#[cfg(test)]
mod test {
    use futures::stable::block_on_stable;
    use test::Bencher;

    use super::super::arguments::{Expansion, KeywordArgument};
    use super::*;

    #[test]
    fn new() {
        PositionalParameters::new(vec![], "".into());
    }

    #[test]
    fn positional_parameters_bind() {
        for (ps, mut a, l) in vec![
            (
                PositionalParameters::new(vec![], "".into()),
                Arguments::default(),
                0,
            ),
            (
                PositionalParameters::new(vec!["x".into()], "".into()),
                Arguments::positionals(&[42.into()]),
                1,
            ),
            (
                PositionalParameters::new(vec!["x".into(), "y".into()], "".into()),
                Arguments::positionals(&[42.into(), 42.into()]),
                2,
            ),
        ] {
            let mut v = vec![];

            ps.bind(&mut a, &mut v).unwrap();

            assert_eq!(v.len(), l);
        }
    }

    #[test]
    fn keyword_parameters_bind() {
        for (ks, mut a, l) in vec![
            (
                KeywordParameters::new(vec![OptionalParameter::new("x", 42)], "".into()),
                Arguments::default(),
                1,
            ),
            (
                KeywordParameters::new(vec![OptionalParameter::new("x", 42)], "".into()),
                Arguments::new(&[], &[Expansion::Unexpanded(KeywordArgument::new("x", 42))]),
                1,
            ),
        ] {
            let mut v = vec![];

            block_on_stable(Ref(&ks).bind((&mut a).into(), (&mut v).into())).unwrap();

            assert_eq!(v.len(), l);
        }
    }

    #[test]
    fn positional_parameters_bind_error() {
        for (ps, mut a) in vec![(
            PositionalParameters::new(vec!["x".into()], "".into()),
            Arguments::default(),
        )] {
            ps.bind(&mut a, &mut vec![]).unwrap_err();
        }
    }

    #[bench]
    fn bench_positional_parameters_bind(b: &mut Bencher) {
        let ps = PositionalParameters::new(vec!["x".into()], "".into());
        let a = Arguments::positionals(&[42.into()]);

        b.iter(|| {
            let mut a = a.clone();
            ps.bind(&mut a, &mut Vec::with_capacity(ps.arity()))
                .unwrap();
        });
    }

    #[bench]
    fn bench_positional_parameters_bind_empty(b: &mut Bencher) {
        let ps = PositionalParameters::new(vec![], "".into());
        let a = Arguments::positionals(&[]);

        b.iter(|| {
            let mut a = a.clone();
            ps.bind(&mut a, &mut Vec::with_capacity(ps.arity()))
                .unwrap();
        });
    }

    #[bench]
    fn bench_keyword_parameters_bind(b: &mut Bencher) {
        let ks = KeywordParameters::new(vec![OptionalParameter::new("x", 42)], "".into());
        let a = Arguments::new(&[], &[Expansion::Unexpanded(KeywordArgument::new("x", 42))]);

        b.iter(|| {
            let mut a = a.clone();
            block_on_stable(Ref(&ks).bind(
                (&mut a).into(),
                (&mut Vec::with_capacity(ks.arity())).into(),
            )).unwrap();
        });
    }
}
