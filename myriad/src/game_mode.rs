use super::prelude::Rune;
use super::prelude::Rune::*;

pub trait GameMode: Default {
    fn legal_letters(&self) -> &'static [Rune];
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ClassicGameMode {}

impl GameMode for ClassicGameMode {
    fn legal_letters(&self) -> &'static [Rune] {
        &[
            One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Plus, Times, Minus, Divide,
        ]
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct CenturyGameMode {}

impl GameMode for CenturyGameMode {
    fn legal_letters(&self) -> &'static [Rune] {
        &[
            RomanOne,
            RomanFive,
            RomanTen,
            RomanFifty,
            RomanOneHundred,
            Plus,
            Times,
            Minus,
            Divide,
        ]
    }
}
