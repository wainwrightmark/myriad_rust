use crate::core::prelude::Rune;
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

pub(crate) fn parse_and_evaluate<J: Iterator<Item = Rune>>(
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
    use crate::core::parser::ParseFail::*;
    use crate::core::parser::*;
    use ntest::test_case;
    

    macro_rules! parse_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (input, expected) = $value;

            let runes: Result<Vec<Rune>, _> = input.chars().map(Rune::try_from).collect();

            assert_eq!(expected, parse_and_evaluate(&mut runes.unwrap().into_iter().peekable()), "'{}'", input);
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
        //t9: ("5/0", Err(PartialSuccess)),
        t10: ("18-2*3", Ok(48)), //would be 12 with BODMAS
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
        y37:("12+8/4", Ok(5)), //14 with BODMAS
    }
}
