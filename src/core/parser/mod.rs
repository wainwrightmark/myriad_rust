use crate::core::parser::Expr::*;
use nom::branch::alt;
use nom::character::complete::{char};
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ParseOutcome {
    Success(i32),
    PartialSuccess,
    Failure,
}

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


fn parse_number(input: &str) -> IResult<&str, Expr> {
    map(nom::character::complete::i32, |n|Expr::ENum(n))(input)
}

fn evaluate(expr: Expr) -> Option<i32> {
    match expr {
        ENum(num) => Some(num),
        EAdd(expr1, expr2) => {
            evaluate(*expr1).and_then(|x| evaluate(*expr2).and_then(|y| Some(x + y)))
        }
        ESub(expr1, expr2) => {
            evaluate(*expr1).and_then(|x| evaluate(*expr2).and_then(|y| Some(x - y)))
        }
        EMul(expr1, expr2) => {
            evaluate(*expr1).and_then(|x| evaluate(*expr2).and_then(|y| Some(x * y)))
        }
        EDiv(expr1, expr2) => evaluate(*expr1).and_then(|x| {
            evaluate(*expr2).and_then(|y| {
                if y != 0 && x % y == 0 {
                    Some(x / y)
                } else {
                    None
                }
            })
        }),
    }
}

pub(crate) fn parse_and_evaluate(input: &str) -> ParseOutcome {
    if input.is_empty() {return ParseOutcome::PartialSuccess;}
    let parse_result = parse(input);
    if let Ok((rem, expr)) = parse_result {
        if rem.is_empty() {
            if let Some(i) = evaluate(expr) {
                ParseOutcome::Success(i)
            } else {
                ParseOutcome::PartialSuccess
            }
        } else {
            ParseOutcome::PartialSuccess
        }
    } else {
        ParseOutcome::Failure
    }
}

#[cfg(test)]
mod tests {
    use crate::core::parser::ParseOutcome::*;
    use crate::core::parser::*;

    macro_rules! parse_tests {
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

    parse_tests! {
        t0: ("", PartialSuccess),
        t1: ("12", Success(12)),

        t2: ("12+34", Success(46)),
        t3: ("12-34", Success(-22)),
        t4: ("12-34+15-9", Success(-16)),
        t5: ("4*5", Success(20)),
        t6: ("4*5+6", Success(26)),
        t7: ("4/2", Success(2)),
        t8: ("5/2", PartialSuccess),
        t9: ("5/0", PartialSuccess),
        t10: ("18-2*3", Success(12)),
        t11: ("18*-1", Success(-18)),
        t12: ("-2+3", Success(1)),
        t13: ("18*", PartialSuccess),
        t14: ("-12", Success(-12)),

    }
}
