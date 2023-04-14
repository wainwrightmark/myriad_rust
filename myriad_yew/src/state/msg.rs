use crate::state::prelude::*;
use crate::web::prelude::*;
use myriad::{parser, prelude::*};
use std::rc::Rc;
use yew_router::prelude::*;
use yewdux::prelude::*;

pub struct LoadGameMessage {
    pub game: Game,
}

impl Reducer<FullGameState> for LoadGameMessage {
    fn apply(self, previous: Rc<FullGameState>) -> Rc<FullGameState> {
        log::debug!(
            "Loading game. New Game '{}'. Old Game '{}'",
            self.game.board.to_single_string(),
            previous.game.board.to_single_string()
        );
        if previous.game.board == self.game.board {
            return previous;
        }

        let found_words = Dispatch::<HistoryState>::new()
            .get()
            .games
            .iter()
            .filter(|x| x.0 == self.game)
            .map(|x| x.1.clone())
            .next()
            .unwrap_or_default();

        Dispatch::<RecentWordState>::new().reduce_mut(|s| s.recent_words.clear());
        Dispatch::<ChosenPositionsState>::new().reduce_mut(|s| s.positions.clear());

        Dispatch::<DialogState>::new().reduce_mut(|x| x.history_dialog_type = None);

        Dispatch::<HistoryState>::new().apply(SaveGameMessage {
            game: previous.game.as_ref().clone(),
            found_words: previous.found_words.words.clone(),
        });

        FullGameState {
            game: self.game.into(),
            found_words: Rc::new(FoundWordsTracker {
                words: found_words,
                most_recent: None,
            }),
        }
        .into()
    }
}

pub fn move_to_new_game(for_today: bool, navigator: &Navigator) {
    let previous: Rc<FullGameState> = Dispatch::new().get();
    if for_today && previous.game.date == Some(Game::get_today_date()) {
        return; //Do nothing
    }

    Dispatch::<RecentWordState>::new().reduce_mut(|s| s.recent_words.clear());
    Dispatch::<ChosenPositionsState>::new().reduce_mut(|s| s.positions.clear());

    Dispatch::<HistoryState>::new().apply(SaveGameMessage {
        game: previous.game.as_ref().clone(),
        found_words: previous.found_words.words.clone(),
    });

    let game = if for_today {
        Game::create_for_today()
    } else {
        Game::create_random()
    };
    let game = game.board.to_single_string();

    navigator.push(&Route::Game { game })
}

pub struct OnCoordinatesSetMsg {
    pub coordinates: ArrayVec<[Tile<GRID_COLUMNS, GRID_ROWS>; 9]>,
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
    fn apply(self, state: Rc<FullGameState>) -> Rc<FullGameState> {
        let coordinates = self.coordinates;
        if coordinates.is_empty() {
            return state;
        }

        let mut letters = coordinates.iter().map(|c| state.game.board[*c]).peekable();
        let parse_result = parser::parse_and_evaluate(&mut letters);

        if let Ok(num) = parse_result {
            let found_word = FoundWord {
                result: num,
                path: coordinates,
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

            let new_found_words: Rc<FoundWordsTracker> = if word_type == FoundWordType::Found {
                let number = found_word.result;
                Dispatch::new().apply(NumberFoundMsg { number });
                let ns = state.found_words.with_word(found_word);

                let len = ns.words.len() as i32;

                if len % 10 == 0 {
                    make_confetti(get_emoji(len / 10), 10 + len);
                }

                if len == 100 {
                    Dispatch::<DialogState>::new().reduce_mut(|s| {
                        s.congratulations_dialog_type = Some(CongratsDialogType::OneHundred)
                    });
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
