use pest::{Error, Parser};
use pest::iterators::Pair;

use super::super::ast::Expression;
use super::super::ast::Statement;

const _GRAMMAR: &'static str = include_str!("grammer.pest");

#[derive(Parser)]
#[grammar = "parse/grammer.pest"]
struct LanguageParser;

pub fn main_module(s: &str) -> Result<Vec<Statement>, Error<Rule>> {
    let mut ss = vec![];

    let p = LanguageParser::parse(Rule::main_module, s)?.nth(0).unwrap();

    for p in p.into_inner() {
        ss.push(match p.as_rule() {
            Rule::statement => {
                let p = p.into_inner().nth(0).unwrap();

                match p.as_rule() {
                    Rule::effect => {
                        Statement::effect(expression(p.into_inner().nth(0).unwrap()), false)
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        });
    }

    Ok(ss)
}

fn expression<'a>(p: Pair<Rule>) -> Expression<'a> {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXPRESSIONS: &[&'static str] = &["nil", "123", "0.1", "-123", "-0.1", "true", "false"];

    #[test]
    fn boolean() {
        for s in vec!["true", "false"] {
            LanguageParser::parse(Rule::boolean, s).unwrap();
        }
    }

    #[test]
    fn nil() {
        LanguageParser::parse(Rule::nil, "nil").unwrap();
    }

    #[test]
    fn number() {
        for s in vec!["123", "-0.1"] {
            LanguageParser::parse(Rule::number, s).unwrap();
        }
    }

    #[test]
    fn expression() {
        for s in EXPRESSIONS {
            println!("{}", s);
            LanguageParser::parse(Rule::expression, s).unwrap();
        }
    }

    #[test]
    fn main_module_combinator() {
        for s in &["", " 123 nil \n \ttrue", "; comment", "; comment\n123"] {
            println!("{}", s);
            LanguageParser::parse(Rule::main_module, s).unwrap();
        }
    }

    #[test]
    fn main_module_function() {
        for &(s, ref m) in &[("", Vec::new())] {
            println!("{:?}", s);
            println!("{:?}", m);
            assert_eq!(main_module(s), Ok(m.clone()));
        }
    }
}
