use nom::branch::alt;
use nom::character::complete::{char, digit1, space0};
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::{delimited, tuple};
use nom::IResult;
use std::str::FromStr;
use crate::core::parser::Expr::*;

#[derive(Debug, PartialEq)]
pub enum Expr {
    ENum(i32),
    EAdd(Box<Expr>, Box<Expr>),
    ESub(Box<Expr>, Box<Expr>),
    EMul(Box<Expr>, Box<Expr>),
    EDiv(Box<Expr>, Box<Expr>),
}

fn parse(input: &str) -> IResult<&str, Expr> {
    parse_basic_expr(input)
}

fn parse_basic_expr(input: &str) -> IResult<&str, Expr> {
    parse_math_expr(input)
}


fn parse_term(input: &str) -> IResult<&str, Expr> {
    let (input, num1) = parse_number(input)?;
    let (input, exprs) = many0(tuple((alt((char('/'), char('*'))), parse_term)))(input)?;
    Ok((input, parse_expr(num1, exprs)))
}

fn parse_math_expr(input: &str) -> IResult<&str, Expr> {
    let (input, num1) = parse_term(input)?;
    let (input, exprs) = many0(tuple((alt((char('+'), char('-'))), parse_term)))(input)?;
    Ok((input, parse_expr(num1, exprs)))
}

fn parse_expr(expr: Expr, rem: Vec<(char, Expr)>) -> Expr {
    rem.into_iter().fold(expr, |acc, val| parse_op(val, acc))
}

fn parse_op(tup: (char, Expr), expr1: Expr) -> Expr {
    let (op, expr2) = tup;
    match op {
        '+' => Expr::EAdd(Box::new(expr1), Box::new(expr2)),
        '-' => Expr::ESub(Box::new(expr1), Box::new(expr2)),
        '*' => Expr::EMul(Box::new(expr1), Box::new(expr2)),
        '/' => Expr::EDiv(Box::new(expr1), Box::new(expr2)),
        _ => panic!("Unknown Operation"),
    }
}

fn parse_enum(parsed_num: &str) -> Expr {
    let num = i32::from_str(parsed_num).unwrap();
    Expr::ENum(num)
}

fn parse_number(input: &str) -> IResult<&str, Expr> {
    map(delimited(space0, digit1, space0), parse_enum)(input)
}

fn evaluate(expr: Expr) -> Option<i32> {
    match expr {
        ENum(num) => Some(num),
        EAdd(expr1, expr2) => evaluate(*expr1).and_then(|x| evaluate(*expr2).and_then(|y| Some(x + y))),
        ESub(expr1, expr2) => evaluate(*expr1).and_then(|x| evaluate(*expr2).and_then(|y| Some(x - y))),
        EMul(expr1, expr2) => evaluate(*expr1).and_then(|x| evaluate(*expr2).and_then(|y| Some(x * y))),
        EDiv(expr1, expr2) => evaluate(*expr1).and_then(|x| evaluate(*expr2).and_then(|y| if y != 0 && x % y ==0 {Some(x / y)} else{None})),
    }
}

pub (crate) fn parse_and_evaluate(input: &str) -> Option<i32>{
let parse_result = parse(input);
if let Ok((_, expr)) = parse_result{
    let ev_result = evaluate(expr);
    return ev_result;
}
None
}

#[cfg(test)]
mod tests {
    use crate::core::parser::*;

    


macro_rules! fib_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (input, expected) = $value;
            assert_eq!(expected, parse_and_evaluate(input));
        }
    )*
    }
}

fib_tests! {
    t0: ("", None),
    t1: ("12", Some(12)),
    t2: ("12+34", Some(46)),
    t3: ("12-34", Some(-22)),
    t4: ("12-34+15-9", Some(-16)),
    t5: ("4*5", Some(20)),
    t6: ("4*5 + 6", Some(26)),
    t7: ("4/2", Some(2)),
    t8: ("5/2", None),
    t9: ("5/0", None),
}


}


