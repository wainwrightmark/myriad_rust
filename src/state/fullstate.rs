use crate::core::prelude::*;
use crate::web::prelude::*;
use crate::state::foundwordsstate::*;
use crate::state::recentwordstate::*;
use crate::state::rotflipstate::*;
use crate::state::chosenpositionsstate::*;
use itertools::Itertools;
use log::debug;
use num::ToPrimitive;
use serde::*;
use std::cell::RefCell;
use std::ops::Deref;
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
    pub rotflip : RotFlipState,

    #[serde(skip)]
    pub recent_words: Rc<RecentWordState>,
}

impl FullState {

    //TODO also return cursor
    pub fn get_color(&self, coordinate: &Coordinate) -> (String, String) {
        // if self.chosen_positions.positions.is_empty() {
        //     return ("blue".to_string(), "pointer".to_string());
        // }

        let move_result = self.get_move_result(coordinate);

        match move_result {
            MoveResult::WordComplete {
                word: _,
                coordinates: _,
            } => ("darkgreen".to_string(), "pointer".to_string()),
            MoveResult::WordIncomplete {
                word: _,
                coordinates: _,
            } => ("green".to_string(), "pointer".to_string()),
            MoveResult::WordOutsideRange {
                word: _,
                coordinates: _,
            } => ("green".to_string(), "pointer".to_string()),
            MoveResult::WordAbandoned =>("darkgreen".to_string(), "pointer".to_string()),
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
            .chosen_positions.positions
            .iter()
            .find_position(|z| z == &coordinate);

        if let Some((index, _)) = find_result {
            let new_chosen_positions: Vec<Coordinate> = self
                .chosen_positions.positions
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
                .chosen_positions.positions
                .last()
                .unwrap()
                .is_adjacent(coordinate)
        {
            let mut new_chosen_positions = self.chosen_positions.positions.clone();
            new_chosen_positions.push(*coordinate);

            let word = self.board.get_word_text(&new_chosen_positions);

            let nodes_iter = new_chosen_positions.iter().map(|c| {
                let letter = &self.board.get_letter_at_coordinate(c);
                Node {
                    coordinate: *c,
                    letter: *letter,
                }
            });

            let nodes = nodes_iter.collect_vec();
            let check_result = self.solver.check(&nodes);

            let final_result = match check_result {
                crate::core::parser::ParseOutcome::Success(i) =>
                
                if self.solver.settings.allow(i){
                    MoveResult::WordComplete {
                    
                        word: FoundWord { result:i, path:  nodes.iter().map(|x| x.coordinate).collect_vec(), },
                        coordinates: new_chosen_positions,
                    }
                }
                else    {
                    MoveResult::WordOutsideRange  {
                        word: i.to_string(),
                        coordinates: new_chosen_positions,
                    }
                }
                ,
                crate::core::parser::ParseOutcome::PartialSuccess =>{
                    MoveResult::WordIncomplete  {
                        word,
                        coordinates: new_chosen_positions,
                    }
                },
                crate::core::parser::ParseOutcome::Failure => MoveResult::IllegalMove {},
            };

            return final_result;
        }

        MoveResult::IllegalMove {}
    }
}

pub enum Msg {
    NewGame,
    Move { coordinate: Coordinate },
}

fn get_emoji(i : i32)-> String{
    (match i {
        1 =>  "🌈⚡️💥✨💫🌸",
        2 => "🐒🐶🦊🐕🐈🐎",
        3 => "🐳🐬🐠🐙🦈",
        4 => "🦋🐛🐝🐞🕷️",
        5 => "🦖🐉🐲🦄👾👻👹👽",
        6 => "🌹🌷🍀🍃🌿🌸🌻💐",
        7 => "🐦🦤🦚🦜🐧🦅🐓🦆",
        8 => "🚀👩‍🚀☄️🌠☀️🌖🌌🛰️",
        9 => "😀🙂😃😺🐮",
        10 => "💯💯💯💯💯💯",
        _ =>  "🎈🎉🥳👯🪅🎊"
    }).to_string()
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

                let boards = create_boards(&solver, 9, &settings, &rng_cell);
                let board = boards[0].to_owned();
                let diff = instant::Instant::now() - start_instant;

                debug!("Board '{:?}' generated in {:?}", board, diff);
                

                FullState {
                    board: board.into(),
                    ..Default::default()
                }
                .into()
            }
            Msg::Move { coordinate } => {
                let move_result = state.get_move_result(coordinate);

                let new_chosen_positions = state.chosen_positions.to_owned().after_move_result(&move_result);

                let mut is_new_word: bool = false;

                let new_found_words: Rc<FoundWordsState> = if let MoveResult::WordComplete {
                    word: found_word,
                    coordinates: _,
                } = move_result.clone()
                {
                    is_new_word = !state.found_words.has_word(&found_word);
                    if is_new_word {                        
                        //let i =found_word.result;
                        let ns = state.found_words.with_word(found_word);

                        let len =  ns.words.len().to_i32().unwrap();
                                            
                        if len % 10 == 0{
                            make_confetti(get_emoji(len / 10), 10 + len* 2);
                        }

                        // if state.found_words.words.len() >= 100{
                        //     make_confetti(get_emoji(i) + "💯💯💯💯💯💯");
                        // }
                        
                        // else if ns.has_all_words(&mut num::iter::range( ((i / GOALSIZE) *GOALSIZE).max(1), ((i / GOALSIZE) + 1) * GOALSIZE)){
                        //     make_confetti(get_emoji(i));
                        // }
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
                    rotflip: state.rotflip.clone(),
                    chosen_positions: new_chosen_positions.into(),
                    recent_words: new_recent_words.into(),
                    found_words: new_found_words,
                }
                .into()
            }
        }
    }
}
