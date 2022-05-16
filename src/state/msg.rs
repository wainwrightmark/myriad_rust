use crate::core::prelude::*;
use crate::state::prelude::*;
use crate::web::prelude::*;
use log::debug;

use num::ToPrimitive;
use std::rc::Rc;
use yewdux::prelude::*;

pub struct NewGameMsg{

}


impl Reducer<FullState> for NewGameMsg {
    fn apply(&self, _: Rc<FullState>) -> Rc<FullState> {
        let solve_settings = SolveSettings { min: 1, max: 100 };

                let settings = BoardCreateSettings {
                    branching_factor: 3,
                };
                let seed: u64 = rand::random();
                let start_instant = instant::Instant::now();
                debug!("Generating new board with seed {:?}", seed);
                let rng = rand::SeedableRng::seed_from_u64(seed);

                let mut boards = settings.create_boards(9, solve_settings, rng);
                let board = boards.next().unwrap();
                let diff = instant::Instant::now() - start_instant;

                debug!("Board '{:?}' generated in {:?}", board, diff);

                Dispatch::<RecentWordState>::new().reduce_mut(|s|{s.recent_words.clear()});
                Dispatch::<RotFlipState>::new().reduce_mut(|s|s.new_game());

                FullState {
                    board: board.into(),
                    ..Default::default()
                }
                .into()
    }
}

pub enum Msg {
    Move { coordinate: Coordinate },    
    Find { number: i32 },
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

            Msg::Find { number } => {
                if let Some(word) = state.found_words.words.get(number) {
                    FullState {
                        board: state.board.clone(),
                        solve_settings: state.solve_settings,
                        chosen_positions: ChosenPositionsState {
                            positions: word.path.clone(),
                        },
                        found_words: state.found_words.clone(),
                    }
                    .into()
                } else {
                    state
                }
            }

            Msg::Move { coordinate } => {
                let move_result = state.get_move_result(coordinate);

                let new_chosen_positions = state
                    .chosen_positions
                    .to_owned()
                    .after_move_result(&move_result);

                let mut is_new_word: bool = false;

                
                let new_found_words: Rc<FoundWordsState> =
                    if let MoveResult::WordComplete { word: found_word } = move_result.clone() {
                        is_new_word = !state.found_words.has_word(&found_word);
                        if is_new_word {

                            Dispatch::new().apply(NumberFoundMsg{number: found_word.result});
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

                Dispatch::new().apply(WordFoundMsg{move_result: move_result, is_new_word});

                FullState {
                    board: state.board.clone(),
                    solve_settings: state.solve_settings,
                    chosen_positions: new_chosen_positions,
                    found_words: new_found_words,
                }
                .into()
            }
        }
    }
}
