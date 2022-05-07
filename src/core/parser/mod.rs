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

//TODO rewrite to use a token stream instead of a string

#[derive(Debug, PartialEq)]
pub enum Expr {
    Empty,
    ENum(i32),
    Minus(Box<Expr>),
    EAdd(Box<Expr>, Box<Expr>),
    ESub(Box<Expr>, Box<Expr>),
    EMul(Box<Expr>, Box<Expr>),
    EDiv(Box<Expr>, Box<Expr>),
}

fn parse(input: &str) -> IResult<&str, Expr> {
    parse_math_expr(input)
}

fn parse_math_expr(input: &str) -> IResult<&str, Expr> {
    let (input, num1) = parse_term(input)?;
    let (input, exprs) = many0(tuple((alt((char('+'), char('-'))), parse_term)))(input)?;
    Ok((input, parse_expr(num1, exprs)))
}

fn parse_term(input: &str) -> IResult<&str, Expr> {
    let (input, num1) = parse_unary(input)?;
    let (input, exprs) = many0(tuple((alt((char('/'), char('*'))), parse_term)))(input)?;
    Ok((input, parse_expr(num1, exprs)))
}

fn parse_expr(expr: Expr, rem: Vec<(char, Expr)>) -> Expr {
    rem.into_iter().fold(expr, |acc, val| parse_op(val, acc))
}

fn parse_number(input: &str) -> IResult<&str, Expr> {
    if input.is_empty(){
        Ok(("", Expr::Empty))
    }else {
        map(nom::character::complete::i32, Expr::ENum)(input)    
    }

    
}

fn parse_unary(input: &str) -> IResult<&str, Expr>{
 alt((parse_number,parse_minus))(input)
}

fn parse_minus(input: &str) -> IResult<&str, Expr>{
    let (rem, _) = char('-')(input)?;
    let (rem2, e) = parse_unary(rem)?;
    Ok((rem2, Expr::Minus(Box::new(e))))
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




fn evaluate(expr: Expr) -> Option<i32> {
    match expr {
        Empty => None,
        ENum(num) => Some(num),
        Minus(e) => evaluate(*e).map(|x| -x),
        EAdd(expr1, expr2) => {
            evaluate(*expr1).and_then(|x| evaluate(*expr2).map(|y| x + y))
        }
        ESub(expr1, expr2) => {
            evaluate(*expr1).and_then(|x| evaluate(*expr2).map(|y| x - y))
        }
        EMul(expr1, expr2) => {
            evaluate(*expr1).and_then(|x| evaluate(*expr2).map(|y| x * y))
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
    match parse(input) {
        Ok((rem, expr)) => if rem.is_empty() {
            if let Some(i) = evaluate(expr) {
                ParseOutcome::Success(i)
            } else {
                ParseOutcome::PartialSuccess
            }
        } else {

            ParseOutcome::Failure

            
        },
        Err(_) => ParseOutcome::Failure
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
            assert_eq!(expected, parse_and_evaluate(input), "'{}'", input);
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
        t15: ("-", PartialSuccess),
        t16: ("--", PartialSuccess),
        t17: ("--1", Success(1)),
        t18:("1+-", PartialSuccess),
        t19:("1+*", Failure),
        t20:("+*", Failure),
        t21:("1-", PartialSuccess),
        t22:("1--", PartialSuccess),
        t23:("1*-", PartialSuccess),
        t24:("1*-2", Success(-2)),
    }
}
