use std::str::FromStr;

use pest::Parser;
use pest::iterators::Pair;

use super::super::ast::Expression;
use super::super::ast::Statement;

use super::error::ParsingError;

const _GRAMMAR: &'static str = include_str!("grammer.pest");

#[derive(Parser)]
#[grammar = "parse/grammer.pest"]
struct LanguageParser;

pub fn main_module(s: &str) -> Result<Vec<Statement>, ParsingError> {
    let mut ss = vec![];

    let p = LanguageParser::parse(Rule::main_module, s)?.nth(0).unwrap();

    for p in p.into_inner() {
        ss.push(match p.as_rule() {
            Rule::statement => {
                let p = first(p);

                match p.as_rule() {
                    Rule::effect => Statement::effect(expression(first(p)), false),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        });
    }

    Ok(ss)
}

fn expression(p: Pair<Rule>) -> Expression {
    let p = first(p);

    match p.as_rule() {
        Rule::boolean => Expression::Boolean(FromStr::from_str(p.as_str()).unwrap()),
        Rule::nil => Expression::Nil,
        Rule::number => Expression::Number(FromStr::from_str(p.as_str()).unwrap()),
        Rule::string => Expression::String({
            let s = p.as_str();

            s[1..(s.len() - 1)]
                .replace("\\\"", "\"")
                .replace("\\\\", "\\")
                .replace("\\n", "\n")
                .replace("\\r", "\r")
                .replace("\\t", "\t")
        }),
        _ => unreachable!(),
    }
}

fn first(p: Pair<Rule>) -> Pair<Rule> {
    p.into_inner().nth(0).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXPRESSIONS: &[&'static str] = &["nil", "123", "0.1", "-123", "-0.1", "true", "false"];

    #[test]
    fn name() {
        for s in &[
            "x",
            "x1",
            "func",
            "PureFunc",
            "alskfj1",
            "?",
            "is_boolean?",
            "isBoolean?",
        ] {
            println!("{}", s);
            LanguageParser::parse(Rule::name, s).unwrap();
        }
    }

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
    fn string() {
        for s in vec![
            "\"\"",
            "\"a\"",
            "\"abc\"",
            "\"Hi!\"",
            "\"\\\"\"",
            "\"\\\\\"",
            "\"\\n\"",
            "\"\\r\"",
            "\"\\t\"",
            "\"\\\"\\\\\\n\\r\\t\"",
        ] {
            LanguageParser::parse(Rule::string, s).unwrap();
        }
    }

    #[test]
    fn escaped_string() {
        assert_eq!(
            main_module("\"\\\"\\\\\\n\\r\\t\"").unwrap(),
            vec![
                Statement::effect(Expression::String("\"\\\n\r\t".to_string()), false),
            ]
        );
    }

    #[test]
    fn anonymous_function() {
        for s in &[
            "(\\ () 123)",
            "(\\ (x) x)",
            "(\\ (x y) (+ x y))",
            "(\\ (x y . ..options) \"Hi!\")",
        ] {
            println!("{}", s);
            LanguageParser::parse(Rule::anonymous_function, s).unwrap();
        }
    }

    #[test]
    fn application() {
        for s in &[
            "(foo)",
            "(f x)",
            "(f x y)",
            "(f ..args)",
            "(f x ..args)",
            "(f . x 123)",
            "(f . x 123 y 456)",
            "(f . ..kwargs)",
            "(f . x 123 ..kwargs)",
        ] {
            println!("{}", s);
            LanguageParser::parse(Rule::application, s).unwrap();
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
    fn def_function() {
        for s in &[
            "(def (func) 123)",
            "(  def \n(func) (let  x  42\t) x)",
            "(def (func x) x)",
            "(def (func x y) x)",
            "(def (func (x 123)) x)",
            "(def (func ..args) x)",
            "(def (func . x) x)",
            "(def (func . x y) x)",
            "(def (func . (x 123)) x)",
            "(def (func . ..kwargs) x)",
            "(def (func x y . ..kwargs) x)",
        ] {
            println!("{}", s);
            LanguageParser::parse(Rule::def_function, s).unwrap();
        }
    }

    #[test]
    fn let_variable() {
        for s in &["(let x 123)", "(   let   thisIsNumber \t\n 123\n)"] {
            println!("{}", s);
            LanguageParser::parse(Rule::let_variable, s).unwrap();
        }
    }

    #[test]
    fn import() {
        for s in &["(import \"foo\")", "(import \"x\")"] {
            LanguageParser::parse(Rule::import, s).unwrap();
        }
    }

    #[test]
    fn main_module_combinator() {
        for s in &[
            "",
            " 123 nil \n \ttrue",
            "; comment",
            "; comment\n123",
            "(def (f) 123)",
            "(let x 123)",
        ] {
            println!("{}", s);
            LanguageParser::parse(Rule::main_module, s).unwrap();
        }
    }

    #[test]
    fn main_module_function() {
        for &(s, ref m) in &[
            ("", vec![]),
            (
                "123",
                vec![Statement::effect(Expression::Number(123.0), false)],
            ),
            (
                "true nil 123 \"foo\"",
                vec![
                    Statement::effect(Expression::Boolean(true), false),
                    Statement::effect(Expression::Nil, false),
                    Statement::effect(Expression::Number(123.0), false),
                    Statement::effect(Expression::String("foo".to_string()), false),
                ],
            ),
            (
                " 123 ; foo \n456",
                vec![
                    Statement::effect(Expression::Number(123.0), false),
                    Statement::effect(Expression::Number(456.0), false),
                ],
            ),
        ] {
            println!("{:?}", s);
            println!("{:?}", m);
            assert_eq!(main_module(s), Ok(m.clone()));
        }
    }
}
