use super::coordinate::Coordinate;
use super::solver::FoundWord;

#[derive(PartialEq,  Clone, Debug)]
pub enum MoveResult{
    WordComplete {word: FoundWord, coordinates: Vec<Coordinate>},
    WordContinued {word: String, coordinates: Vec<Coordinate>},
    WordAbandoned,
    MoveRetraced{word: String, coordinates: Vec<Coordinate>},
    IllegalMove
}

impl MoveResult {
    pub fn is_legal(&self)->bool{
        ! matches!(self, MoveResult::IllegalMove)
    }
}
