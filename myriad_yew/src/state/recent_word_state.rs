use itertools::Itertools;
use myriad::prelude::Tile;
use yewdux::prelude::*;

use super::prelude::*;

pub struct WordFoundMsg {
    pub word: i32,
    pub word_type: FoundWordType,
    pub coordinate: Tile<GRID_COLUMNS, GRID_ROWS>,
}

impl Reducer<RecentWordState> for WordFoundMsg {
    fn apply(self, state: std::rc::Rc<RecentWordState>) -> std::rc::Rc<RecentWordState> {
        state
            .with_word(self.word, self.word_type, self.coordinate)
            .into()
    }
}

pub struct ClearExpiredWordsMsg {}

impl Reducer<RecentWordState> for ClearExpiredWordsMsg {
    fn apply(self, state: std::rc::Rc<RecentWordState>) -> std::rc::Rc<RecentWordState> {
        state.clear_expired().into()
    }
}

#[derive(PartialEq, Eq, Clone, Default, Store)]
pub struct RecentWordState {
    pub recent_words: Vec<RecentWord>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct RecentWord {
    pub number: i32,
    pub word_type: FoundWordType,
    pub expiry_time: instant::Instant,
    pub coordinate: Tile<GRID_COLUMNS, GRID_ROWS>,
}

impl RecentWordState {
    fn with_word(
        &self,
        word: i32,
        word_type: FoundWordType,
        coordinate: Tile<GRID_COLUMNS, GRID_ROWS>,
    ) -> Self {

        if word_type != FoundWordType::Found{
            return self.clone();
        }

        let now = instant::Instant::now();
        let linger = word_type.linger_duration_ms();

        let r_word = RecentWord {
            number: word,
            word_type,
            coordinate,
            expiry_time: now + instant::Duration::from_millis(linger),
        };

        let mut new_words = self.recent_words.clone();

        new_words.push(r_word);

        Self {
            recent_words: new_words,
        }
    }

    pub fn clear_expired(&self) -> Self {
        if self.recent_words.is_empty() {
            return Default::default();
        };

        let now = instant::Instant::now();
        let new_words = self
            .recent_words
            .iter()
            .filter(|&x| x.expiry_time > now)
            .cloned()
            .collect_vec();

        Self {
            recent_words: new_words,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum FoundWordType {
    Found,
    PreviouslyFound,
    NotInRange,
}

impl FoundWordType {
    pub fn linger_duration_ms(&self) -> u64 {
        const BASIS: u64 = 1000;

        match self {
            FoundWordType::Found => BASIS * 10,
            FoundWordType::PreviouslyFound => BASIS * 5,
            FoundWordType::NotInRange => BASIS * 4,
        }
    }
}

impl RecentWord {
    pub fn linger_duration_ms(&self) -> u64 {
        self.word_type.linger_duration_ms()
    }

    pub fn get_color(&self) -> String {
        match self.word_type {
            FoundWordType::Found => "Green".to_string(),
            FoundWordType::PreviouslyFound => "Blue".to_string(),
            FoundWordType::NotInRange => "Orange".to_string(),
        }
    }
}
