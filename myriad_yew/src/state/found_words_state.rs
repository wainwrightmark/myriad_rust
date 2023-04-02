use crate::state::prelude::*;
use myriad::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, ops::Range};

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
pub struct FoundWordsTracker {
    pub words: BTreeMap<i32, FoundWord<GRID_COLUMNS, GRID_ROWS, GRID_SIZE>>,
    pub most_recent: Option<i32>,
}

impl FoundWordsTracker {
    pub fn with_word(&self, word: FoundWord<GRID_COLUMNS, GRID_ROWS, GRID_SIZE>) -> Self {
        let mut new_map = self.words.clone();

        let i = word.result;
        new_map.insert(i, word);

        FoundWordsTracker {
            words: new_map,
            most_recent: Some(i),
        }
    }

    pub fn has_word(&self, word: &FoundWord<GRID_COLUMNS, GRID_ROWS, GRID_SIZE>) -> bool {
        self.words.contains_key(&word.result)
    }

    pub fn is_goal_complete(&self, index: usize) -> bool {
        let u = index as i32;

        self.has_all_words(((u * GOALSIZE) + 1)..(((u + 1) * GOALSIZE) + 1))
    }

    pub fn has_all_words(&self, mut range: Range<i32>) -> bool {
        if self.words.len() < range.size_hint().0 {
            return false;
        }

        range.all(|x| self.words.contains_key(&x))
    }
}
