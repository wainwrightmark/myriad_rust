use crate::core::parser::ParseOutcome::*;
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

fn parse(input: &str) -> IResult<&str, ParseOutcome> {
    parse_math_expr(input)
}

fn parse_math_expr(input: &str) -> IResult<&str, ParseOutcome> {
    let (input, num1) = parse_term(input)?;
    let (input, exprs) = many0(tuple((alt((char('+'), char('-'))), parse_term)))(input)?;
    Ok((input, parse_expr(num1, exprs)))
}

fn parse_term(input: &str) -> IResult<&str, ParseOutcome> {
    let (input, num1) = parse_unary(input)?;
    let (input, exprs) = many0(tuple((alt((char('/'), char('*'))), parse_term)))(input)?;
    Ok((input, parse_expr(num1, exprs)))
}

fn parse_expr(expr: ParseOutcome, rem: Vec<(char, ParseOutcome)>) -> ParseOutcome {
    rem.into_iter().fold(expr, |acc, val| parse_op(val, acc))
}

fn parse_number(input: &str) -> IResult<&str, ParseOutcome> {
    if input.is_empty(){
        Ok(("", PartialSuccess))
    }else {
        map(nom::character::complete::i32, |x|ParseOutcome::Success(x))(input)    
    }

    
}

fn parse_unary(input: &str) -> IResult<&str, ParseOutcome>{
 alt((parse_number,parse_minus,parse_plus))(input)
}

fn parse_minus(input: &str) -> IResult<&str, ParseOutcome>{
    let (rem, _) = char('-')(input)?;
    let (rem2, e) = parse_unary(rem)?;
    
    if let Success(i) = e{
        Ok((rem2, Success(-i)))
    }
    else{
        Ok((rem2, e))
    }
}

fn parse_plus(input: &str) -> IResult<&str, ParseOutcome>{
    let (rem, _) = char('+')(input)?;
    parse_unary(rem)
}

fn parse_op(tup: (char, ParseOutcome), expr1: ParseOutcome) -> ParseOutcome {
    let (op, expr2) = tup;

    if let Success(i1) = expr1{
        if let Success(i2) = expr2{
            match op {
                '+' => Success(i1 + i2),
                '-' => Success(i1 - i2),
                '*' => Success(i1 * i2),
                '/' => {
                    if i2 == 0 {  ParseOutcome::PartialSuccess}
                    else if i1 % i2 != 0 {  ParseOutcome::PartialSuccess}
                    else{ Success(i1 / i2)}
                }
                _ => panic!("Unknown Operation"),
            }
        }else {
            expr2
        }
    }
    else {expr1}    
}

pub(crate) fn parse_and_evaluate(input: &str) -> ParseOutcome {    
    if input == "+" {return ParseOutcome::Failure;}
    match parse(input) {
        Ok((rem, expr)) =>
         if rem.is_empty() { expr
        } else {
            ParseOutcome::Failure            
        },
        Err(_) => ParseOutcome::Failure
    }

}

#[cfg(test)]
mod tests {
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
        t25:("8-+7", Success(1)),
        t26:("8-+", PartialSuccess),
    }
}
