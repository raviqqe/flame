use futures::prelude::*;

use super::arguments::Arguments;
use super::half_signature::HalfSignature;
use super::optional_argument::OptionalArgument;
use super::result::Result;
use super::value::Value;

#[derive(Clone, Debug)]
pub struct Signature {
    positionals: HalfSignature,
    keywords: HalfSignature,
}

impl Signature {
    pub fn new(
        pr: Vec<String>,
        po: Vec<OptionalArgument>,
        pp: String,
        kr: Vec<String>,
        ko: Vec<OptionalArgument>,
        kk: String,
    ) -> Self {
        Signature {
            positionals: HalfSignature::new(pr, po, pp),
            keywords: HalfSignature::new(kr, ko, kk),
        }
    }

    #[async]
    pub fn bind(self, mut args: Arguments) -> Result<Vec<Value>> {
        let mut vs = vec![];
        self.positionals.bind_positionals(&mut args, &mut vs)?;
        self.keywords.bind_keywords(&mut args, &mut vs)?;
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
        Signature::new(
            vec![],
            vec![],
            "".to_string(),
            vec![],
            vec![],
            "".to_string(),
        );
    }
}
