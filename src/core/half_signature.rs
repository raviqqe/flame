use futures::prelude::*;

use super::arguments::Arguments;
use super::optional_parameter::OptionalParameter;
use super::result::Result;
use super::string::Str;
use super::unsafe_ref::{Ref, RefMut};
use super::value::Value;

#[derive(Clone, Debug, Default)]
pub struct HalfSignature {
    requireds: Vec<Str>,
    optionals: Vec<OptionalParameter>,
    rest: Str,
}

impl HalfSignature {
    pub fn new(rs: Vec<Str>, os: Vec<OptionalParameter>, r: Str) -> Self {
        HalfSignature {
            requireds: rs,
            optionals: os,
            rest: r,
        }
    }

    #[async_move]
    pub fn bind_positionals(
        this: Ref<Self>,
        mut args: RefMut<Arguments>,
        mut vs: RefMut<Vec<Value>>,
    ) -> Result<()> {
        for i in 0..this.requireds.len() {
            let v = match args.clone().next_positional() {
                Some(v) => v,
                None => await!(Arguments::search_keyword(
                    args.clone(),
                    this.requireds[i].clone()
                ))?,
            };

            vs.push(v);
        }

        for i in 0..this.optionals.len() {
            let o = this.optionals[i].clone();
            let r = await!(Arguments::search_keyword(args.clone(), o.name));
            vs.push(
                args.clone()
                    .next_positional()
                    .unwrap_or(r.unwrap_or(o.value.clone())),
            );
        }

        if this.rest != "" {
            vs.push(args.rest_positionals());
        }

        Ok(())
    }

    #[async_move]
    pub fn bind_keywords(
        this: Ref<HalfSignature>,
        mut args: RefMut<Arguments>,
        mut vs: RefMut<Vec<Value>>,
    ) -> Result<()> {
        for i in 0..this.requireds.len() {
            let v = await!(Arguments::search_keyword(
                args.clone(),
                this.requireds[i].clone()
            ))?;
            vs.push(v);
        }

        for i in 0..this.optionals.len() {
            let o = this.optionals[i].clone();
            let r = await!(Arguments::search_keyword(args.clone(), o.name));
            vs.push(r.unwrap_or(o.value));
        }

        if this.rest != "" {
            vs.push(args.rest_keywords());
        }

        Ok(())
    }

    pub fn arity(&self) -> usize {
        self.requireds.len() + self.optionals.len() + (self.rest == "") as usize
    }
}

#[cfg(test)]
mod test {
    use futures::executor::block_on;
    use test::Bencher;

    use super::*;
    use super::super::arguments::{Expansion, KeywordArgument};

    #[test]
    fn new() {
        HalfSignature::new(vec![], vec![], "".into());
    }

    #[test]
    fn bind_positionals() {
        for (s, mut a, l) in vec![
            (
                HalfSignature::new(vec![], vec![], "".into()),
                Arguments::positionals(&[]),
                0,
            ),
            (
                HalfSignature::new(vec!["x".into()], vec![], "".into()),
                Arguments::positionals(&[42.into()]),
                1,
            ),
            (
                HalfSignature::new(vec!["x".into(), "y".into()], vec![], "".into()),
                Arguments::positionals(&[42.into(), 42.into()]),
                2,
            ),
            (
                HalfSignature::new(vec![], vec![OptionalParameter::new("x", 42)], "".into()),
                Arguments::positionals(&[]),
                1,
            ),
            (
                HalfSignature::new(vec![], vec![OptionalParameter::new("x", 42)], "".into()),
                Arguments::positionals(&[42.into()]),
                1,
            ),
        ] {
            let mut v = vec![];

            block_on(HalfSignature::bind_positionals(
                (&s).into(),
                (&mut a).into(),
                (&mut v).into(),
            )).unwrap();

            assert_eq!(v.len(), l);
        }
    }

    #[test]
    fn bind_positionals_error() {
        for (s, mut a) in vec![
            (
                HalfSignature::new(vec!["x".into()], vec![], "".into()),
                Arguments::positionals(&[]),
            ),
        ] {
            block_on(HalfSignature::bind_positionals(
                (&s).into(),
                (&mut a).into(),
                (&mut vec![]).into(),
            )).unwrap_err();
        }
    }

    #[bench]
    fn bench_half_signature_bind_positionals(b: &mut Bencher) {
        let s = HalfSignature::new(vec!["x".into()], vec![], "".into());
        let a = Arguments::positionals(&[42.into()]);

        b.iter(|| {
            let mut a = a.clone();
            block_on(HalfSignature::bind_positionals(
                (&s).into(),
                (&mut a).into(),
                (&mut Vec::with_capacity(s.arity())).into(),
            )).unwrap();
        });
    }

    #[bench]
    fn bench_half_signature_bind_keywords(b: &mut Bencher) {
        let s = HalfSignature::new(vec!["x".into()], vec![], "".into());
        let a = Arguments::new(&[], &[Expansion::Unexpanded(KeywordArgument::new("x", 42))]);

        b.iter(|| {
            let mut a = a.clone();
            block_on(HalfSignature::bind_positionals(
                (&s).into(),
                (&mut a).into(),
                (&mut Vec::with_capacity(s.arity())).into(),
            )).unwrap();
        });
    }
}
