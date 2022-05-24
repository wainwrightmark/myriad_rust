use std::iter::Peekable;

use num::ToPrimitive;

use crate::core::prelude::Letter;
use crate::core::prelude::Operation;

type R = Result<i32, ParseFail>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ParseFail {
    PartialSuccess,
    Failure,
}

fn parse<J: Iterator<Item = Letter>>(input: &mut Peekable<J>) -> R {
    parse_math_expr(input)
}

fn parse_math_expr<J: Iterator<Item = Letter>>(input: &mut Peekable<J>) -> R {
    //Plus and Minus
    let num1 = parse_term(input)?;

    let mut current = num1;
    loop {
        if let Some(Letter::Operator { operation }) = input.peek() {
            match operation {
                Operation::Plus => {
                    input.next();
                    let other = parse_term(input)?;
                    current += other;
                }
                Operation::Times => panic!("Shoould not encounter times in top level math expr"),

                Operation::Minus => {
                    input.next();
                    let other = parse_term(input)?;
                    current -= other;
                }
                Operation::Divide => panic!("Shoould not encounter divide in top level math expr"),
            }
        } else {
            return Ok(current);
        }
    }
}

fn parse_term<J: Iterator<Item = Letter>>(input: &mut Peekable<J>) -> R {
    //Times and Divide
    let num1 = parse_unary(input)?;

    let mut current = num1;
    loop {
        if let Some(Letter::Operator { operation }) = input.peek() {
            match operation {
                Operation::Plus => return Ok(current),
                Operation::Times => {
                    input.next();
                    let multiplicant = parse_unary(input)?;
                    current *= multiplicant;
                }
                Operation::Minus => return Ok(current),
                Operation::Divide => {
                    input.next();
                    let denominator = parse_unary(input)?;
                    if denominator == 0 {

                        if input.peek().is_some()
                        {
                            return Err(ParseFail::Failure);
                        }
                        else {
                            return Err(ParseFail::PartialSuccess);    
                        }
                        
                    }
                    if current % denominator != 0 {
                        if input.peek().is_some()
                        {
                            return Err(ParseFail::Failure);
                        }
                        else {
                            return Err(ParseFail::PartialSuccess);    
                        }
                    }

                    current /= denominator;
                }
            }
        } else {
            return Ok(current);
        }
    }
}

fn parse_number<J: Iterator<Item = Letter>>(input: &mut Peekable<J>) -> R {
    let mut current = 0u32;
    while let Some(Letter::Number { value }) = input.peek() {
        current *= 10;
        current += value;
        input.next();
    }

    Ok(current.to_i32().unwrap())
}

fn parse_unary<J: Iterator<Item = Letter>>(input: &mut Peekable<J>) -> R {
    let mut negative = false;
    loop {
        if let Some(l) = input.peek() {
            match l {
                Letter::Number { value: _ } => {
                    return parse_number(input).map(|i| if negative { -i } else { i })
                }
                Letter::Operator { operation } => match operation {
                    Operation::Plus => {
                        input.next();
                    }
                    Operation::Times => return Err(ParseFail::Failure),
                    Operation::Minus => {
                        negative = !negative;
                        input.next();
                    }
                    Operation::Divide => return Err(ParseFail::Failure),
                },
                Letter::Blank => return Err(ParseFail::Failure),
            }
        } else {
            return Err(ParseFail::PartialSuccess);
        }
    }
}

pub(crate) fn parse_and_evaluate<J: Iterator<Item = Letter>>(
    input: &mut Peekable<J>,
) -> Result<i32, ParseFail> {
    if let Some(Letter::Operator {
        operation: Operation::Plus,
    }) = input.peek()
    {
        return Err(ParseFail::Failure);
    }

    match parse(input) {
        Ok(expr) => match input.peek() {
            Some(l) => {
                if l == &Letter::Blank {
                    Err(ParseFail::Failure)
                } else {
                    Err(ParseFail::PartialSuccess)
                }
            }
            None => Ok(expr),
        },
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use crate::core::parser::ParseFail::*;
    use crate::core::parser::*;

    macro_rules! parse_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (input, expected) = $value;

            let letters: Option<Vec<Letter>> = input.chars().map(Letter::try_create).collect();

            assert_eq!(expected, parse_and_evaluate(&mut letters.unwrap().into_iter().peekable()), "'{}'", input);
        }
    )*
    }
}

    parse_tests! {
        t0: ("", Err(PartialSuccess)),
        t1: ("12", Ok(12)),

        t2: ("12+34", Ok(46)),
        t3: ("12-34", Ok(-22)),
        t4: ("12-34+15-9", Ok(-16)),
        t5: ("4*5", Ok(20)),
        t6: ("4*5+6", Ok(26)),
        t7: ("4/2", Ok(2)),
        t8: ("5/2", Err(PartialSuccess)),
        t9: ("5/0", Err(PartialSuccess)),
        t10: ("18-2*3", Ok(12)),
        t11: ("18*-1", Ok(-18)),
        t12: ("-2+3", Ok(1)),
        t13: ("18*", Err(PartialSuccess)),
        t14: ("-12", Ok(-12)),
        t15: ("-", Err(PartialSuccess)),
        t16: ("--", Err(PartialSuccess)),
        t17: ("--1", Ok(1)),
        t18:("1+-", Err(PartialSuccess)),
        t19:("1+*", Err(Failure)),
        t20:("+*", Err(Failure)),
        t21:("1-", Err(PartialSuccess)),
        t22:("1--", Err(PartialSuccess)),
        t23:("1*-", Err(PartialSuccess)),
        t24:("1*-2", Ok(-2)),
        t25:("8-+7", Ok(1)),
        t26:("8-+", Err(PartialSuccess)),
        t27:("+", Err(Failure)),
        t28:("+1", Err(Failure)),
        t29:("*", Err(Failure)),
        t30:("*1", Err(Failure)),

        t31:("_", Err(Failure)),
        t32:("1_", Err(Failure)),
        t33:("_1", Err(Failure)),
        t34:("12/", Err(PartialSuccess)),
        t35:("12/5", Err(PartialSuccess)), //This is legal as maybe the next digit makes division possible
        t36:("12/5+", Err(Failure)), //This is not legal as it is impossible for the division to work
    }
}
