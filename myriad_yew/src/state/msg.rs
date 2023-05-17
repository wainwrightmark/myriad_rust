use crate::state::prelude::*;
use crate::web::prelude::*;
use myriad::{parser, prelude::*};
use std::rc::Rc;
use yew_router::prelude::*;
use yewdux::prelude::*;

use super::logging;

pub struct LoadGameMessage {
    pub game: Game,
}

impl Reducer<FullGameState> for LoadGameMessage {
    fn apply(self, previous: Rc<FullGameState>) -> Rc<FullGameState> {
        log::debug!(
            "Loading game. New Game '{}'. Old Game '{}'",
            self.game.board.canonical_string(),
            previous.game.board.canonical_string()
        );
        if previous.game.board == self.game.board {
            return previous;
        }

        let history = Dispatch::<HistoryState>::new().get();

        let loaded = history.games.iter().filter(|x| x.game == self.game).next();

        Dispatch::<RecentWordState>::new().reduce_mut(|s| s.recent_words.clear());
        Dispatch::<ChosenPositionsState>::new().reduce_mut(|s| s.positions.clear());
        Dispatch::<RotFlipState>::new().reduce_mut(|x| x.clear());

        Dispatch::<HistoryState>::new().apply(SaveGameMessage(previous.into()));

        match loaded {
            Some(state) => Rc::new(state.clone()),
            None => Rc::new(FullGameState {
                game: self.game,
                timing: Default::default(),
                found_words: Default::default(),
            }),
        }
    }
}

pub fn move_to_new_game(for_today: bool, navigator: &Navigator) {
    let previous: Rc<FullGameState> = Dispatch::new().get();
    if for_today && previous.game.date == Some(Game::get_today_date()) {
        return; //Do nothing
    }

    Dispatch::<RecentWordState>::new().reduce_mut(|s| s.recent_words.clear());
    Dispatch::<ChosenPositionsState>::new().reduce_mut(|s| s.positions.clear());

    Dispatch::<HistoryState>::new().apply(SaveGameMessage(previous));

    let game = if for_today {
        Game::create_for_today()
    } else {
        Game::create_random()
    };
    let game_string = game.board.canonical_string();

    let event = logging::LoggableEvent::NewGame {
        today: for_today,
        board: game_string.clone(),
    };
    event.try_log1();

    Dispatch::<FullGameState>::new().apply(LoadGameMessage {
        game,
    });

    navigator.push(&Route::Game { game: game_string })
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

            let timing: GameTiming;
            let new_found_words: Rc<FoundWordsTracker>;

            match word_type {
                FoundWordType::Found => {
                    let number = found_word.result;
                    Dispatch::new().apply(NumberFoundMsg { number });
                    let ns = state.found_words.with_word(found_word);

                    let len = ns.words.len();

                    if len % 10 == 0 {
                        make_confetti(get_emoji(len as i32 / 10), (10 + len) as i32);
                    }

                    if len == state.game.total_solutions {
                        let event = LoggableEvent::GameComplete {
                            board: state.game.board.canonical_string(),
                        };
                        event.try_log1();

                        Dispatch::<DialogState>::new().reduce_mut(|s| {
                            s.congratulations_dialog_type = Some(CongratsDialogType::OneHundred)
                        });

                        timing = match state.timing {
                            GameTiming::Started {
                                utc_time_milliseconds,
                            } => {
                                let js_today = js_sys::Date::new_0();
                                let utc_time = js_today.get_time();
                                let now_time_milliseconds = utc_time.floor() as i64;

                                let difference =
                                    now_time_milliseconds.saturating_sub(utc_time_milliseconds);

                                if difference.is_positive() {
                                    let total_milliseconds = difference.unsigned_abs();
                                    GameTiming::Finished { total_milliseconds }
                                } else {
                                    GameTiming::Unknown
                                }
                            }
                            GameTiming::Finished { total_milliseconds } => {
                                GameTiming::Finished { total_milliseconds }
                            }
                            GameTiming::Unknown => GameTiming::Unknown,
                            GameTiming::Cheat => GameTiming::Cheat,
                        }
                    } else {
                        timing = state.timing;
                    }

                    new_found_words = ns.into();
                }
                FoundWordType::PreviouslyFound => {
                    //update if the word is shorter
                    if let Some(_) = state
                        .found_words
                        .words
                        .get(&num)
                        .filter(|x| x.path.len() > coordinates.len())
                    {
                        new_found_words = state.found_words.with_word(found_word).into();
                    } else {
                        new_found_words = state.found_words.clone();
                    }

                    timing = state.timing;
                }
                FoundWordType::NotInRange => {
                    new_found_words = state.found_words.clone();
                    timing = state.timing;
                }
            }

            Dispatch::new().apply(WordFoundMsg {
                word: num,
                coordinate: *coordinates.last().unwrap(),
                word_type,
            });

            FullGameState {
                game: state.game.clone(),
                found_words: new_found_words,
                timing,
            }
            .into()
        } else {
            state
        }
    }
}
