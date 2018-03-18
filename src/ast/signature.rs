use super::optional_parameter::OptionalParameter;

#[derive(Clone, Debug, PartialEq)]
pub struct Signature {
    positionals: HalfSignature,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HalfSignature {
    requireds: Vec<String>,
    optionals: Vec<OptionalParameter>,
    rest: String,
}
