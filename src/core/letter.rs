use serde::{Deserialize, Serialize};
use strum::FromRepr;
use strum::{EnumIter, IntoEnumIterator};


#[derive(PartialEq, Debug, Eq, Copy, Clone, Serialize, Deserialize, Hash)]

pub enum Letter {
    Number { value: Digit },
    Operator { operation: Operation },
    Blank,
}

impl Letter {
    pub fn try_create(rune: char) -> Option<Letter> {
        let d = rune.to_digit(10).and_then(|x| Digit::from_repr(x as usize)) .map(|value| Letter::Number { value});
        if d != None {
            return d;
        }

        match rune {
            '-' => Some(Letter::Operator {
                operation: Operation::Minus,
            }),
            '⨉' => Some(Letter::Operator {
                operation: Operation::Plus,
            }),
            '+' => Some(Letter::Operator {
                operation: Operation::Plus,
            }),
            '*' => Some(Letter::Operator {
                operation: Operation::Times,
            }),
            '/' => Some(Letter::Operator {
                operation: Operation::Divide,
            }),
            '_' => Some(Letter::Blank),
            _ => None,
        }
    }

    pub fn legal_letters() -> impl Iterator<Item = Letter> {
        let nums = Digit::iter().map(|value| Letter::Number { value});
        let ops = [
            Letter::Operator {
                operation: Operation::Plus,
            },
            Letter::Operator {
                operation: Operation::Minus,
            },
            Letter::Operator {
                operation: Operation::Times,
            },
            Letter::Operator {
                operation: Operation::Divide,
            },
        ];
        nums.chain(ops.into_iter())
    }

    pub fn word_text(&self) -> String {
        match self {
            Letter::Number { value } => value.to_string(),
            Letter::Operator { operation } => operation.to_string(),
            Letter::Blank => "_".to_string(),
        }
    }
}

impl std::fmt::Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = &self.word_text();
        write!(f, "{}", r)
    }
}

#[derive(PartialEq, Debug, Eq, Copy, Clone, Serialize, Deserialize, Hash, FromRepr, EnumIter)]
pub enum Digit{ //TODO Zero
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9
}

impl Into<i32> for &Digit{
    fn into(self) -> i32 {
        (*self as u8) as i32
    }
}

impl std::fmt::Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = *self as u8;
        write!(f, "{}", r)
    }
}

#[derive(PartialEq, Debug, Eq, Copy, Clone, Serialize, Deserialize, Hash)]
pub enum Operation {
    Plus = 10,
    Times,
    Minus,
    Divide,
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = match self {
            Operation::Plus => "+",
            Operation::Times => "⨉",
            Operation::Minus => "-",
            Operation::Divide => "/",
        };
        write!(f, "{}", r)
    }
}
