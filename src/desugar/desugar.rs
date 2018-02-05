use super::super::ast::Statement;

use super::error::DesugarError;

pub fn desugar(ss: Vec<Statement>) -> Result<Vec<Statement>, DesugarError> {
    Ok(ss)
}
