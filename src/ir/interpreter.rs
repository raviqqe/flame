use std::convert::TryInto;

use super::super::core::{app, Arguments, Expansion, KeywordArgument, Str, Value};

use super::ir;

#[derive(Clone, Debug)]
pub struct Interpreter<'a> {
    code: &'a [u8],
    index: usize,
    variables: Vec<Value>,
}

impl<'a> Interpreter<'a> {
    pub fn new(variables: Vec<Value>, code: &'a [u8]) -> Self {
        Interpreter {
            code,
            index: 0,
            variables,
        }
    }

    pub fn interpret(&mut self) -> Value {
        while self.index < self.code.len() - 1 {
            let f = self.get_variable();
            let a = self.interpret_arguments();
            self.variables.push(app(f, a));
        }

        self.get_variable()
    }

    fn interpret_arguments(&mut self) -> Arguments {
        let ps = self.interpret_positional_arguments();
        let ks = self.interpret_keyword_arguments();
        Arguments::new(&ps, &ks)
    }

    fn interpret_positional_arguments(&mut self) -> Vec<Expansion<Value>> {
        let mut ps = vec![];

        for _ in 0..self.read_byte() {
            let e = self.read_byte();
            let v = self.get_variable();

            ps.push(match e.into() {
                ir::Expansion::Expanded => Expansion::Expanded,
                ir::Expansion::Unexpanded => Expansion::Unexpanded,
            }(v));
        }

        ps
    }

    fn interpret_keyword_arguments(&mut self) -> Vec<Expansion<KeywordArgument>> {
        let mut ks = vec![];

        for _ in 0..self.read_byte() {
            ks.push(match self.read_byte().into() {
                ir::Expansion::Expanded => Expansion::Expanded(self.get_variable()),
                ir::Expansion::Unexpanded => {
                    let k = self.get_variable();
                    let v = self.get_variable();
                    Expansion::Unexpanded(KeywordArgument::new(k.try_into().unwrap(): Str, v))
                }
            });
        }

        ks
    }

    fn read_byte(&mut self) -> u8 {
        let b = self.code[self.index];
        self.index += 1;
        b
    }

    fn get_variable(&mut self) -> Value {
        let i = self.read_byte();
        self.variables[i as usize].clone()
    }
}
