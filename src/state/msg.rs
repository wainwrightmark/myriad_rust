use crate::core::prelude::*;
use crate::state::chosenpositionsstate::*;
use crate::state::foundwordsstate::*;
use crate::state::fullstate::*;
use crate::web::prelude::*;
use log::debug;

use num::ToPrimitive;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use yewdux::prelude::*;

pub enum Msg {
    NewGame,
    Move { coordinate: Coordinate },
    SelectTab { index: usize },
    Find { number: i32 },
    FlipAndRotateAbsolute { rotate: i8, flip: bool },
    FlipAndRotateRelative { rotate: i8, flip: bool },
}

fn get_emoji(i: i32) -> String {
    (match i {
        1 => "🌈⚡️💥✨💫🌸",
        2 => "🐒🐶🦊🐕🐈🐎",
        3 => "🐳🐬🐠🐙🦈",
        4 => "🦋🐛🐝🐞🕷️",
        5 => "🦖🐉🐲🦄👾👻👹👽",
        6 => "🌹🌷🍀🍃🌿🌸🌻💐",
        7 => "🐦🦤🦚🦜🐧🦅🐓🦆",
        8 => "🚀👩‍🚀☄️🌠☀️🌖🌌🛰️",
        9 => "😀🙂😃😺🐮",
        10 => "💯💯💯💯💯💯",
        _ => "🎈🎉🥳👯🪅🎊",
    })
    .to_string()
}

impl Reducer<FullState> for Msg {
    fn apply(&self, state: Rc<FullState>) -> Rc<FullState> {
        match self {
            Msg::FlipAndRotateRelative { rotate, flip } => FullState {
                board: state.board.clone(),
                solver: state.solver.clone(),
                rotflip: super::rotflipstate::RotFlipState {
                    rotate: state.rotflip.rotate + *rotate,
                    flip: state.rotflip.flip ^ *flip,
                    max_coordinate: state.rotflip.max_coordinate,
                },
                chosen_positions: state.chosen_positions.clone(),
                recent_words: state.recent_words.clone(),
                found_words: state.found_words.clone(),
                selected_tab_state: state.selected_tab_state,
            }
            .into(),

            Msg::FlipAndRotateAbsolute { rotate, flip } => FullState {
                board: state.board.clone(),
                solver: state.solver.clone(),
                rotflip: super::rotflipstate::RotFlipState {
                    rotate: *rotate,
                    flip: *flip,
                    max_coordinate: state.rotflip.max_coordinate,
                },
                chosen_positions: state.chosen_positions.clone(),
                recent_words: state.recent_words.clone(),
                found_words: state.found_words.clone(),
                selected_tab_state: state.selected_tab_state,
            }
            .into(),

            Msg::Find { number } => {
                if let Some(word) = state.found_words.words.get(number) {
                    FullState {
                        board: state.board.clone(),
                        solver: state.solver.clone(),
                        rotflip: state.rotflip,
                        chosen_positions: ChosenPositionsState {
                            positions: word.path.clone(),
                        },
                        recent_words: state.recent_words.clone(),
                        found_words: state.found_words.clone(),
                        selected_tab_state: state.selected_tab_state,
                    }
                    .into()
                } else {
                    state
                }
            }

            Msg::SelectTab { index } => FullState {
                board: state.board.clone(),
                solver: state.solver.clone(),
                rotflip: state.rotflip,
                chosen_positions: state.chosen_positions.clone(),
                recent_words: state.recent_words.clone(),
                found_words: state.found_words.clone(),
                selected_tab_state: state.selected_tab_state.tab_clicked(*index),
            }
            .into(),

            Msg::NewGame => {
                let solver = Solver {
                    settings: SolveSettings { min: 1, max: 100 },
                };

                let settings = BoardCreateSettings {
                    branches_to_take: 2,
                    desired_solutions: 100,
                    number_to_return: 1,
                };
                let seed: u64 = rand::random();
                let start_instant = instant::Instant::now();
                debug!("Generating new board with seed {:?}", seed);
                let rng = rand::SeedableRng::seed_from_u64(seed);
                let rng_cell = RefCell::new(rng);

                let boards = create_boards(&solver, 9, &settings, &rng_cell);
                let board = boards[0].to_owned();
                let diff = instant::Instant::now() - start_instant;

                debug!("Board '{:?}' generated in {:?}", board, diff);

                FullState {
                    board: board.into(),
                    selected_tab_state: state.selected_tab_state,
                    ..Default::default()
                }
                .into()
            }

            Msg::Move { coordinate } => {
                let move_result = state.get_move_result(coordinate);

                let new_chosen_positions = state
                    .chosen_positions
                    .to_owned()
                    .after_move_result(&move_result);

                let mut is_new_word: bool = false;

                let mut new_selected_tab_state = state.selected_tab_state;
                let new_found_words: Rc<FoundWordsState> = if let MoveResult::WordComplete {
                    word: found_word,
                } = move_result.clone()
                {
                    is_new_word = !state.found_words.has_word(&found_word);
                    if is_new_word {
                        new_selected_tab_state =
                            new_selected_tab_state.number_found(found_word.result);
                        let ns = state.found_words.with_word(found_word);

                        let len = ns.words.len().to_i32().unwrap();

                        if len % 10 == 0 {
                            make_confetti(get_emoji(len / 10), 10 + len);
                        }
                        ns.into()
                    } else {
                        state.found_words.clone()
                    }
                } else {
                    state.found_words.clone()
                };

                let new_recent_words = state
                    .recent_words
                    .deref()
                    .clone()
                    .after_move_result(&move_result, is_new_word);

                FullState {
                    board: state.board.clone(),
                    solver: state.solver.clone(),
                    rotflip: state.rotflip,
                    chosen_positions: new_chosen_positions,
                    recent_words: new_recent_words.into(),
                    found_words: new_found_words,
                    selected_tab_state: new_selected_tab_state,
                }
                .into()
            }
        }
    }
}
