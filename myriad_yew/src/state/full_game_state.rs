use crate::state::prelude::*;
use myriad::prelude::*;
use serde::*;
use serde_with::serde_as;
use std::rc::Rc;
use yewdux::prelude::*;

use chrono::{Datelike, NaiveDate};

#[derive(PartialEq, Eq, Store, Clone, Default, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)] // can also be "session"
pub struct FullGameState {
    pub game: Rc<Game>,
    pub found_words: Rc<FoundWordsTracker>,
}

impl FullGameState {}

#[serde_as]
#[derive(PartialEq, Eq, Store, Clone, Serialize, Deserialize)]
pub struct Game {
    #[serde_as(as = "_")]
    pub board: Board<3, 3, 9>,
    pub date: Option<NaiveDate>,
    pub solve_settings: SolveSettings,
    pub difficulties: Rc<Vec<Option<Difficulty>>>,
}

pub const CHALLENGE_WORDS: usize = 3;

impl Game {
    pub fn get_today_date() -> NaiveDate {
        let js_today = js_sys::Date::new_0();

        NaiveDate::from_ymd_opt(
            js_today.get_full_year() as i32,
            js_today.get_month() + 1,
            js_today.get_date(),
        )
        .expect("Invalid date")
    }

    pub fn create_for_today() -> Self {
        let today = Self::get_today_date();
        log::debug!("Creating game for today {:?}", today);

        Game::create_for_date(today)
    }

    pub fn create_for_date(date: NaiveDate) -> Self {
        let solve_settings = SolveSettings::default();

        let seed: u32 = (date.year().abs_diff(0) * 2000) + (date.month() * 100) + date.day();
        let rng = rand::SeedableRng::seed_from_u64(seed as u64);

        let settings = BoardCreateSettings {
            branching_factor: 3,
        };
        let board = settings
            .create_boards::<GRID_COLUMNS, 9, ClassicGameMode>(solve_settings, rng)
            .next()
            .unwrap();

        //let challenge_words = Self::create_challenge_words(solve_settings, &board);
        let difficulties = Self::get_difficulties(solve_settings, &board);

        Game {
            board,
            date: Some(date),
            solve_settings,
            difficulties: difficulties.into(),
        }
    }

    pub fn from_string(s: &str) -> Option<Self> {
        let board = Board::<3, 3, 9>::try_create(s)?;
        let solve_settings = SolveSettings::default();

        let difficulties = Self::get_difficulties(solve_settings, &board);

        let game = Self {
            board,
            date: None,
            solve_settings,
            difficulties: difficulties.into(),
        };

        Some(game)
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

        let mut boards =
            settings.create_boards::<GRID_COLUMNS, 9, ClassicGameMode>(solve_settings, rng);
        let board = boards.next().unwrap();
        let diff = instant::Instant::now() - start_instant;

        log::debug!("Board '{:?}' generated in {:?}", board, diff);

        let difficulties = Self::get_difficulties(solve_settings, &board);

        Game {
            board,
            date: None,
            solve_settings,
            difficulties: difficulties.into(),
        }
    }

    fn get_difficulties(
        solve_settings: SolveSettings,
        board: &Board<GRID_COLUMNS, GRID_ROWS, 9>,
    ) -> Vec<Option<Difficulty>> {
        let mut difficulties: Vec<Option<Difficulty>> = vec![None; 100];

        let solutions = solve_settings.solve(board.clone());

        for s in solutions {
            if s.result < 1 {
                continue;
            }
            let diff = s.get_difficulty();

            if let Some(current) = difficulties.get_mut((s.result - 1) as usize) {
                if let Some(c) = current {
                    if *c <= diff {
                        continue;
                    }
                }
                *current = Some(diff);
            }
        }

        difficulties
    }
}

impl Default for Game {
    fn default() -> Self {
        Game::create_for_today()
    }
}