use crate::core::prelude::*;
use crate::state::prelude::*;
use itertools::Itertools;
use num::ToPrimitive;
use serde::*;
use std::rc::Rc;
use yewdux::prelude::*;

use chrono::{Datelike, NaiveDate};

#[derive(PartialEq, Eq, Store, Clone, Default, Serialize, Deserialize)]
#[store(storage = "local")] // can also be "session"
pub struct FullGameState {
    pub game: Rc<Game>,
    pub found_words: Rc<FoundWordsState>,
}

impl FullGameState {}

#[derive(PartialEq, Eq, Store, Clone, Serialize, Deserialize)]
pub struct Game {
    pub board: Board<GRID_COLUMNS, GRID_ROWS>,
    pub challenge_words: Vec<i32>,
    pub date: Option<NaiveDate>,
    pub solve_settings: SolveSettings,
}

pub const CHALLENGE_WORDS: usize = 3;

impl Game {
    pub fn get_today_date() -> NaiveDate {
        let js_today = js_sys::Date::new_0();
        
        NaiveDate::from_ymd(
            js_today.get_full_year().to_i32().unwrap(),
            js_today.get_month() + 1,
            js_today.get_date(),
        )
    }

    pub fn create_for_today() -> Self {
        let today = Self::get_today_date();
        log::debug!("Creating game for today {:?}", today);

        Game::create_for_date(today)
    }

    pub fn create_for_date(date: NaiveDate) -> Self {
        let solve_settings = SolveSettings::default();

        let seed = (date.year().abs() * 2000)
            + (date.month().to_i32().unwrap() * 100)
            + date.day().to_i32().unwrap();
        let rng = rand::SeedableRng::seed_from_u64(seed.to_u64().unwrap());

        let settings = BoardCreateSettings {
            branching_factor: 3,
        };
        let board = settings.create_boards(solve_settings, rng).next().unwrap();

        let challenge_words = Self::create_challenge_words(solve_settings, &board);

        Game {
            board,
            date: Some(date),
            solve_settings,
            challenge_words,
        }
    }

    pub fn create_random() -> Self {
        let solve_settings = SolveSettings::default();

        let settings = BoardCreateSettings {
            branching_factor: 3,
        };
        let seed: u64 = rand::random();
        let start_instant = instant::Instant::now();
        log::debug!("Generating new board with seed {:?}", seed);
        let rng = rand::SeedableRng::seed_from_u64(seed);

        let mut boards = settings.create_boards(solve_settings, rng);
        let board = boards.next().unwrap();
        let diff = instant::Instant::now() - start_instant;

        log::debug!("Board '{:?}' generated in {:?}", board, diff);

        let challenge_words = Self::create_challenge_words(solve_settings, &board);

        Game {
            board,
            date: None,
            solve_settings,
            challenge_words,
        }
    }

    fn create_challenge_words(
        solve_settings: SolveSettings,
        board: &Board<GRID_COLUMNS, GRID_ROWS>,
    ) -> Vec<i32> {
        solve_settings
            .solve(board.clone())
            .sorted_by(|a, b| b.path.len().cmp(&a.path.len()))
            .take(CHALLENGE_WORDS)
            .map(|f| f.result)
            .collect_vec()
    }
}

impl Default for Game {
    fn default() -> Self {
        Game::create_for_today()
    }
}
