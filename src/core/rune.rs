use serde::{Deserialize, Serialize};
use strum::EnumIter;
use strum::FromRepr;

#[derive(
    PartialEq,
    Debug,
    Eq,
    Copy,
    Clone,
    Serialize,
    Deserialize,
    Hash,
    FromRepr,
    EnumIter,
    strum::Display,
    strum::AsRefStr,
)]
pub enum RuneType {
    Digit,
    Operator,
    Blank,
}

#[derive(
    PartialEq,
    Debug,
    Eq,
    Copy,
    Clone,
    Serialize,
    Deserialize,
    Hash,
    FromRepr,
    EnumIter,
    strum::Display,
    strum::AsRefStr,
    Default,
    PartialOrd,
    Ord,
)]
pub enum Rune {
    #[default]
    #[strum(serialize = "0")]
    Zero = 0,
    #[strum(serialize = "1")]
    One = 1,
    #[strum(serialize = "2")]
    Two = 2,
    #[strum(serialize = "3")]
    Three = 3,
    #[strum(serialize = "4")]
    Four = 4,
    #[strum(serialize = "5")]
    Five = 5,
    #[strum(serialize = "6")]
    Six = 6,
    #[strum(serialize = "7")]
    Seven = 7,
    #[strum(serialize = "8")]
    Eight = 8,
    #[strum(serialize = "9")]
    Nine = 9,

    #[strum(serialize = "+")]
    Plus = 16,
    #[strum(serialize = "⨉")]
    Times = 17,
    #[strum(serialize = "-")]
    Minus = 18,
    #[strum(serialize = "/")]
    Divide = 19,

    #[strum(serialize = "_")]
    Blank = 255,
}

static_assertions::assert_eq_size!(Rune, u8);

impl TryFrom<usize> for Rune {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value < 10 {
            Ok(Rune::from_repr(value).unwrap())
        } else {
            Err(())
        }
    }
}

impl TryFrom<char> for Rune {
    type Error = ();

    fn try_from(rune: char) -> Result<Self, Self::Error> {
        let d = rune
            .to_digit(10)
            .and_then(|x| Self::try_from(x as usize).ok());
        if d.is_some() {
            return d.ok_or(());
        }

        match rune {
            '-' => Ok(Rune::Minus),
            '⨉' => Ok(Rune::Times),
            '+' => Ok(Rune::Plus),
            '*' => Ok(Rune::Times),
            '/' => Ok(Rune::Divide),
            '_' => Ok(Rune::Blank),
            _ => Err(()),
        }
    }
}

impl TryInto<i32> for &Rune {
    type Error = ();

    fn try_into(self) -> Result<i32, Self::Error> {
        match self {
            Rune::Zero => Ok(0),
            Rune::One => Ok(1),
            Rune::Two => Ok(2),
            Rune::Three => Ok(3),
            Rune::Four => Ok(4),
            Rune::Five => Ok(5),
            Rune::Six => Ok(6),
            Rune::Seven => Ok(7),
            Rune::Eight => Ok(8),
            Rune::Nine => Ok(9),
            _ => Err(()),
        }
    }
}

impl From<Rune> for RuneType {
    fn from(val: Rune) -> Self {
        use RuneType::*;

        match val {
            Rune::Zero => Digit,
            Rune::One => Digit,
            Rune::Two => Digit,
            Rune::Three => Digit,
            Rune::Four => Digit,
            Rune::Five => Digit,
            Rune::Six => Digit,
            Rune::Seven => Digit,
            Rune::Eight => Digit,
            Rune::Nine => Digit,
            Rune::Plus => Operator,
            Rune::Times => Operator,
            Rune::Minus => Operator,
            Rune::Divide => Operator,
            Rune::Blank => Blank,
        }
    }
}
