use nom::double_s;

use super::super::ast::Expression;

named!(
    number<&str, Expression>,
    map!(ws!(double_s), { |n: f64| Expression::Number(n) })
);
