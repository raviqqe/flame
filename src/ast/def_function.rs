use super::super::core::Str;

use super::expression::Expression;
use super::inner_statement::InnerStatement;
use super::signature::Signature;

#[derive(Clone, Debug, PartialEq)]
pub struct DefFunction {
    name: Str,
    signature: Signature,
    inner_statements: Vec<InnerStatement>,
    body: Expression,
}

impl DefFunction {
    pub fn new(
        name: Str,
        signature: Signature,
        inner_statements: Vec<InnerStatement>,
        body: Expression,
    ) -> Self {
        DefFunction {
            name,
            signature,
            inner_statements,
            body,
        }
    }
}
