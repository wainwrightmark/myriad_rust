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

fn evaluate(expr: Expr) -> i32 {
    match expr {
        ENum(num) => num,
        EAdd(expr1, expr2) => evaluate(*expr1) + evaluate(*expr2),
        ESub(expr1, expr2) => evaluate(*expr1) - evaluate(*expr2),
        EMul(expr1, expr2) => evaluate(*expr1) * evaluate(*expr2),
        EDiv(expr1, expr2) => evaluate(*expr1) / evaluate(*expr2),
    }
}

pub (crate) fn parse_and_evaluate(input: &str) -> Result<i32, nom::Err<nom::error::Error<&str>>>{
    parse(input).map(|x|evaluate(x.1))
}

#[cfg(test)]
mod tests {
    use crate::core::parser::*;

    #[test]
    fn parse_add_statement() {
        let r = parse_and_evaluate("12+34");
        assert_eq!(
            r,
            Ok(46)
        );
    }

    #[test]
    fn parse_subtract_statement() {
        let r = parse_and_evaluate("12-34");
        assert_eq!(
            r,
            Ok(-22)
        );
    }

    #[test]
    fn parse_nested_add_sub_statements() {
        let r = parse_and_evaluate("12-34+15 - 9");
        assert_eq!(
            r,
            Ok(-16)
        );
    }
}


