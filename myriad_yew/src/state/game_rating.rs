use myriad::prelude::FoundWord;
use std::collections::BTreeMap;

use super::{full_game_state::*, prelude::*};

#[derive(Debug, PartialEq, Eq)]
pub struct GameRating {
    pub min_steps: u32,
    pub actual_steps: u32,
    pub worst_word: Option<SuboptimalWord>,
    pub hardest_word: Option<FoundWord<GRID_COLUMNS, GRID_ROWS, GRID_SIZE>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SuboptimalWord {
    pub best: FoundWord<GRID_COLUMNS, GRID_ROWS, GRID_SIZE>,
    pub actual: FoundWord<GRID_COLUMNS, GRID_ROWS, GRID_SIZE>,
}

impl SuboptimalWord {
    pub fn extra_length(&self) -> usize {
        self.actual.path.len().saturating_sub(self.best.path.len())
    }

    pub fn result(&self) -> i32 {
        self.actual.result
    }
}

impl GameRating {
    pub fn create(state: &FullGameState) -> Self {
        let mut map: BTreeMap<i32, FoundWord<GRID_COLUMNS, GRID_ROWS, GRID_SIZE>> =
            Default::default();

        let solutions = state.game.solve_settings.solve(state.game.board.clone());

        for s in solutions {
            map.insert(s.result, s);
        }

        let mut result = Self {
            min_steps: 0,
            actual_steps: 0,
            worst_word: None,
            hardest_word: None,
        };

        for (number, actual) in state.found_words.words.iter() {
            let Some(best) = map.get(number) else {continue;};

            result.min_steps += best.path.len() as u32;
            result.actual_steps += actual.path.len() as u32;

            if let Some(extra_length) = actual.path.len().checked_sub(best.path.len()) {
                if extra_length > 0 {
                    if let Some(current_worst) = &result.worst_word {
                        if current_worst.extra_length() < extra_length {
                            result.worst_word = Some(SuboptimalWord {
                                best: best.clone(),
                                actual: actual.clone(),
                            })
                        }
                    } else {
                        result.worst_word = Some(SuboptimalWord {
                            best: best.clone(),
                            actual: actual.clone(),
                        })
                    }
                }
            }

            if let Some(hardest) = &result.hardest_word{
                if actual.path.len() > hardest.path.len(){
                    result.hardest_word = Some(actual.clone());
                }
            }else{
                result.hardest_word = Some(actual.clone());
            }
        }

        result
    }
}
