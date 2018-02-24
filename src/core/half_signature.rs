use futures::prelude::*;

use super::arguments::Arguments;
use super::optional_argument::OptionalArgument;
use super::result::Result;
use super::unsafe_ref::{Ref, RefMut};
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

    #[async]
    pub fn bind_positionals(
        this: Ref<Self>,
        mut args: RefMut<Arguments>,
        mut vs: RefMut<Vec<Value>>,
    ) -> Result<()> {
        for s in this.requireds.clone() {
            let v = await!(Arguments::search_keyword(args.clone(), s.clone()))?;
            vs.push(args.clone().next_positional().unwrap_or(v));
        }

        for o in this.optionals.clone() {
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

    #[async]
    pub fn bind_keywords(
        this: Ref<HalfSignature>,
        mut args: RefMut<Arguments>,
        mut vs: RefMut<Vec<Value>>,
    ) -> Result<()> {
        for s in this.requireds.clone() {
            let v = await!(Arguments::search_keyword(args.clone(), s))?;
            vs.push(v);
        }

        for o in this.optionals.clone() {
            let r = await!(Arguments::search_keyword(args.clone(), o.name));
            vs.push(r.unwrap_or(o.value.clone()));
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
