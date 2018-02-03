use super::app::App;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression<'a> {
    App(&'a App<'a>),
    Name(&'a str),
    Nil,
    Number(f64),
}
