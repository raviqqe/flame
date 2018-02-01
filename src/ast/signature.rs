use super::optional_parameter::OptionalParameter;

#[derive(Clone, Debug)]
pub struct Signature<'a> {
    positionals: HalfSignature<'a>,
}

#[derive(Clone, Debug)]
pub struct HalfSignature<'a> {
    requireds: Vec<&'a str>,
    optionals: Vec<OptionalParameter<'a>>,
    rest: &'a str,
}
