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

    #[async]
    pub fn bind(self, mut args: Arguments) -> Result<Vec<Value>> {
        let mut vs = vec![];
        await!(HalfSignature::bind_positionals(
            Ref(&self.positionals),
            RefMut(&mut args),
            RefMut(&mut vs),
        ))?;
        await!(HalfSignature::bind_keywords(
            Ref(&self.keywords),
            RefMut(&mut args),
            RefMut(&mut vs),
        ))?;
        await!(args.check_empty())?;
        Ok(vs)
    }

    pub fn arity(&self) -> usize {
        self.positionals.arity() + self.keywords.arity()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        Signature::new(vec![], vec![], "".into(), vec![], vec![], "".into());
    }

    #[test]
    fn bind() {
        for (s, a) in vec![(Signature::default(), Arguments::default())] {
            s.bind(a).wait().unwrap();
        }
    }
}
