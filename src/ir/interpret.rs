use super::super::core::Value;

use super::interpreter::Interpreter;

pub fn interpret(vs: Vec<Value>, bs: &[u8]) -> Value {
    Interpreter::new(vs, bs).interpret()
}
