use super::coordinate::Coordinate;
use super::solver::FoundWord;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum MoveResult {
    WordComplete {
        word: FoundWord,
    },
    WordOutsideRange {
        word: FoundWord,
    },
    WordIncomplete {
        word: String,
        coordinates: Vec<Coordinate>,
    },
    WordAbandoned,
    MoveRetraced {
        word: String,
        coordinates: Vec<Coordinate>,
    },
    IllegalMove,
}

impl MoveResult {
    pub fn is_legal(&self) -> bool {
        !matches!(self, MoveResult::IllegalMove)
    }
}
