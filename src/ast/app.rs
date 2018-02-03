use super::arguments::Arguments;
use super::expression::Expression;

#[derive(Clone, Debug, PartialEq)]
pub struct App<'a> {
    function: Expression<'a>,
    arguments: Arguments<'a>,
}
