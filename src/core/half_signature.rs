use super::arguments::Arguments;
use super::optional_argument::OptionalArgument;
use super::result::Result;
use super::value::Value;

#[derive(Clone, Debug, Default)]
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
        for s in &self.requireds {
            vs.push(args.next_positional().unwrap_or(args.search_keyword(&s)?));
        }

        for o in &self.optionals {
            vs.push(
                args.next_positional()
                    .unwrap_or(args.search_keyword(&o.name).unwrap_or(o.value.clone())),
            );
        }

        if self.rest != "" {
            vs.push(args.rest_positionals());
        }

        Ok(())
    }

    pub fn bind_keywords(&self, args: &mut Arguments, vs: &mut Vec<Value>) -> Result<()> {
        for s in &self.requireds {
            vs.push(args.search_keyword(&s)?);
        }

        for o in &self.optionals {
            vs.push(args.search_keyword(&o.name).unwrap_or(o.value.clone()));
        }

        if self.rest != "" {
            vs.push(args.rest_keywords());
        }

        Ok(())
    }

    pub fn arity(&self) -> usize {
        self.requireds.len() + self.optionals.len() + (self.rest == "") as usize
    }
}
