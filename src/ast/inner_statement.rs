use super::def_function::DefFunction;
use super::let_variable::LetVariable;

#[derive(Clone, Debug, PartialEq)]
pub enum InnerStatement {
    DefFunction(DefFunction),
    LetVariable(LetVariable),
}
