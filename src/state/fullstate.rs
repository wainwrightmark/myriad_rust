use crate::core::prelude::*;
use crate::state::foundwordsstate::*;
use crate::state::gamestate::*;
use crate::state::recentwordstate::*;
use log::debug;
use serde::*;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use yewdux::prelude::*;

#[derive(PartialEq, Store, Clone, Default, Serialize, Deserialize)]
#[store(storage = "local")] // can also be "session"

pub struct FullState {
    pub game: Rc<Gamestate>,
    pub found_words: Rc<FoundWordsState>,
    #[serde(skip)]
    pub recent_words: Rc<RecentWordState>,
}

pub enum Msg {
    NewGame,
    Move { coordinate: Coordinate },
}

impl Reducer<FullState> for Msg {
    fn apply(&self, state: Rc<FullState>) -> Rc<FullState> {
        match self {
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

                let boards = crate::core::creator::create_boards(&solver, 9, &settings, &rng_cell);
                let board = boards[0].to_owned();
                let diff = instant::Instant::now() - start_instant;

                debug!("Board '{:?}' generated in {:?}", board, diff);
                let new_game_state = Gamestate {
                    board: board,
                    ..Default::default()
                };

                FullState {
                    game: new_game_state.into(),
                    recent_words: Default::default(),
                    found_words: Default::default(),
                }
                .into()
            }
            Msg::Move { coordinate } => {
                let move_result = state.game.get_move_result(coordinate);

                let new_game_state = state.game.deref().clone().after_move_result(&move_result);

                let mut is_new_word: bool = false;

                let new_found_words: Rc<FoundWordsState> = if let MoveResult::WordComplete {
                    word: found_word,
                    coordinates: _,
                } = move_result.clone()
                {
                    is_new_word = !state.found_words.has_word(&found_word);
                    if is_new_word {                        
                        let i =found_word.result;
                        let ns = state.found_words.with_word(found_word);

                        
                        const BLOCKSIZE : i32= 20;
                        if state.found_words.words.len() >= 100{
                            crate::web::confetti::make_confetti("💯💯💯💯💯💯🌈⚡️💥✨💫🌸".to_string());
                        }
                        
                        else if ns.has_all_words(&mut num::iter::range( ((i / BLOCKSIZE) *BLOCKSIZE).max(1), ((i / BLOCKSIZE) + 1) * BLOCKSIZE)){
                            crate::web::confetti::make_confetti("🌈⚡️💥✨💫🌸".to_string());
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
                    game: new_game_state.into(),
                    recent_words: new_recent_words.into(),
                    found_words: new_found_words,
                }
                .into()
            }
        }
    }
}
