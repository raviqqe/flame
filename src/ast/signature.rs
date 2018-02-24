use super::optional_parameter::OptionalParameter;

#[derive(Clone, Debug)]
pub struct Signature {
    positionals: HalfSignature,
}

#[derive(Clone, Debug)]
pub struct HalfSignature {
    requireds: Vec<String>,
    optionals: Vec<OptionalParameter>,
    rest: String,
}
