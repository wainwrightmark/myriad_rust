use std::rc::Rc;

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
                if (1..=100).contains(&number) {
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
                positions: *coordinates,
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
                    positions: path.path,
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
                    Dispatch::<FullGameState>::new().apply(|mut x: Rc<FullGameState>| {
                        let gs = Rc::make_mut(&mut x);
                        gs.timing = GameTiming::Cheat;
                        x
                    });

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
        message: ChangeChosenPositionsMessage,
    ) -> std::rc::Rc<Self> {
        let coordinate = match message {
            ChangeChosenPositionsMessage::Continue(coordinate) => coordinate,
            ChangeChosenPositionsMessage::Abandon => {
                Dispatch::new().apply(ClearExpiredWordsMsg {});

                return ChosenPositionsState::default().into();
            }
        };
        if state.positions.last() == Some(&coordinate) {
            return state; //no change
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
            let mut new_chosen_positions = state.positions;
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
                coordinates: new_chosen_positions,
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
pub enum InputState {
    #[default]
    Up,
    DownFromLast(Tile<GRID_COLUMNS, GRID_ROWS>),
    DownFromNew(Tile<GRID_COLUMNS, GRID_ROWS>),
    DownMoved,
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
                let dispatch: Dispatch<ChosenPositionsState> = Dispatch::new();

                let from_last = dispatch.get().positions.last() == Some(&coordinate);

                dispatch.apply(ChangeChosenPositionsMessage::Continue(coordinate));

                if from_last {
                    InputState::DownFromLast(coordinate).into()
                } else {
                    InputState::DownFromNew(coordinate).into()
                }
            }
            InputMsg::Up {} => {
                //log::debug!("Input up");
                match state.as_ref() {
                    InputState::DownFromLast(_) => {
                        Dispatch::new().apply(ChangeChosenPositionsMessage::Abandon);
                    }
                    _ => {}
                }

                InputState::Up.into()
            }
            InputMsg::Enter { coordinate } => match state.as_ref() {
                InputState::Up => state,
                InputState::DownFromLast(c) | InputState::DownFromNew(c) => {
                    if c == &coordinate {
                        state
                    } else {
                        Dispatch::new().apply(ChangeChosenPositionsMessage::Continue(coordinate));
                        InputState::DownMoved.into()
                    }
                }
                InputState::DownMoved => {
                    Dispatch::new().apply(ChangeChosenPositionsMessage::Continue(coordinate));
                    state
                }
            },
            InputMsg::None => state,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeChosenPositionsMessage {
    Continue(Tile<GRID_COLUMNS, GRID_ROWS>),
    Abandon,
}

impl Reducer<ChosenPositionsState> for ChangeChosenPositionsMessage {
    fn apply(self, state: std::rc::Rc<ChosenPositionsState>) -> std::rc::Rc<ChosenPositionsState> {
        //log::debug!("{self:?}");
        ChosenPositionsState::next(state, self)
    }
}
