use super::super::ast::Statement;

use super::effect::Effect;
use super::error::CompileError;

pub fn compile(ss: Vec<Statement>) -> Result<Vec<Effect>, CompileError> {
    Ok(vec![])
}
