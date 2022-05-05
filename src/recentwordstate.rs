use bounce::Atom;
use itertools::Itertools;
use log::debug;
use std::{ops::Deref, sync::Arc};

use crate::core::{coordinate::*, move_result::*};

#[derive(PartialEq, Atom, Clone, Default)]
pub struct RecentWordState {
    pub recent_words: Arc<Vec<RecentWord>>,
}

impl RecentWordState {
    fn with_word(&self, word: String, word_type: FoundWordType, coordinate: Coordinate) -> Self {
        let now = instant::Instant::now();
        let linger = word_type.linger_duration_ms();

        let r_word = RecentWord {
            word: word,
            word_type: word_type,
            coordinate: coordinate,
            expiry_time: now + instant::Duration::from_millis(linger),
        };

        debug!("{:?}", r_word);

        let new_words = self
            .recent_words
            .deref()
            .clone()
            .into_iter()
            .chain(std::iter::once(r_word))
            .collect_vec();

        Self {
            recent_words: Arc::new(new_words),
        }
    }

    pub fn clear_expired(&self) -> Self {

        if self.recent_words.is_empty(){return self.clone();};

        let now = instant::Instant::now();
        let new_words = self
            .recent_words
            .deref()
            .clone()
            .into_iter()
            .filter(|x| x.expiry_time > now)            
            .collect_vec();

        Self {
            recent_words: Arc::new(new_words),
        }
    }

    pub fn after_move_result(&self, move_result: &MoveResult, is_new_word: bool) -> Self {
        match move_result {
            MoveResult::WordComplete { word, coordinates } => self.with_word(
                word.result.to_string(),
                if is_new_word {
                    FoundWordType::Found
                } else {
                    FoundWordType::PreviouslyFound
                },
                coordinates.last().unwrap().clone(),
            ),
            MoveResult::WordContinued { word, coordinates } => self.with_word(
                word.clone(),
                FoundWordType::Illegal,
                coordinates.last().unwrap().clone(),
            ),
            MoveResult::WordAbandoned => self.clear_expired(),
            MoveResult::MoveRetraced {
                word: _,
                coordinates: _,
            } => self.clone(),
            MoveResult::IllegalMove => self.clone(),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum FoundWordType {
    Found,
    PreviouslyFound,
    Invalid,
    Illegal,
}

impl FoundWordType {
    pub fn linger_duration_ms(&self) -> u64 {
        const BASIS: u64 = 1000;

        match self {
            FoundWordType::Found => BASIS * 10,
            FoundWordType::PreviouslyFound => BASIS * 5,
            FoundWordType::Invalid => BASIS * 2,
            FoundWordType::Illegal => BASIS * 4,
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
            FoundWordType::Illegal => "Orange".to_string(),
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
