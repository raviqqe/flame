use super::super::core::Str;

use super::arguments::Arguments;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    App(Box<Expression>, Arguments),
    Boolean(bool),
    Name(Str),
    Nil,
    Number(f64),
    String(Str),
}
