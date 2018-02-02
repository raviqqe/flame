use pest::Parser;

use super::super::ast::Effect;
use super::super::ast::Expression;
use super::super::ast::Statement;

const _GRAMMAR: &'static str = include_str!("flame.pest");

#[derive(Parser)]
#[grammar = "parse/flame.pest"]
struct FlameParser;

#[cfg(test)]
mod test {
    use super::*;

    const EXPRESSIONS: &[&'static str] = &["nil", "123", "0.1", "-123", "-0.1"];

    #[test]
    fn nil() {
        for s in vec!["nil"] {
            FlameParser::parse(Rule::nil, s).unwrap();
        }
    }

    #[test]
    fn number() {
        for s in vec!["123", "-0.1"] {
            FlameParser::parse(Rule::number, s).unwrap();
        }
    }

    #[test]
    fn expression() {
        for s in EXPRESSIONS {
            println!("{}", s);
            FlameParser::parse(Rule::expression, s).unwrap();
        }
    }

    #[test]
    fn main_module() {
        for s in &[""] {
            println!("{}", s);
            FlameParser::parse(Rule::main_module, s).unwrap();
        }
    }
}
