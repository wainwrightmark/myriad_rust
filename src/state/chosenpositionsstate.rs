use crate::core::prelude::*;
use crate::state::prelude::*;
use itertools::Itertools;
use serde::*;
use yewdux::prelude::*;

#[derive(PartialEq, Clone, Default, Serialize, Deserialize, Store)]
pub struct ChosenPositionsState {
    pub positions: Vec<Coordinate>,
}

impl ChosenPositionsState {
    pub fn after_move_result(self, move_result: &MoveResult) -> Self {
        match move_result {
            MoveResult::WordComplete { word } => Self {
                positions: word.path.to_owned(),
            },
            MoveResult::WordOutsideRange { word } => Self {
                positions: word.path.to_owned(),
            },
            MoveResult::WordIncomplete {
                word: _,
                coordinates,
            } => Self {
                positions: coordinates.to_owned(),
                ..self
            },
            MoveResult::WordAbandoned => Self {
                positions: Default::default(),
            },
            MoveResult::MoveRetraced {
                word: _,
                coordinates,
            } => Self {
                positions: coordinates.clone(),
            },
            MoveResult::IllegalMove => self,
        }
    }
}

pub struct OnClickMsg {
    pub coordinate: Coordinate,
}

pub struct FindNumberMsg {
    pub number: i32,
}

impl Reducer<ChosenPositionsState> for FindNumberMsg {
    fn apply(&self, state: std::rc::Rc<ChosenPositionsState>) -> std::rc::Rc<ChosenPositionsState> {
        let fs = Dispatch::<FullState>::new().get();

        if let Some(path) = fs.found_words.words.get(&self.number) {
            ChosenPositionsState {
                positions: path.path.clone(),
            }
            .into()
        } else {
            state
        }
    }
}

impl Reducer<ChosenPositionsState> for OnClickMsg {
    fn apply(&self, state: std::rc::Rc<ChosenPositionsState>) -> std::rc::Rc<ChosenPositionsState> {
        let coordinate = self.coordinate;

        if let Some(last) = state.positions.last() {
            if last == &coordinate {
                //Abandon word - empty state
                Dispatch::new().apply(ClearExpiredWordsMsg {});

                return ChosenPositionsState::default().into();
            }
        }

        let find_result = state.positions.iter().find_position(|&z| z == &coordinate);

        if let Some((index, _)) = find_result {
            let new_chosen_positions: Vec<Coordinate> = state
                .positions
                .iter()
                .take(index + 1)
                .copied()
                .collect_vec();

            //TOOD maybe send a find message here

            return ChosenPositionsState {
                positions: new_chosen_positions,
            }
            .into(); //Retrace move
        }

        if state.positions.is_empty() || state.positions.last().unwrap().is_adjacent(&coordinate) {
            let mut new_chosen_positions = state.positions.clone();
            new_chosen_positions.push(coordinate);

            Dispatch::new().apply(OnCoordinatesSetMsg {
                coordinates: new_chosen_positions.clone(),
            });

            return ChosenPositionsState {
                positions: new_chosen_positions,
            }
            .into(); //New move
        }
        state
    }
}
