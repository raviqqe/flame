use super::arguments::Arguments;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    App {
        function: Box<Expression>,
        arguments: Arguments,
    },
    Boolean(bool),
    Name(String),
    Nil,
    Number(f64),
    String(String),
}
