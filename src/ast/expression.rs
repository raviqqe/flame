use super::super::core::Str;

use super::arguments::Arguments;
use super::expansion::Expansion;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    App(Box<Expression>, Arguments),
    Boolean(bool),
    Dictionary(Vec<Expansion<(Expression, Expression)>>),
    List(Vec<Expansion<Expression>>),
    Name(Str),
    Nil,
    Number(f64),
    String(Str),
}
