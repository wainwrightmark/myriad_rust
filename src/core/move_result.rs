use super::coordinate::Coordinate;
use super::solver::FoundWord;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum MoveResult<const C : usize, const  R: usize> {
    WordComplete {
        word: FoundWord<C, R>,
    },
    WordOutsideRange {
        word: FoundWord<C, R>,
    },
    WordIncomplete {
        word: String,
        coordinates: Vec<Coordinate<C, R>>,
    },
    WordAbandoned,
    MoveRetraced {
        word: String,
        coordinates: Vec<Coordinate<C, R>>,
    },
    IllegalMove,
}

impl<const C : usize, const  R: usize> MoveResult<C, R> {
    pub fn is_legal(&self) -> bool {
        !matches!(self, MoveResult::IllegalMove)
    }
}
