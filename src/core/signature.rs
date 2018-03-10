use futures::prelude::*;

use super::arguments::Arguments;
use super::half_signature::HalfSignature;
use super::optional_argument::OptionalArgument;
use super::result::Result;
use super::string::Str;
use super::unsafe_ref::{Ref, RefMut};
use super::value::Value;

#[derive(Clone, Debug, Default)]
pub struct Signature {
    positionals: HalfSignature,
    keywords: HalfSignature,
}

impl Signature {
    pub fn new(
        pr: Vec<Str>,
        po: Vec<OptionalArgument>,
        pp: Str,
        kr: Vec<Str>,
        ko: Vec<OptionalArgument>,
        kk: Str,
    ) -> Self {
        Signature {
            positionals: HalfSignature::new(pr, po, pp),
            keywords: HalfSignature::new(kr, ko, kk),
        }
    }

    #[async_move]
    pub fn bind(this: Ref<Self>, a: RefMut<Arguments>) -> Result<Vec<Value>> {
        let mut vs = Vec::with_capacity(this.arity());

        await!(HalfSignature::bind_positionals(
            Ref(&this.positionals),
            a.clone(),
            RefMut(&mut vs),
        ))?;

        await!(HalfSignature::bind_keywords(
            Ref(&this.keywords),
            a.clone(),
            RefMut(&mut vs),
        ))?;

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

    use super::*;

    #[test]
    fn new() {
        Signature::new(vec![], vec![], "".into(), vec![], vec![], "".into());
    }

    #[test]
    fn bind() {
        for (s, mut a) in vec![
            (Signature::default(), Arguments::default()),
            (
                Signature::new(
                    vec!["x".into()],
                    vec![],
                    "".into(),
                    vec![],
                    vec![],
                    "".into(),
                ),
                Arguments::positionals(&[42.into()]),
            ),
            (
                Signature::new(
                    vec![],
                    vec![OptionalArgument::new("x", 42)],
                    "".into(),
                    vec![],
                    vec![],
                    "".into(),
                ),
                Arguments::positionals(&[42.into()]),
            ),
        ] {
            block_on(Signature::bind(Ref(&s), RefMut(&mut a))).unwrap();
        }
    }

    #[bench]
    fn bench_signature_bind(b: &mut Bencher) {
        let s = Signature::new(
            vec!["x".into()],
            vec![],
            "".into(),
            vec![],
            vec![],
            "".into(),
        );
        let a = Arguments::positionals(&[42.into()]);

        b.iter(|| {
            let mut a = a.clone();
            Signature::bind((&s).into(), (&mut a).into());
        });
    }
}
