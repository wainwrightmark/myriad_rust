use crate::core::{parser, prelude::*};
use crate::state::prelude::*;
use crate::web::prelude::*;

use num::ToPrimitive;
use std::rc::Rc;
use yewdux::prelude::*;

pub struct NewGameMsg {pub today: bool }

impl Reducer<FullGameState> for NewGameMsg {
    fn apply(&self, _: Rc<FullGameState>) -> Rc<FullGameState> {

        Dispatch::<RecentWordState>::new().reduce_mut(|s| s.recent_words.clear());
        Dispatch::<RotFlipState>::new().reduce_mut(|s| s.new_game());
        Dispatch::<ChosenPositionsState>::new().reduce_mut(|s| s.positions.clear());

        let game = 
        if self.today{
            Game::create_for_today()
        }else{
            Game::create_random()
        };   

        FullGameState {
            game: game.into(),
            ..Default::default()
        }
        .into()
    }
}

pub struct OnCoordinatesSetMsg {
    pub coordinates: Vec<Coordinate>,
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

impl Reducer<FullGameState> for OnCoordinatesSetMsg {
    fn apply(&self, state: Rc<FullGameState>) -> Rc<FullGameState> {
        let coordinates = self.coordinates.clone();
        if coordinates.is_empty() {
            return state;
        }

        let mut letters = coordinates
            .iter()
            .map(|c| state.game.board.get_letter_at_coordinate(c))
            .peekable();
        let parse_result = parser::parse_and_evaluate(&mut letters);

        if let Ok(num) = parse_result {
            let found_word = FoundWord {
                result: num,
                path: coordinates.clone(),
            };
            let word_type = if state.game.solve_settings.allow(num) {
                if state.found_words.has_word(&found_word) {
                    FoundWordType::PreviouslyFound
                } else {
                    FoundWordType::Found
                }
            } else {
                FoundWordType::NotInRange
            };

            let new_found_words: Rc<FoundWordsState> = if word_type == FoundWordType::Found {
                Dispatch::new().apply(NumberFoundMsg {
                    number: found_word.result,
                });
                let ns = state.found_words.with_word(found_word);

                let len = ns.words.len().to_i32().unwrap();

                if len % 10 == 0 {
                    make_confetti(get_emoji(len / 10), 10 + len);
                }
                ns.into()
            } else {
                state.found_words.clone()
            };

            Dispatch::new().apply(WordFoundMsg {
                word: num,
                coordinate: *coordinates.last().unwrap(),
                word_type,
            });

            FullGameState {
                game: state.game.clone(),
                found_words: new_found_words,
            }
            .into()
        } else {
            state
        }
    }
}
