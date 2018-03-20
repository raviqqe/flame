use std::collections::HashMap;

use super::super::ast;
use super::super::builtin::LIST;
use super::super::core::{app, Arguments, Dictionary, Expansion, Normal, Str, Value};

use super::builtins::builtins;
use super::effect::Effect;
use super::error::CompileError;

#[derive(Clone, Debug)]
pub struct Compiler {
    environment: HashMap<Str, Value>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            environment: builtins(),
        }
    }

    pub fn compile_module(&mut self, m: ast::Module) -> Result<Vec<Effect>, CompileError> {
        // TODO: Use imports field.

        let mut es = vec![];

        for s in m.statements {
            match s {
                ast::Statement::Effect(ast::Effect { value, expanded }) => {
                    es.push(Effect::new(self.compile_expression(value)?, expanded));
                }
                _ => unimplemented!(),
            }
        }

        Ok(es)
    }

    fn compile_expression(&mut self, e: ast::Expression) -> Result<Value, CompileError> {
        Ok(match e {
            ast::Expression::App(f, a) => {
                app(self.compile_expression(*f)?, self.compile_arguments(a)?)
            }
            ast::Expression::Number(n) => n.into(),
            ast::Expression::Boolean(b) => b.into(),
            ast::Expression::Dictionary(es) => {
                let mut d: Value = Dictionary::new().into();

                for e in es {
                    match e {
                        ast::Expansion::Expanded(e) => d = d.merge(self.compile_expression(e)?),
                        ast::Expansion::Unexpanded((k, v)) => {
                            d = d.insert(self.compile_expression(k)?, self.compile_expression(v)?)
                        }
                    }
                }

                d
            }
            ast::Expression::List(es) => {
                let mut ps = vec![];

                for e in es {
                    ps.push(match e {
                        ast::Expansion::Expanded(e) => {
                            Expansion::Expanded(self.compile_expression(e)?)
                        }
                        ast::Expansion::Unexpanded(e) => {
                            Expansion::Unexpanded(self.compile_expression(e)?)
                        }
                    });
                }

                app(LIST.clone(), Arguments::new(&ps, &[]))
            }
            ast::Expression::Name(n) => self.environment[&n].clone(),
            ast::Expression::Nil => Normal::Nil.into(),
            ast::Expression::String(s) => s.into(),
        })
    }

    fn compile_arguments(&mut self, a: ast::Arguments) -> Result<Arguments, CompileError> {
        unimplemented!()
    }
}
