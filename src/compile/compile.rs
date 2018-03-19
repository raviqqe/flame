use super::super::ast::Module;

use super::effect::Effect;
use super::error::CompileError;

pub fn compile(m: Module) -> Result<Vec<Effect>, CompileError> {
    Ok(vec![])
}
