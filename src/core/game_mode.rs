use super::prelude::Rune;
use super::prelude::Rune::*;


pub trait GameMode
{
    fn legal_letters(&self)-> &'static [Rune];
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ClassicGameMode{

}

impl GameMode for ClassicGameMode{
    fn legal_letters(&self)-> &'static [Rune] {
        &[
            One,Two,Three,Four,Five,Six,Seven,Eight,Nine,Plus,Times,Minus,Divide
            ]
    }
}