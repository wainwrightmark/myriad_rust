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

impl ChosenPositionsState {
    pub fn next(
        state: std::rc::Rc<ChosenPositionsState>,
        allow_abandon: bool,
        coordinate: Coordinate,
    ) -> std::rc::Rc<ChosenPositionsState> {
        if let Some(last) = state.positions.last() {
            if last == &coordinate {
                //Abandon word - empty state

                if !allow_abandon{
                    return state;
                }

                //log::debug!("Abandon word");
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

            //log::debug!("Retrace word");

            return ChosenPositionsState {
                positions: new_chosen_positions,
            }
            .into(); //Retrace move
        }

        if state.positions.is_empty() || state.positions.last().unwrap().is_adjacent(coordinate) {
            let mut new_chosen_positions = state.positions.clone();
            new_chosen_positions.push(coordinate);


            //log::debug!("Found word");

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

#[derive(PartialEq, Clone, Default, Serialize, Deserialize, Store)]
pub  struct InputState{
    is_down: bool
}

impl Reducer<InputState> for InputMsg{
    fn apply(&self, state: std::rc::Rc<InputState>) -> std::rc::Rc<InputState> {
        match self {
            InputMsg::Down { coordinate } => {

                //log::debug!("Input down {}", coordinate);
                Dispatch::new().apply(OnClickMsg{coordinate: coordinate.clone(), allow_abandon: true});

                InputState{is_down: true}.into()
            },
            InputMsg::Up{} => {
                //log::debug!("Input up");

                InputState{is_down: false}.into()
            },
            InputMsg::Enter { coordinate } => {                
                if state.is_down{
                    //log::debug!("Input Enter {}", coordinate);
                    Dispatch::new().apply(OnClickMsg{coordinate: coordinate.clone(), allow_abandon: false})
                }

                

                state
            },
        }
    }
}

pub enum InputMsg {
    Down { coordinate: Coordinate },
    Up { },
    Enter { coordinate: Coordinate },
}

pub struct OnClickMsg {
    pub coordinate: Coordinate,
    pub allow_abandon : bool
}

impl Reducer<ChosenPositionsState> for OnClickMsg {
    fn apply(&self, state: std::rc::Rc<ChosenPositionsState>) -> std::rc::Rc<ChosenPositionsState> {
        ChosenPositionsState::next(state, self.allow_abandon, self.coordinate)
    }
}
