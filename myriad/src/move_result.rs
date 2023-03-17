use geometrid::prelude::Tile;
use tinyvec::ArrayVec;

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
        coordinates: ArrayVec<[Tile<C, R>;9]>,
    },
    WordAbandoned,
    MoveRetraced {
        word: String,
        coordinates: ArrayVec<[Tile<C, R>;9]>,
    },
    IllegalMove,
}

impl<const C: u8, const R: u8> MoveResult<C, R> {
    pub fn is_legal(&self) -> bool {
        !matches!(self, MoveResult::IllegalMove)
    }
}
