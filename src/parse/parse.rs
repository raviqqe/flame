use std::str::FromStr;

use nom::digit;

use super::super::ast::Expression;

named!(
    unsigned_number<&str, f64>,
    map_res!(
        alt_complete!(
            recognize!(tuple!(digit, tag!("."), digit)) |
            recognize!(digit)
        ),
        FromStr::from_str
    )
);

named!(
    number<&str, f64>,
    map!(
        pair!(opt!(tag!("-")), unsigned_number),
        |(s, n): (Option<&str>, f64)| { if s == Some("-") { -1.0 * n  } else { n }}
    )
);

named!(
    expression<&str, Expression>,
    ws!(
        alt!(
            tag!("nil") => { |_| Expression::Nil } |
            number => { |n| Expression::Number(n) }
        )
    )
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unsigned_number_parser() {
        for s in &["123", "0.1"] {
            let r = unsigned_number(s);
            println!("{:?}", r);
            assert!(r.is_done());
        }
    }

    #[test]
    fn expression_parser() {
        for s in &[
            "nil",
            "  nil\t",
            "123",
            "0.1",
            "-123",
            "-0.1",
            "  \n-0.1\t ",
        ] {
            let r = expression(s);
            println!("{:?}", r);
            assert!(r.is_done());
        }
    }
}
