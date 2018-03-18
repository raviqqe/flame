use super::super::core::Str;

use super::optional_parameter::OptionalParameter;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Signature {
    positionals: HalfSignature,
    keywords: HalfSignature,
}

impl Signature {
    pub fn new(positionals: HalfSignature, keywords: HalfSignature) -> Self {
        Signature {
            positionals,
            keywords,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct HalfSignature {
    requireds: Vec<Str>,
    optionals: Vec<OptionalParameter>,
    rest: Str,
}

impl HalfSignature {
    pub fn new(requireds: Vec<Str>, optionals: Vec<OptionalParameter>, rest: Str) -> Self {
        HalfSignature {
            requireds,
            optionals,
            rest,
        }
    }
}
