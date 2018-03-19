use super::super::core::Str;

use super::arguments::Arguments;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    App(Box<Expression>, Arguments),
    Boolean(bool),
    Dictionary(Vec<DictionaryElement>),
    List(Vec<ListElement>),
    Name(Str),
    Nil,
    Number(f64),
    String(Str),
}

#[derive(Clone, Debug, PartialEq)]
pub enum DictionaryElement {
    Unexpanded(Str, Expression),
    Expanded(Expression),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ListElement {
    Unexpanded(Expression),
    Expanded(Expression),
}
