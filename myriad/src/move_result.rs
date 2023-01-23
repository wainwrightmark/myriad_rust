use geometrid::prelude8::PointAbsolute8;

use super::solver::FoundWord;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum MoveResult<const C: u8, const R: u8> {
    WordComplete {
        word: FoundWord<C, R>,
    },
    WordOutsideRange {
        word: FoundWord<C, R>,
    },
    WordIncomplete {
        word: String,
        coordinates: Vec<PointAbsolute8<C, R>>,
    },
    WordAbandoned,
    MoveRetraced {
        word: String,
        coordinates: Vec<PointAbsolute8<C, R>>,
    },
    IllegalMove,
}

impl<const C: u8, const R: u8> MoveResult<C, R> {
    pub fn is_legal(&self) -> bool {
        !matches!(self, MoveResult::IllegalMove)
    }
}
