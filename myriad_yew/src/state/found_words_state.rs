use crate::state::prelude::*;
use myriad::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::*;

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
pub struct FoundWordsTracker {
    pub words: BTreeMap<i32, FoundWord<GRID_COLUMNS, GRID_ROWS, GRID_SIZE>>,
}

impl FoundWordsTracker {
    pub fn with_word(&self, word: FoundWord<GRID_COLUMNS, GRID_ROWS, GRID_SIZE>) -> Self {
        let mut new_map = self.words.clone();

        let i = word.result;
        new_map.insert(i, word);

        FoundWordsTracker {
            words: new_map,
        }
    }

    pub fn has_word(&self, word: &FoundWord<GRID_COLUMNS, GRID_ROWS, GRID_SIZE>) -> bool {
        self.words.contains_key(&word.result)
    }
}
