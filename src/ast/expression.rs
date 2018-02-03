use super::arguments::Arguments;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression<'a> {
    App {
        function: Box<Expression<'a>>,
        arguments: Arguments<'a>,
    },
    Boolean(bool),
    Name(&'a str),
    Nil,
    Number(f64),
    String(String),
}
