use futures::prelude::*;

use super::arguments::Arguments;
use super::half_signature::HalfSignature;
use super::optional_parameter::OptionalParameter;
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
        po: Vec<OptionalParameter>,
        pp: Str,
        kr: Vec<Str>,
        ko: Vec<OptionalParameter>,
        kk: Str,
    ) -> Self {
        Signature {
            positionals: HalfSignature::new(pr, po, pp),
            keywords: HalfSignature::new(kr, ko, kk),
        }
    }

    #[async_move]
    pub fn bind(self: Ref<Self>, a: RefMut<Arguments>) -> Result<Vec<Value>> {
        let mut vs = Vec::with_capacity(self.arity());

        await!(Ref(&self.positionals).bind_positionals(a, RefMut(&mut vs),))?;
        await!(Ref(&self.keywords).bind_keywords(a, RefMut(&mut vs),))?;

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
                    vec![OptionalParameter::new("x", 42)],
                    "".into(),
                    vec![],
                    vec![],
                    "".into(),
                ),
                Arguments::positionals(&[42.into()]),
            ),
        ] {
            block_on(Ref(&s).bind(RefMut(&mut a))).unwrap();
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
            block_on(Ref(&s).bind((&mut a).into())).unwrap();
        });
    }
}
