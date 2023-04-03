use crate::state::prelude::*;
use itertools::Itertools;
use myriad::parser::parse_and_evaluate;
use myriad::prelude::*;
use serde::*;
use yewdux::prelude::*;

use super::info_bar_state::{InfoBarSetMessage, InfoBarState};

#[derive(PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
pub struct ChosenPositionsState {
    pub positions: ArrayVec<[Tile<GRID_COLUMNS, GRID_ROWS>; GRID_SIZE]>,
}

impl Store for ChosenPositionsState {
    fn new() -> Self {
        init_listener(CPSListener);
        Default::default()
    }

    fn should_notify(&self, old: &Self) -> bool {
        self != old
    }
}

#[derive(Debug, Default)]
pub struct CPSListener;

impl Listener for CPSListener {
    type Store = ChosenPositionsState;

    fn on_change(&mut self, state: std::rc::Rc<Self::Store>) {
        let new_chosen_positions = state.positions;

        let board = Dispatch::<FullGameState>::new().get().game.board.clone();

        let mut letters = new_chosen_positions.iter().map(|c| board[*c]).peekable();

        let parse_result = parse_and_evaluate(&mut letters);

        let infobar_state: InfoBarState;

        match parse_result {
            Ok(number) => {
                if number >= 1 && number <= 100 {
                    infobar_state = InfoBarState::ValidNumber(number);
                } else {
                    infobar_state = InfoBarState::InvalidNumber(number);
                }
            }
            Err(parse_fail) => match parse_fail {
                myriad::parser::ParseFail::PartialSuccess => {
                    let text = new_chosen_positions.iter().map(|c| board[*c]).join("");
                    infobar_state = InfoBarState::Equation(text)
                }
                myriad::parser::ParseFail::Failure => {
                    return;
                }
            },
        }

        Dispatch::new().apply(InfoBarSetMessage(infobar_state));
    }
}

impl ChosenPositionsState {
    pub fn after_move_result(
        self,
        move_result: &MoveResult<GRID_COLUMNS, GRID_ROWS, GRID_SIZE>,
    ) -> Self {
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
    pub cheat: bool,
}

impl Reducer<ChosenPositionsState> for FindNumberMsg {
    fn apply(self, state: std::rc::Rc<ChosenPositionsState>) -> std::rc::Rc<ChosenPositionsState> {
        //log::debug!("Clicked");
        let fs = Dispatch::<FullGameState>::new().get();

        if let Some(path) = fs.found_words.words.get(&self.number) {
            if state.positions == path.path {
                ChosenPositionsState::default().into()
            } else {
                ChosenPositionsState {
                    positions: path.path.clone(),
                }
                .into()
            }
        } else {
            if self.cheat {
                //log::debug!("Cheating");
                if let Some(solution) = fs
                    .game
                    .solve_settings
                    .solve(fs.game.board.clone())
                    .find(|x| x.result == self.number)
                {
                    //log::debug!("Cheating Path found");
                    return ChosenPositionsState {
                        positions: solution.path,
                    }
                    .into();
                }
            }

            state
        }
    }
}

impl ChosenPositionsState {
    pub fn next(
        state: std::rc::Rc<Self>,
        allow_abandon: bool,
        coordinate: Tile<GRID_COLUMNS, GRID_ROWS>,
    ) -> std::rc::Rc<Self> {
        if let Some(last) = state.positions.last() {
            if last == &coordinate {
                //Abandon word - empty state

                if !allow_abandon {
                    return state;
                }

                //log::debug!("Abandon word");
                Dispatch::new().apply(ClearExpiredWordsMsg {});

                return ChosenPositionsState::default().into();
            }
        }

        let find_result = state.positions.iter().find_position(|&z| z == &coordinate);

        if let Some((index, _)) = find_result {
            let new_chosen_positions: ArrayVec<[Tile<GRID_COLUMNS, GRID_ROWS>; 9]> =
                state.positions.iter().take(index + 1).copied().collect();

            //TODO maybe send a find message here

            //log::debug!("Retrace word");

            return ChosenPositionsState {
                positions: new_chosen_positions,
            }
            .into(); //Retrace move
        }

        if state.positions.is_empty() || state.positions.last().unwrap().is_adjacent_to(&coordinate)
        {
            let mut new_chosen_positions = state.positions.clone();
            new_chosen_positions.push(coordinate);

            let board = Dispatch::<FullGameState>::new().get().game.board.clone();

            let mut letters = new_chosen_positions.iter().map(|c| board[*c]).peekable();

            let parse_result = parse_and_evaluate(&mut letters);

            if let Err(e) = parse_result {
                if matches!(e, myriad::parser::ParseFail::Failure) {
                    //illegal move
                    return state;
                }
            }

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

#[derive(PartialEq, Eq, Clone, Default, Serialize, Deserialize, Store)]
pub struct InputState {
    is_down: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMsg {
    Down {
        coordinate: Tile<GRID_COLUMNS, GRID_ROWS>,
    },
    Up {},
    Enter {
        coordinate: Tile<GRID_COLUMNS, GRID_ROWS>,
    },
    None,
}

impl Reducer<InputState> for InputMsg {
    fn apply(self, state: std::rc::Rc<InputState>) -> std::rc::Rc<InputState> {
        if self == InputMsg::None {
            return state;
        }
        //log::debug!("{self:?}");
        match self {
            InputMsg::Down { coordinate } => {
                //log::debug!("Input down {}", coordinate);
                Dispatch::new().apply(OnClickMsg {
                    coordinate,
                    allow_abandon: true,
                });

                InputState { is_down: true }.into()
            }
            InputMsg::Up {} => {
                //log::debug!("Input up");

                InputState { is_down: false }.into()
            }
            InputMsg::Enter { coordinate } => {
                if state.is_down {
                    //log::debug!("Input Enter {}", coordinate);
                    Dispatch::new().apply(OnClickMsg {
                        coordinate,
                        allow_abandon: false,
                    })
                }

                state
            }
            InputMsg::None => state,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OnClickMsg {
    pub coordinate: Tile<GRID_COLUMNS, GRID_ROWS>,
    pub allow_abandon: bool,
}

impl Reducer<ChosenPositionsState> for OnClickMsg {
    fn apply(self, state: std::rc::Rc<ChosenPositionsState>) -> std::rc::Rc<ChosenPositionsState> {
        //log::debug!("{self:?}");
        ChosenPositionsState::next(state, self.allow_abandon, self.coordinate)
    }
}
