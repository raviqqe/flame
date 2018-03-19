use super::import::Import;
use super::statement::Statement;

#[derive(Clone, Debug, PartialEq)]
pub struct Module {
    imports: Vec<Import>,
    statements: Vec<Statement>,
}

impl Module {
    pub fn new(imports: Vec<Import>, statements: Vec<Statement>) -> Self {
        Module {
            imports,
            statements,
        }
    }
}
