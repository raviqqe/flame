use super::arguments::Arguments;
use super::optional_argument::OptionalArgument;
use super::result::Result;
use super::value::Value;

#[derive(Clone, Debug)]
pub struct HalfSignature {
    requireds: Vec<String>,
    optionals: Vec<OptionalArgument>,
    rest: String,
}

impl HalfSignature {
    pub fn new(rs: Vec<String>, os: Vec<OptionalArgument>, r: String) -> Self {
        HalfSignature {
            requireds: rs,
            optionals: os,
            rest: r,
        }
    }

    pub fn bind_positionals(&self, args: &mut Arguments, vs: &mut Vec<Value>) -> Result<()> {
        unimplemented!()
    }

    pub fn bind_keywords(&self, args: &mut Arguments, vs: &mut Vec<Value>) -> Result<()> {
        unimplemented!()
    }

    fn arity(&self) -> usize {
        self.requireds.len() + self.optionals.len() + if self.rest == "" { 0 } else { 1 }
    }
}
