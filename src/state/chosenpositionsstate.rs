use serde::*;
use crate::core::prelude::*;

#[derive(PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ChosenPositionsState{
    pub positions : Vec<Coordinate>
}

impl ChosenPositionsState{
    pub fn after_move_result(self, move_result: &MoveResult) -> Self {
        match move_result {
            MoveResult::WordComplete {
                word: _,
                coordinates,
            } => Self {
                positions: coordinates.to_owned(),
                ..self
            },
            MoveResult::WordContinued {
                word: _,
                coordinates,
            } => Self {
                positions: coordinates.to_owned(),
                ..self
            },
            MoveResult::WordAbandoned => Self {
                positions: Default::default(),
                ..self
            },
            MoveResult::MoveRetraced {
                word: _,
                coordinates,
            } => Self {
                positions: coordinates.clone(),
                ..self
            },
            MoveResult::IllegalMove => self,
        }
    }
}

