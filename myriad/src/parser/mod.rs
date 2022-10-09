use crate::{prelude::Rune, rune::RomanNumeral};
use std::iter::Peekable;

use super::rune::RuneType;

type R = Result<i32, ParseFail>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ParseFail {
    PartialSuccess,
    Failure,
}

fn parse<J: Iterator<Item = Rune>>(input: &mut Peekable<J>) -> R {
    parse_math_expr(input)
}

fn parse_math_expr<J: Iterator<Item = Rune>>(input: &mut Peekable<J>) -> R {
    //Plus and Minus
    let num1 = parse_unary(input)?;

    let mut current = num1;
    loop {
        if let Some(RuneType::Operator) = input.peek().map(|x| -> RuneType { RuneType::from(*x) }) {
            match input.peek().unwrap() {
                Rune::Plus => {
                    input.next();
                    let other = parse_unary(input)?;
                    current += other;
                }
                Rune::Times => {
                    input.next();
                    let other = parse_unary(input)?;
                    current *= other;
                }

                Rune::Minus => {
                    input.next();
                    let other = parse_unary(input)?;
                    current -= other;
                }
                Rune::Divide => {
                    input.next();
                    let other = parse_unary(input)?;

                    if other == 0 {
                        if input.peek().is_some() {
                            return Err(ParseFail::Failure);
                        } else {
                            return Err(ParseFail::PartialSuccess);
                        }
                    }
                    if current % other != 0 {
                        if input.peek().is_some() {
                            return Err(ParseFail::Failure);
                        } else {
                            return Err(ParseFail::PartialSuccess);
                        }
                    }

                    current /= other;
                }
                _ => unreachable!(),
            }
        } else {
            return Ok(current);
        }
    }
}

fn parse_number<J: Iterator<Item = Rune>>(input: &mut Peekable<J>) -> R {
    let mut current = 0i32;
    while let Some(v) = input
        .peek()
        .and_then(|x| -> Option<i32> { x.try_into().ok() })
    {
        current *= 10; //Need to use i32 here to prevent overflow

        current += v;
        input.next();
    }

    Ok(current)
}

fn parse_roman_numeral<J: Iterator<Item = Rune>>(input: &mut Peekable<J>) -> R {
    let mut current: usize = 0;
    while let Some(n) = input.peek().and_then(|x| RomanNumeral::try_from(*x).ok()) {
        input.next();
        if let Some(combination) = n.try_suffix(&current) {
            current = combination;
        } else {
            return Err(ParseFail::Failure);
        }
    }
    Ok(current as i32)
}

fn parse_unary<J: Iterator<Item = Rune>>(input: &mut Peekable<J>) -> R {
    let mut negative = false;
    loop {
        if let Some(l) = input.peek() {
            match RuneType::from(*l) {
                RuneType::Operator => {
                    if l == &Rune::Minus {
                        negative = !negative;
                        input.next();
                    } else if l == &Rune::Plus {
                        input.next();
                    } else {
                        return Err(ParseFail::Failure);
                    }
                }
                RuneType::RomanNumeral => {
                    return parse_roman_numeral(input).map(|i| if negative { -i } else { i })
                }
                RuneType::Digit => {
                    return parse_number(input).map(|i| if negative { -i } else { i })
                }
                RuneType::Blank => return Err(ParseFail::Failure),
            }
        } else {
            return Err(ParseFail::PartialSuccess);
        }
    }
}

pub fn parse_and_evaluate<J: Iterator<Item = Rune>>(
    input: &mut Peekable<J>,
) -> Result<i32, ParseFail> {
    if let Some(Rune::Plus) = input.peek() {
        return Err(ParseFail::Failure);
    }

    match parse(input) {
        Ok(expr) => match input.peek() {
            Some(l) => {
                if l == &Rune::Blank {
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
    use crate::parser::ParseFail::*;
    use crate::parser::*;
    use ntest::test_case;

    #[test_case("42", 42)]
    #[test_case("12+34", 46)]
    #[test_case("4*5", 20)]
    #[test_case("4*5+6", 26)]
    #[test_case("4/2", 2)]
    #[test_case("-2+3", 1)]
    #[test_case("--1", 1)]
    #[test_case("8-+7", 1)]
    #[test_case("i", 1)]
    #[test_case("ii", 2)]
    #[test_case("iii", 3)]
    #[test_case("iv", 4)]
    #[test_case("v", 5)]
    #[test_case("vi", 6)]
    #[test_case("vii", 7)]
    #[test_case("viii", 8)]
    #[test_case("ix", 9)]
    #[test_case("x", 10)]
    #[test_case("xi", 11)]
    #[test_case("xii", 12)]
    #[test_case("xiii", 13)]
    #[test_case("xiv", 14)]
    #[test_case("xv", 15)]
    #[test_case("xvi", 16)]
    #[test_case("xx", 20)]
    #[test_case("xxx", 30)]
    #[test_case("xl", 40)]
    #[test_case("l", 50)]
    #[test_case("lx", 60)]
    #[test_case("lxx", 70)]
    #[test_case("lxxx", 80)]
    #[test_case("xc", 90)]
    #[test_case("c", 100)]
    #[test_case("cc", 200)]
    #[test_case("ii*x", 20)]
    #[test_case("v-i", 4)]
    fn test_parse_success(input: &str, expected: i32) {
        let result = run(input);
        assert_eq!(result, Ok(expected))
    }

    #[test_case("12-34+15-9", 16)]
    #[test_case("18*-1", 18)]
    #[test_case("1*-2", 2)]
    #[test_case("12-34", 22)]
    fn test_parse_success_negative(input: &str, expected: i32) {
        let result = run(input);
        assert_eq!(result, Ok(-expected))
    }

    #[test_case("")]
    #[test_case("5/2")]
    #[test_case("5/0")]
    #[test_case("18*")]
    #[test_case("-")]
    #[test_case("--")]
    #[test_case("1+-")]
    #[test_case("1-")]
    #[test_case("1--")]
    #[test_case("1*-")]
    #[test_case("8-+")]
    #[test_case("12/")]
    #[test_case("12/5")]
    fn test_partial_success(input: &str) {
        let result = run(input);
        assert_eq!(result, Err(PartialSuccess))
    }

    #[test_case("1+*")]
    #[test_case("+*")]
    #[test_case("+")]
    #[test_case("+1")]
    #[test_case("*")]
    #[test_case("*1")]
    #[test_case("12/5+")]
    #[test_case("_")]
    #[test_case("1_")]
    #[test_case("_1")]
    #[test_case("iiii")]
    #[test_case("iix")]
    #[test_case("xxc")]
    fn test_failure(input: &str) {
        let result = run(input);
        assert_eq!(result, Err(Failure))
    }

    fn run(input: &str) -> Result<i32, ParseFail> {
        let runes: Result<Vec<Rune>, _> = input.chars().map(Rune::try_from).collect();
        parse_and_evaluate(&mut runes.unwrap().into_iter().peekable())
    }
}
