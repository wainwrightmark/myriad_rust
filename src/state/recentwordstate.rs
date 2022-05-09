use itertools::Itertools;
use crate::core::prelude::*;

#[derive(PartialEq, Clone, Default)]
pub struct RecentWordState {
    pub recent_words: Vec<RecentWord>,
}

impl RecentWordState {
    fn with_word(self, word: String, word_type: FoundWordType, coordinate: Coordinate) -> Self {
        let now = instant::Instant::now();
        let linger = word_type.linger_duration_ms();

        let r_word = RecentWord {
            word,
            word_type,
            coordinate,
            expiry_time: now + instant::Duration::from_millis(linger),
        };

        let new_words = self
            .recent_words
            .into_iter()
            .chain(std::iter::once(r_word))
            .collect_vec();

        Self {
            recent_words: new_words,
        }
    }

    pub fn clear_expired(self) -> Self {
        if self.recent_words.is_empty() {
            return self;
        };

        let now = instant::Instant::now();
        let new_words = self
            .recent_words
            .into_iter()
            .filter(|x| x.expiry_time > now)
            .collect_vec();

        Self {
            recent_words: new_words,
        }
    }

    pub fn after_move_result(self, move_result: &MoveResult, is_new_word: bool) -> Self {
        match move_result {
            MoveResult::WordComplete { word, coordinates } => self.with_word(
                word.result.to_string(),
                if is_new_word {
                    FoundWordType::Found
                } else {
                    FoundWordType::PreviouslyFound
                },
                *coordinates.last().unwrap(),
            ),
            MoveResult::WordOutsideRange { word, coordinates } => self.with_word(
                word.clone(),
                FoundWordType::NotInRange,
                *coordinates.last().unwrap(),
            ),
            MoveResult::WordAbandoned => self.clear_expired(),
            MoveResult::MoveRetraced {
                word: _,
                coordinates: _,
            } => self,
            MoveResult::IllegalMove => self,
            MoveResult::WordIncomplete{ word:_, coordinates:_ } => self
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum FoundWordType {
    Found,
    PreviouslyFound,
    Incomplete,
    NotInRange,
    Invalid,
}

impl FoundWordType {
    pub fn linger_duration_ms(&self) -> u64 {
        const BASIS: u64 = 1000;

        match self {
            FoundWordType::Found => BASIS * 10,
            FoundWordType::PreviouslyFound => BASIS * 5,
            FoundWordType::Invalid => BASIS * 2,
            FoundWordType::Incomplete => BASIS * 4,
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
            FoundWordType::Invalid => "Red".to_string(),
            FoundWordType::Incomplete => "Orange".to_string(),
            FoundWordType::NotInRange => "Orange".to_string(),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct RecentWord {
    pub word: String,
    pub word_type: FoundWordType,
    pub expiry_time: instant::Instant,
    pub coordinate: Coordinate,
}
