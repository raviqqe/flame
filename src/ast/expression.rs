use super::app::App;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression<'a> {
    Boolean(bool),
    App(&'a App<'a>),
    Name(&'a str),
    Nil,
    Number(f64),
    String(String),
}
