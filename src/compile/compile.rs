use super::super::ast::Module;

use super::compiler::Compiler;
use super::effect::Effect;
use super::error::CompileError;

pub fn compile(m: Module) -> Result<Vec<Effect>, CompileError> {
    Compiler::new().compile_module(m)
}
