use super::super::core::{app, Arguments, Expansion, KeywordArgument, Value};

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
            let f = self.interpret_function();
            let a = self.interpret_arguments();
            self.variables.push(app(f, a));
        }

        self.variables[self.code[self.index] as usize].clone()
    }

    fn interpret_function(&mut self) -> Value {
        let f = self.variables[self.code[self.index] as usize].clone();
        self.index += 1;
        f
    }

    fn interpret_arguments(&mut self) -> Arguments {
        let ps = self.interpret_positional_arguments();
        let ks = self.interpret_keyword_arguments();
        Arguments::new(&ps, &ks)
    }

    fn interpret_positional_arguments(&mut self) -> Vec<Expansion<Value>> {
        unimplemented!()
    }

    fn interpret_keyword_arguments(&mut self) -> Vec<Expansion<KeywordArgument>> {
        unimplemented!()
    }
}
