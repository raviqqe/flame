use futures::prelude::*;

use super::arguments::Arguments;
use super::parameters::{KeywordParameters, OptionalParameter, PositionalParameters};
use super::result::Result;
use super::string::Str;
use super::unsafe_ref::{Ref, RefMut};
use super::value::Value;

#[derive(Clone, Debug, Default)]
pub struct Signature {
    positionals: PositionalParameters,
    keywords: KeywordParameters,
}

impl Signature {
    pub fn new(ps: Vec<Str>, pr: Str, ks: Vec<OptionalParameter>, kr: Str) -> Self {
        Signature {
            positionals: PositionalParameters::new(ps, pr),
            keywords: KeywordParameters::new(ks, kr),
        }
    }

    #[async_move]
    pub fn bind(self: Ref<Self>, a: RefMut<Arguments>) -> Result<Vec<Value>> {
        let mut vs = Vec::with_capacity(self.arity());

        await!(Ref(&self.positionals).bind(a, RefMut(&mut vs)))?;
        await!(Ref(&self.keywords).bind(a, RefMut(&mut vs)))?;

        await!(Arguments::check_empty(a.into()))?;

        Ok(vs)
    }

    pub fn arity(&self) -> usize {
        self.positionals.arity() + self.keywords.arity()
    }
}

#[cfg(test)]
mod test {
    use futures::executor::block_on;
    use test::Bencher;

    use super::super::arguments::{Expansion, KeywordArgument};

    use super::*;

    #[test]
    fn new() {
        Signature::new(vec![], "".into(), vec![], "".into());
    }

    #[test]
    fn bind() {
        for (s, mut a) in vec![
            (Signature::default(), Arguments::default()),
            (
                Signature::new(vec!["x".into()], "".into(), vec![], "".into()),
                Arguments::positionals(&[42.into()]),
            ),
            (
                Signature::new(
                    vec![],
                    "".into(),
                    vec![OptionalParameter::new("x", 42)],
                    "".into(),
                ),
                Arguments::new(&[], &[Expansion::Unexpanded(KeywordArgument::new("x", 42))]),
            ),
        ] {
            block_on(Ref(&s).bind(RefMut(&mut a))).unwrap();
        }
    }

    #[bench]
    fn bench_signature_bind(b: &mut Bencher) {
        let s = Signature::new(vec!["x".into()], "".into(), vec![], "".into());
        let a = Arguments::positionals(&[42.into()]);

        b.iter(|| {
            let mut a = a.clone();
            block_on(Ref(&s).bind((&mut a).into())).unwrap();
        });
    }
}
