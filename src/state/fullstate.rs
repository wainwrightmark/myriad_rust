use crate::core::parser::ParseFail;
use crate::core::prelude::*;
use crate::state::chosenpositionsstate::*;
use crate::state::foundwordsstate::*;
use crate::state::recentwordstate::*;
use crate::state::rotflipstate::*;
use crate::state::selectedtabstate::*;
use itertools::Itertools;
use serde::*;
use std::rc::Rc;
use yewdux::prelude::*;

#[derive(PartialEq, Store, Clone, Default, Serialize, Deserialize)]
#[store(storage = "local")] // can also be "session"
pub struct FullState {
    pub board: Rc<Board>,
    #[serde(skip)]
    pub chosen_positions: ChosenPositionsState,
    pub found_words: Rc<FoundWordsState>,
    pub solver: Solver,
    #[serde(skip)]
    pub rotflip: RotFlipState,

    #[serde(skip)]
    pub recent_words: Rc<RecentWordState>,

    #[serde(skip)]
    pub selected_tab_state: SelectedTabState,
}

impl FullState {
    pub fn get_color(&self, coordinate: &Coordinate) -> (String, String) {
        let move_result = self.get_move_result(coordinate);

        match move_result {
            MoveResult::WordComplete {
                word: _,
            } => ("darkgreen".to_string(), "pointer".to_string()),
            MoveResult::WordIncomplete {
                word: _,
                coordinates: _,
            } => ("green".to_string(), "pointer".to_string()),
            MoveResult::WordOutsideRange {
                word: _,
            } => ("green".to_string(), "pointer".to_string()),
            MoveResult::WordAbandoned => ("darkgreen".to_string(), "pointer".to_string()),
            MoveResult::MoveRetraced {
                word: _,
                coordinates: _,
            } => ("blue".to_string(), "pointer".to_string()),
            MoveResult::IllegalMove => ("grey".to_string(), "not-allowed".to_string()),
        }
    }

    pub fn get_move_result(&self, coordinate: &Coordinate) -> MoveResult {
        if !self.chosen_positions.positions.is_empty()
            && (self.chosen_positions.positions.first().unwrap() == coordinate
                || self.chosen_positions.positions.last().unwrap() == coordinate)
        {
            return MoveResult::WordAbandoned;
        }

        let find_result = self
            .chosen_positions
            .positions
            .iter()
            .find_position(|z| z == &coordinate);

        if let Some((index, _)) = find_result {
            let new_chosen_positions: Vec<Coordinate> = self
                .chosen_positions
                .positions
                .iter()
                .take(index + 1)
                .copied()
                .collect_vec();
            let word = self.board.get_word_text(&new_chosen_positions);
            return MoveResult::MoveRetraced {
                word,
                coordinates: new_chosen_positions,
            };
        }

        if self.chosen_positions.positions.is_empty()
            || self
                .chosen_positions
                .positions
                .last()
                .unwrap()
                .is_adjacent(coordinate)
        {
            let mut new_chosen_positions = self.chosen_positions.positions.clone();
            new_chosen_positions.push(*coordinate);

            let word = self.board.get_word_text(&new_chosen_positions);

            
            let check_result = self.board.check(&new_chosen_positions);

            let final_result = match check_result {
                Ok(i) => {
                    if self.solver.settings.allow(i) {
                        MoveResult::WordComplete {
                            word: FoundWord {
                                result: i,
                                path: new_chosen_positions,
                            }
                        }
                    } else {
                        MoveResult::WordOutsideRange {
                            word: FoundWord {
                                result: i,
                                path: new_chosen_positions,
                            }
                        }
                    }
                }
                Err(ParseFail::PartialSuccess) => MoveResult::WordIncomplete {
                    word,
                    coordinates: new_chosen_positions,
                },
                Err(ParseFail::Failure) => MoveResult::IllegalMove {},
            };

            return final_result;
        }

        MoveResult::IllegalMove {}
    }
}
